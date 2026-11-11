//! Error types returned by the RustGram client, request builders, and helpers.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Convenience result alias used across `rustgram-api`.
pub type Result<T> = std::result::Result<T, Error>;

/// Additional parameters returned by Telegram on certain API errors
/// (for example rate limits or supergroup migrations).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ResponseParameters {
    /// New identifier of the supergroup after a group-to-supergroup migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    /// Number of seconds left before a rate-limited request can be retried.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

/// Error returned by the Telegram Bot API HTTP endpoint (`ok: false`).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("Telegram Bot API error {error_code}: {description}")]
pub struct ApiError {
    /// Numeric Telegram error code (typically an HTTP-like status such as 400, 403, 429).
    pub error_code: i32,
    /// Human-readable error description provided by Telegram.
    pub description: String,
    /// Optional structured recovery hints (`retry_after`, `migrate_to_chat_id`).
    pub parameters: Option<ResponseParameters>,
}

impl ApiError {
    /// Creates a new [`ApiError`].
    pub fn new(
        error_code: i32,
        description: impl Into<String>,
        parameters: Option<ResponseParameters>,
    ) -> Self {
        Self {
            error_code,
            description: description.into(),
            parameters,
        }
    }

    /// Returns the suggested retry delay when Telegram responds with `429 Too Many Requests`.
    pub fn retry_after(&self) -> Option<Duration> {
        self.parameters
            .as_ref()
            .and_then(|p| p.retry_after)
            .map(Duration::from_secs)
    }

    /// Returns the target supergroup ID if the chat migrated to a supergroup.
    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        self.parameters.as_ref().and_then(|p| p.migrate_to_chat_id)
    }

    /// Returns `true` if the request was rate limited (`error_code == 429`).
    pub fn is_rate_limited(&self) -> bool {
        self.error_code == 429
    }

    /// Returns `true` if the bot is blocked or lacks permissions (`error_code == 403`).
    pub fn is_forbidden(&self) -> bool {
        self.error_code == 403
    }

    /// Returns `true` if Telegram rejected an edit because content was unchanged.
    pub fn is_not_modified(&self) -> bool {
        self.error_code == 400 && self.description.contains("message is not modified")
    }
}

/// Top-level error type for all `rustgram-api` operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Error payload returned by Telegram Bot API (`ok: false`).
    #[error(transparent)]
    Api(#[from] ApiError),

    /// Transport-level HTTP error from `reqwest`.
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization or deserialization error.
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Local filesystem I/O error (for example reading an `InputFile::Path` upload).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Client-side validation failure (for example widget HTML > 32 KiB or NUL byte).
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// Unexpected HTTP response body or status that could not be decoded as `ApiResponse`.
    #[error("unexpected API response (status {status}): {body}")]
    UnexpectedResponse {
        /// HTTP status code received from the server.
        status: u16,
        /// Raw response body snippet.
        body: String,
    },

    /// Error while parsing or verifying Telegram Mini App `initData`.
    #[error("WebApp verification failed: {0}")]
    WebAppAuth(String),

    /// Error in webhook server or request signature verification.
    #[error("webhook error: {0}")]
    Webhook(String),

    /// Error in FSM state storage backend.
    #[error("state storage error: {0}")]
    State(String),
}
