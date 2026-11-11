//! Composable synchronous and asynchronous update filters (`on::command`, `on::widget_query`, `on::state`, etc.).

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::router::context::Context;
use crate::types::{UpdateKind, WidgetQueryType};

/// Boxed future returned by [`Filter::matches`].
pub type FilterFuture<'a> = Pin<Box<dyn Future<Output = bool> + Send + 'a>>;

/// Asynchronous predicate matching incoming updates inside [`Router`](crate::router::Router).
pub trait Filter: Send + Sync {
    /// Returns `true` if `ctx` satisfies this filter.
    fn matches<'a>(&'a self, ctx: &'a Context) -> FilterFuture<'a>;
}

/// Shared, cheaply cloneable boxed [`Filter`] with combinator methods.
#[derive(Clone)]
pub struct BoxedFilter(pub Arc<dyn Filter>);

impl BoxedFilter {
    /// Wraps any [`Filter`] implementation in a [`BoxedFilter`].
    pub fn new<F: Filter + 'static>(filter: F) -> Self {
        Self(Arc::new(filter))
    }

    /// Combines two filters with logical AND (short-circuiting).
    pub fn and(self, other: BoxedFilter) -> Self {
        Self::new(AndFilter(self.0, other.0))
    }

    /// Combines two filters with logical OR (short-circuiting).
    pub fn or(self, other: BoxedFilter) -> Self {
        Self::new(OrFilter(self.0, other.0))
    }

    /// Negates this filter (also available via the `!filter` operator).
    pub fn negated(self) -> Self {
        Self::new(NotFilter(self.0))
    }
}

impl std::ops::Not for BoxedFilter {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(NotFilter(self.0))
    }
}

impl Filter for BoxedFilter {
    fn matches<'a>(&'a self, ctx: &'a Context) -> FilterFuture<'a> {
        self.0.matches(ctx)
    }
}

struct AndFilter(Arc<dyn Filter>, Arc<dyn Filter>);

impl Filter for AndFilter {
    fn matches<'a>(&'a self, ctx: &'a Context) -> FilterFuture<'a> {
        Box::pin(async move { self.0.matches(ctx).await && self.1.matches(ctx).await })
    }
}

struct OrFilter(Arc<dyn Filter>, Arc<dyn Filter>);

impl Filter for OrFilter {
    fn matches<'a>(&'a self, ctx: &'a Context) -> FilterFuture<'a> {
        Box::pin(async move { self.0.matches(ctx).await || self.1.matches(ctx).await })
    }
}

struct NotFilter(Arc<dyn Filter>);

impl Filter for NotFilter {
    fn matches<'a>(&'a self, ctx: &'a Context) -> FilterFuture<'a> {
        Box::pin(async move { !self.0.matches(ctx).await })
    }
}

struct SyncFnFilter<F>(F);

impl<F> Filter for SyncFnFilter<F>
where
    F: Fn(&Context) -> bool + Send + Sync,
{
    fn matches<'a>(&'a self, ctx: &'a Context) -> FilterFuture<'a> {
        let matched = (self.0)(ctx);
        Box::pin(async move { matched })
    }
}

struct AsyncFnFilter<F>(F);

impl<F, Fut> Filter for AsyncFnFilter<F>
where
    F: Fn(&Context) -> Fut + Send + Sync,
    Fut: Future<Output = bool> + Send + 'static,
{
    fn matches<'a>(&'a self, ctx: &'a Context) -> FilterFuture<'a> {
        let fut = (self.0)(ctx);
        Box::pin(fut)
    }
}

/// Builds a synchronous [`BoxedFilter`] from a closure `|ctx: &Context| -> bool`.
pub fn filter_fn<F>(predicate: F) -> BoxedFilter
where
    F: Fn(&Context) -> bool + Send + Sync + 'static,
{
    BoxedFilter::new(SyncFnFilter(predicate))
}

