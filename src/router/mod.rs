//! Asynchronous update router, composable middleware, and dispatcher.

pub mod context;
pub mod filters;
pub mod fsm;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[cfg(feature = "webhook")]
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};

use crate::client::Bot;
use crate::error::Result;
use crate::polling::Polling;
use crate::types::Update;

pub use context::{Context, Extensions};
pub use filters::{BoxedFilter, Filter, async_filter, filter_fn, on};
pub use fsm::{MemoryStorage, StateContext, StateKey, StateStorage};

/// Boxed future returned by route handlers and middleware.
pub type HandlerFuture = Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>>;

/// Async handler function type.
pub type HandlerFn = Arc<dyn Fn(Context) -> HandlerFuture + Send + Sync>;

/// Next handler continuation passed into middleware.
#[derive(Clone)]
pub struct Next {
    inner: HandlerFn,
}

impl Next {
    /// Invokes the remainder of the middleware / route chain.
    pub async fn run(self, ctx: Context) -> Result<()> {
        (self.inner)(ctx).await
    }
}

/// Async middleware function type.
pub type MiddlewareFn = Arc<dyn Fn(Context, Next) -> HandlerFuture + Send + Sync>;

/// Result of attempting to dispatch an update through a [`Router`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchOutcome {
    /// A matching route (or fallback) handled the update.
    Handled,
    /// No route matched and no fallback was configured.
    Unhandled,
}

enum RouteEntry {
    Handler {
        filter: BoxedFilter,
        handler: HandlerFn,
    },
    Nested {
        filter: BoxedFilter,
        router: Router,
    },
}

/// Composable tree of filtered update handlers and middleware layers.
#[derive(Clone, Default)]
pub struct Router {
    routes: Vec<Arc<RouteEntry>>,
    middlewares: Vec<MiddlewareFn>,
    fallback: Option<HandlerFn>,
}

impl Router {
    /// Creates an empty [`Router`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a handler guarded by `filter`.
    pub fn route<H, Fut>(mut self, filter: BoxedFilter, handler: H) -> Self
    where
        H: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let wrapped: HandlerFn = Arc::new(move |ctx| Box::pin(handler(ctx)));
        self.routes.push(Arc::new(RouteEntry::Handler {
            filter,
            handler: wrapped,
        }));
        self
    }

    /// Mounts a sub-router scoped under `filter`.
    pub fn nest(mut self, filter: BoxedFilter, router: Router) -> Self {
        self.routes
            .push(Arc::new(RouteEntry::Nested { filter, router }));
        self
    }

    /// Attaches a middleware layer executed around matching routes on this router.
    ///
    /// Outer layers added later wrap earlier layers.
    pub fn layer<M, Fut>(mut self, middleware: M) -> Self
    where
        M: Fn(Context, Next) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.middlewares
            .push(Arc::new(move |ctx, next| Box::pin(middleware(ctx, next))));
        self
    }

    /// Registers a fallback handler called when no route matches.
    pub fn fallback<H, Fut>(mut self, handler: H) -> Self
    where
        H: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.fallback = Some(Arc::new(move |ctx| Box::pin(handler(ctx))));
        self
    }

    /// Evaluates routes in registration order and executes the first matching handler.
    pub fn handle<'a>(
        &'a self,
        ctx: Context,
    ) -> Pin<Box<dyn Future<Output = Result<DispatchOutcome>> + Send + 'a>> {
        Box::pin(async move {
            let outcome = Arc::new(tokio::sync::Mutex::new(DispatchOutcome::Unhandled));
            let router = self.clone();
            let outcome_ref = Arc::clone(&outcome);

            let leaf: HandlerFn = Arc::new(move |ctx| {
                let router = router.clone();
                let outcome_ref = Arc::clone(&outcome_ref);
                Box::pin(async move {
                    if let Some(handler) = router.resolve(&ctx).await {
                        *outcome_ref.lock().await = DispatchOutcome::Handled;
                        handler(ctx).await?;
                    } else if let Some(ref fb) = router.fallback {
                        *outcome_ref.lock().await = DispatchOutcome::Handled;
                        fb(ctx).await?;
                    }
                    Ok(())
                })
            });

            let chain = self.middlewares.iter().fold(leaf, |next_fn, mw| {
                let mw = Arc::clone(mw);
                let next = Next { inner: next_fn };
                Arc::new(move |ctx| mw(ctx, next.clone()))
            });

            chain(ctx).await?;
            let final_outcome = *outcome.lock().await;
            Ok(final_outcome)
        })
    }

    fn resolve<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> Pin<Box<dyn Future<Output = Option<HandlerFn>> + Send + 'a>> {
        Box::pin(async move {
            for entry in &self.routes {
                match entry.as_ref() {
                    RouteEntry::Handler { filter, handler } => {
                        if filter.matches(ctx).await {
                            return Some(Arc::clone(handler));
                        }
                    }
                    RouteEntry::Nested { filter, router } => {
                        if filter.matches(ctx).await
                            && let Some(matched) = router.resolve(ctx).await
                        {
                            // Wrap nested middlewares around the matched inner handler.
                            let wrapped = router.middlewares.iter().fold(matched, |next_fn, mw| {
                                let mw = Arc::clone(mw);
                                let next = Next { inner: next_fn };
                                Arc::new(move |ctx| mw(ctx, next.clone()))
                            });
                            return Some(wrapped);
                        }
                    }
                }
            }
            None
        })
    }
}

