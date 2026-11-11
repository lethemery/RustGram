//! Axum-based webhook server with secret-token verification (enabled via `webhook` feature).

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router as AxumRouter;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::error::{Error, Result};
use crate::types::Update;

/// Header name sent by Telegram containing the webhook secret token.
pub const SECRET_TOKEN_HEADER: &str = "x-telegram-bot-api-secret-token";

/// Configuration for the HTTP webhook receiver.
#[derive(Debug, Clone)]
pub struct WebhookConfig {
    /// Path on which Telegram POSTs updates (for example `"/webhook"`).
    pub path: String,
    /// Optional secret token configured via `setWebhook(secret_token = ...)`.
    pub secret_token: Option<String>,
    /// When `true`, spawns each update handler on a background Tokio task and returns `200 OK` immediately.
    pub concurrent_handling: bool,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            path: "/webhook".to_owned(),
            secret_token: None,
            concurrent_handling: true,
        }
    }
}

type UpdateCallback = Arc<dyn Fn(Update) -> futures::future::BoxFuture<'static, ()> + Send + Sync>;

#[derive(Clone)]
struct WebhookState {
    secret_token: Option<String>,
    concurrent: bool,
    handler: UpdateCallback,
}

/// Builds an [`axum::Router`] ready to accept Telegram Bot API webhook `POST` requests.
pub fn webhook_router<F, Fut>(config: WebhookConfig, handler: F) -> AxumRouter
where
    F: Fn(Update) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let callback: UpdateCallback = Arc::new(move |update| Box::pin(handler(update)));
    let state = WebhookState {
        secret_token: config.secret_token,
        concurrent: config.concurrent_handling,
        handler: callback,
    };

    AxumRouter::new()
        .route(&config.path, post(handle_webhook_post))
        .with_state(state)
}

/// Starts an HTTP server listening on `addr` and serving webhook updates until `cancel` triggers.
pub async fn serve_webhook<F, Fut>(
    addr: SocketAddr,
    config: WebhookConfig,
    cancel: CancellationToken,
    handler: F,
) -> Result<()>
where
    F: Fn(Update) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let app = webhook_router(config, handler);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| Error::Webhook(format!("failed to bind {addr}: {e}")))?;
    info!(%addr, "webhook server listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            cancel.cancelled().await;
            info!("webhook server shutting down");
        })
        .await
        .map_err(|e| Error::Webhook(format!("webhook server error: {e}")))
}

/// Verifies the optional Telegram secret token header and deserializes the webhook [`Update`] body.
pub fn parse_webhook_request(
    secret_token: Option<&str>,
    headers: &HeaderMap,
    body: &[u8],
) -> std::result::Result<Update, StatusCode> {
    if let Some(expected) = secret_token {
        let provided = headers
            .get(SECRET_TOKEN_HEADER)
            .and_then(|v| v.to_str().ok());
        match provided {
            Some(actual) if constant_time_eq(actual.as_bytes(), expected.as_bytes()) => {}
            _ => {
                warn!("rejected webhook request with invalid secret token");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }

    serde_json::from_slice(body).map_err(|err| {
        error!(error = %err, "failed to deserialize webhook Update payload");
        StatusCode::BAD_REQUEST
    })
}

async fn handle_webhook_post(
    State(state): State<WebhookState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let update = match parse_webhook_request(state.secret_token.as_deref(), &headers, &body) {
        Ok(u) => u,
        Err(status) => return status,
    };

    if state.concurrent {
        let handler = Arc::clone(&state.handler);
        tokio::spawn(async move {
            handler(update).await;
        });
    } else {
        (state.handler)(update).await;
    }

    StatusCode::OK
}

/// Constant-time byte comparison to prevent timing leaks on the webhook secret header.
pub(crate) fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
