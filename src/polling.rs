//! Resilient asynchronous long-polling runner and `Update` stream.

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use futures::Stream;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::client::Bot;
use crate::error::Error;
use crate::types::{AllowedUpdate, Update};

/// Configuration for long polling (`getUpdates`).
#[derive(Debug, Clone)]
pub struct PollingConfig {
    /// Long-polling request timeout sent to Telegram (seconds, typically `10..=50`).
    pub timeout_secs: u32,
    /// Maximum number of updates fetched per request (`1..=100`).
    pub limit: u8,
    /// Optional explicit list of allowed update types.
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
    /// Delete any configured webhook before starting polling.
    pub delete_webhook: bool,
    /// Drop queued updates before entering the main loop.
    pub drop_pending_updates: bool,
    /// Initial retry backoff on transient errors.
    pub initial_backoff: Duration,
    /// Maximum retry backoff on repeated errors.
    pub max_backoff: Duration,
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            limit: 100,
            allowed_updates: None,
            delete_webhook: true,
            drop_pending_updates: false,
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(30),
        }
    }
}

/// Long-polling runner that yields Telegram [`Update`] objects with automatic offset tracking and backoff.
#[derive(Debug, Clone)]
pub struct Polling {
    bot: Bot,
    config: PollingConfig,
    cancel: CancellationToken,
}

impl Polling {
    /// Creates a new [`Polling`] runner with default configuration.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            config: PollingConfig::default(),
            cancel: CancellationToken::new(),
        }
    }

    /// Overrides the polling timeout in seconds.
    pub fn timeout_secs(mut self, secs: u32) -> Self {
        self.config.timeout_secs = secs;
        self
    }

    /// Sets the batch limit per `getUpdates` call (`1..=100`).
    pub fn limit(mut self, limit: u8) -> Self {
        self.config.limit = limit.clamp(1, 100);
        self
    }

    /// Restricts update types requested from Telegram.
    pub fn allowed_updates(mut self, updates: Vec<AllowedUpdate>) -> Self {
        self.config.allowed_updates = Some(updates);
        self
    }

    /// Drops pending updates on startup.
    pub fn drop_pending_updates(mut self, drop: bool) -> Self {
        self.config.drop_pending_updates = drop;
        self
    }

    /// Controls whether `deleteWebhook` is called on startup.
    pub fn delete_webhook(mut self, delete: bool) -> Self {
        self.config.delete_webhook = delete;
        self
    }

    /// Attaches a [`CancellationToken`] for graceful shutdown.
    pub fn cancellation_token(mut self, token: CancellationToken) -> Self {
        self.cancel = token;
        self
    }

    /// Returns a handle to the cancellation token.
    pub fn token(&self) -> CancellationToken {
        self.cancel.clone()
    }

    /// Converts this polling runner into an async `Stream` of [`Update`]s.
    pub fn into_stream(self) -> Pin<Box<dyn Stream<Item = Update> + Send>> {
        let (tx, rx) = tokio::sync::mpsc::channel::<Update>(256);
        tokio::spawn(async move {
            if self.cancel.is_cancelled() {
                info!("polling stopped via cancellation token");
                return;
            }
            let mut offset: Option<i64> = None;
            let mut backoff = self.config.initial_backoff;

            if let Err(err) = self.prepare(&mut offset).await {
                error!(error = %err, "failed to prepare polling");
            }

            loop {
                if self.cancel.is_cancelled() {
                    info!("polling stopped via cancellation token");
                    break;
                }

                let mut req = self
                    .bot
                    .get_updates()
                    .limit(self.config.limit)
                    .timeout(self.config.timeout_secs);
                if let Some(off) = offset {
                    req = req.offset(off);
                }
                if let Some(ref allowed) = self.config.allowed_updates {
                    req = req.allowed_updates(allowed.clone());
                }

                let fut = req.send();
                tokio::select! {
                    _ = self.cancel.cancelled() => {
                        info!("polling cancelled while waiting for updates");
                        break;
                    }
                    res = fut => {
                        match res {
                            Ok(updates) => {
                                backoff = self.config.initial_backoff;
                                for update in updates {
                                    let next = update.update_id + 1;
                                    offset = Some(offset.map_or(next, |cur| cur.max(next)));
                                    if tx.send(update).await.is_err() {
                                        debug!("polling receiver dropped, exiting");
                                        return;
                                    }
                                }
                            }
                            Err(err) => {
                                let wait = match &err {
                                    Error::Api(api) => api.retry_after().unwrap_or(backoff),
                                    _ => backoff,
                                };
                                warn!(error = %err, wait_secs = wait.as_secs_f32(), "getUpdates error, backing off");
                                tokio::select! {
                                    _ = self.cancel.cancelled() => break,
                                    _ = tokio::time::sleep(wait) => {}
                                }
                                backoff = (backoff * 2).min(self.config.max_backoff);
                            }
                        }
                    }
                }
            }
        });

        Box::pin(futures::stream::unfold(rx, |mut rx| async move {
            rx.recv().await.map(|u| (u, rx))
        }))
    }

    /// Runs `handler` sequentially for every incoming update until cancelled.
    pub async fn run<F, Fut>(self, mut handler: F)
    where
        F: FnMut(Update) -> Fut + Send,
        Fut: Future<Output = ()> + Send,
    {
        use futures::StreamExt;
        let mut stream = self.into_stream();
        while let Some(update) = stream.next().await {
            handler(update).await;
        }
    }

    async fn prepare(&self, offset: &mut Option<i64>) -> crate::error::Result<()> {
        if self.config.delete_webhook {
            self.bot
                .delete_webhook()
                .drop_pending_updates(self.config.drop_pending_updates)
                .await?;
        } else if self.config.drop_pending_updates {
            let drained = self
                .bot
                .get_updates()
                .offset(-1)
                .limit(1)
                .timeout(0)
                .await?;
            if let Some(last) = drained.last() {
                *offset = Some(last.update_id + 1);
            }
        }
        Ok(())
    }
}