type ErrorHandlerFn = Arc<
    dyn Fn(Context, crate::error::Error) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

/// Top-level runtime coordinating [`Bot`], shared state, [`Router`], and update ingestion.
#[derive(Clone)]
pub struct Dispatcher {
    bot: Bot,
    router: Router,
    extensions: Extensions,
    on_error: Option<ErrorHandlerFn>,
}

impl Dispatcher {
    /// Creates a new [`Dispatcher`] for `bot` and `router`.
    pub fn new(bot: Bot, router: Router) -> Self {
        Self {
            bot,
            router,
            extensions: Extensions::new(),
            on_error: None,
        }
    }

    /// Registers shared state of type `T`, accessible via `ctx.state::<T>()`.
    pub fn manage<T: Send + Sync + 'static>(mut self, value: T) -> Self {
        self.extensions.insert(value);
        self
    }

    /// Registers an FSM [`StateStorage`] implementation for state type `S`.
    pub fn manage_fsm<S>(mut self, storage: Arc<dyn StateStorage<S>>) -> Self
    where
        S: Clone + Send + Sync + 'static,
    {
        self.extensions
            .insert_arc::<Arc<dyn StateStorage<S>>>(Arc::new(storage));
        self
    }

    /// Registers a global error handler invoked whenever a route or middleware returns `Err`.
    pub fn on_error<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Context, crate::error::Error) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_error = Some(Arc::new(move |ctx, err| Box::pin(handler(ctx, err))));
        self
    }

    /// Dispatches a single [`Update`] through the router.
    pub async fn dispatch(&self, update: Update) -> DispatchOutcome {
        let extensions = Arc::new(self.extensions.clone());
        let ctx = Context::new(self.bot.clone(), update, extensions);
        match self.router.handle(ctx.clone()).await {
            Ok(outcome) => {
                debug!(
                    update_id = ctx.update.update_id,
                    ?outcome,
                    "dispatched update"
                );
                outcome
            }
            Err(err) => {
                if let Some(ref handler) = self.on_error {
                    handler(ctx, err).await;
                } else {
                    error!(update_id = ctx.update.update_id, error = %err, "handler error");
                }
                DispatchOutcome::Handled
            }
        }
    }

    /// Starts long polling using default [`Polling`] settings and handles Ctrl-C shutdown.
    pub async fn run_polling(self) {
        let polling = Polling::new(self.bot.clone());
        self.run_polling_with(polling).await;
    }

    /// Starts long polling with a customized [`Polling`] runner and graceful shutdown.
    pub async fn run_polling_with(self, polling: Polling) {
        let cancel = polling.token();
        let ctrl_c_token = cancel.clone();
        tokio::spawn(async move {
            if tokio::signal::ctrl_c().await.is_ok() {
                ctrl_c_token.cancel();
            }
        });

        let shared = Arc::new(self);
        polling
            .run(move |update| {
                let dp = Arc::clone(&shared);
                async move {
                    dp.dispatch(update).await;
                }
            })
            .await;
    }

    /// Starts an Axum webhook server and dispatches incoming updates (requires `webhook` feature).
    #[cfg(feature = "webhook")]
    pub async fn run_webhook(
        self,
        addr: std::net::SocketAddr,
        config: crate::webhook::WebhookConfig,
        cancel: CancellationToken,
    ) -> Result<()> {
        let shared = Arc::new(self);
        crate::webhook::serve_webhook(addr, config, cancel, move |update| {
            let dp = Arc::clone(&shared);
            async move {
                dp.dispatch(update).await;
            }
        })
        .await
    }
}