/// Builds an asynchronous [`BoxedFilter`] from a closure `|ctx: &Context| async move { ... }`.
pub fn async_filter<F, Fut>(predicate: F) -> BoxedFilter
where
    F: Fn(&Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send + 'static,
{
    BoxedFilter::new(AsyncFnFilter(predicate))
}

/// Namespace of built-in Telegram Bot API 10.4 filters.
pub mod on {
    use super::*;

    /// Matches every update.
    pub fn any() -> BoxedFilter {
        filter_fn(|_| true)
    }

    /// Matches incoming regular messages (`Update.message`).
    pub fn message() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::Message(_)))
    }

    /// Matches edited messages (`Update.edited_message`).
    pub fn edited_message() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::EditedMessage(_)))
    }

    /// Matches channel posts (`Update.channel_post`).
    pub fn channel_post() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::ChannelPost(_)))
    }

    /// Matches messages that contain non-empty text.
    pub fn text() -> BoxedFilter {
        filter_fn(|ctx| ctx.message().and_then(|m| m.text.as_deref()).is_some())
    }

    /// Matches messages containing a specific bot command (without leading `/`, e.g. `"start"`).
    pub fn command(name: impl Into<String>) -> BoxedFilter {
        let target = name.into();
        filter_fn(move |ctx| {
            matches!(ctx.update.kind(), UpdateKind::Message(_))
                && ctx
                    .command()
                    .is_some_and(|(cmd, _)| cmd.eq_ignore_ascii_case(&target))
        })
    }

    /// Matches messages containing any `/command`.
    pub fn any_command() -> BoxedFilter {
        filter_fn(|ctx| {
            matches!(ctx.update.kind(), UpdateKind::Message(_)) && ctx.command().is_some()
        })
    }

    /// Matches messages containing a photo.
    pub fn photo() -> BoxedFilter {
        filter_fn(|ctx| ctx.message().and_then(|m| m.photo.as_ref()).is_some())
    }

    /// Matches messages containing a document.
    pub fn document() -> BoxedFilter {
        filter_fn(|ctx| ctx.message().and_then(|m| m.document.as_ref()).is_some())
    }

    /// Matches messages containing a structured Rich Message (Bot API 10.1+).
    pub fn rich_message() -> BoxedFilter {
        filter_fn(|ctx| ctx.message().is_some_and(|m| m.has_rich_message()))
    }

    /// Matches callback queries from inline buttons.
    pub fn callback_query() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::CallbackQuery(_)))
    }

    /// Matches callback queries whose `data` equals `exact`.
    pub fn callback_data(exact: impl Into<String>) -> BoxedFilter {
        let exact = exact.into();
        filter_fn(move |ctx| {
            ctx.callback_query().and_then(|cq| cq.data.as_deref()) == Some(exact.as_str())
        })
    }

    /// Matches callback queries whose `data` starts with `prefix`.
    pub fn callback_prefix(prefix: impl Into<String>) -> BoxedFilter {
        let prefix = prefix.into();
        filter_fn(move |ctx| {
            ctx.callback_query()
                .and_then(|cq| cq.data.as_deref())
                .is_some_and(|d| d.starts_with(&prefix))
        })
    }

    /// Matches inline queries (`@bot ...`).
    pub fn inline_query() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::InlineQuery(_)))
    }

    /// Matches pre-checkout queries.
    pub fn pre_checkout_query() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::PreCheckoutQuery(_)))
    }

    /// Matches shipping queries.
    pub fn shipping_query() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::ShippingQuery(_)))
    }

    /// Matches chat join requests.
    pub fn chat_join_request() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::ChatJoinRequest(_)))
    }

    /// Matches Bot API 10.4 [`WidgetQuery`](crate::types::WidgetQuery) updates.
    pub fn widget_query() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::WidgetQuery(_)))
    }

    /// Matches Bot API 10.4 [`SharedWidgetQuery`](crate::types::SharedWidgetQuery) updates.
    pub fn shared_widget_query() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::SharedWidgetQuery(_)))
    }

    /// Matches either `WidgetQuery` or `SharedWidgetQuery` (Bot API 10.4).
    pub fn any_widget_query() -> BoxedFilter {
        filter_fn(|ctx| {
            matches!(
                ctx.update.kind(),
                UpdateKind::WidgetQuery(_) | UpdateKind::SharedWidgetQuery(_)
            )
        })
    }

    /// Matches widget queries of type `WidgetQueryType::Data` (Bot API 10.4).
    pub fn widget_data() -> BoxedFilter {
        filter_fn(|ctx| match ctx.update.kind() {
            UpdateKind::WidgetQuery(wq) => matches!(wq.query_type, WidgetQueryType::Data { .. }),
            UpdateKind::SharedWidgetQuery(swq) => {
                matches!(swq.widget_type, WidgetQueryType::Data { .. })
            }
            _ => false,
        })
    }

    /// Matches widget queries carrying TON wallet signed data (`WidgetQueryType::SignData`, Bot API 10.4).
    pub fn widget_sign_data() -> BoxedFilter {
        filter_fn(|ctx| match ctx.update.kind() {
            UpdateKind::WidgetQuery(wq) => {
                matches!(wq.query_type, WidgetQueryType::SignData { .. })
            }
            UpdateKind::SharedWidgetQuery(swq) => {
                matches!(swq.widget_type, WidgetQueryType::SignData { .. })
            }
            _ => false,
        })
    }

    /// Matches widget queries carrying TON transaction notifications (`WidgetQueryType::SentTransaction`, Bot API 10.4).
    pub fn widget_sent_transaction() -> BoxedFilter {
        filter_fn(|ctx| match ctx.update.kind() {
            UpdateKind::WidgetQuery(wq) => {
                matches!(wq.query_type, WidgetQueryType::SentTransaction { .. })
            }
            UpdateKind::SharedWidgetQuery(swq) => {
                matches!(swq.widget_type, WidgetQueryType::SentTransaction { .. })
            }
            _ => false,
        })
    }

    /// Matches Guest Mode messages (`Update.guest_message`, Bot API 10.0).
    pub fn guest_message() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::GuestMessage(_)))
    }

    /// Matches user stop-generation updates (`Update.stopped_message_generation`, Bot API 10.2/10.3).
    pub fn stopped_generation() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::StoppedMessageGeneration(_)))
    }

    /// Matches managed bot lifecycle updates (`Update.managed_bot`, Bot API 9.6/10.4).
    pub fn managed_bot() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::ManagedBot(_)))
    }

    /// Matches user subscription updates (`Update.subscription`, Bot API 10.2).
    pub fn subscription() -> BoxedFilter {
        filter_fn(|ctx| matches!(ctx.update.kind(), UpdateKind::Subscription(_)))
    }

    /// Matches updates originating from a private chat.
    pub fn private_chat() -> BoxedFilter {
        filter_fn(|ctx| ctx.chat().is_some_and(|c| c.is_private()))
    }

    /// Matches updates originating from a group or supergroup chat.
    pub fn group_chat() -> BoxedFilter {
        filter_fn(|ctx| ctx.chat().is_some_and(|c| c.is_group()))
    }

    /// Matches when the user/chat FSM state equals `expected`.
    ///
    /// Requires registering an FSM backend via [`Dispatcher::manage_fsm`](crate::router::Dispatcher::manage_fsm).
    pub fn state<S>(expected: S) -> BoxedFilter
    where
        S: Clone + PartialEq + Send + Sync + 'static,
    {
        async_filter(move |ctx| {
            let expected = expected.clone();
            let fsm = ctx.fsm::<S>();
            async move {
                let Ok(fsm) = fsm else {
                    return false;
                };
                matches!(fsm.get().await, Ok(Some(current)) if current == expected)
            }
        })
    }
}
