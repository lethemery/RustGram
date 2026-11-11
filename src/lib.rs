//! **RustGram** — Ergonomic asynchronous library for creating Telegram bots in Rust,
//! built for **Telegram Bot API 10.4**.
//!
//! # Highlights
//! - **Full Bot API 10.4 coverage**: Interactive HTML Widgets (`sendWidget`, `sendSharedWidget`,
//!   `answerWidgetQuery`, `editMessageWidgetData`, `editSharedWidgetData`, TON wallet query types),
//!   Rich Messages & Rich Blocks (`sendRichMessage`, `sendRichMessageDraft`, tables, expandable quotes,
//!   buttons, `RichBlock::Thinking`), Ephemeral Messages (`ephemeral_message_parameters`,
//!   `editEphemeralMessage*`, `deleteEphemeralMessage`), Streaming Drafts with user-stop handling
//!   (`sendMessageDraft`, `MessageGenerationStopped`), Guest Mode (`answerGuestQuery`),
//!   and Managed Bots (`KeyboardButtonRequestManagedBot`, `getManagedBotToken`, `BotAccessSettings`).
//! - **Ergonomic `Bot` client & fluent `.await`able requests**: Every request builder implements
//!   `IntoFuture`, so `bot.send_message(chat_id, "hi").parse_mode(ParseMode::Html).await?` works directly.
//! - **Composable Router, Filters, Middleware & FSM**: `Router`, `Dispatcher`, async/sync filters
//!   under `on::*`, and typed conversation states via `MemoryStorage` / `StateStorage`.
//! - **Long Polling, Webhook Server (`webhook`) & Mini App `initData` Verification (`webapp`)**.

pub mod client;
pub mod error;
pub mod formatting;
pub mod polling;
pub mod requests;
pub mod router;
pub mod types;

#[cfg(feature = "webapp")]
pub mod webapp;
#[cfg(feature = "webhook")]
pub mod webhook;

/// Latest Telegram Bot API specification targeted by this crate.
pub const BOT_API_VERSION: &str = "10.4";

pub use client::{Bot, BotBuilder, DEFAULT_API_URL};
pub use error::{ApiError, Error, ResponseParameters, Result};

/// Convenient re-exports for everyday bot development (`use rustgram::prelude::*;`).
pub mod prelude {
    pub use crate::BOT_API_VERSION;
    pub use crate::client::{Bot, BotBuilder};
    pub use crate::error::{ApiError, Error, Result};
    pub use crate::formatting::{
        bold_html, code_html, custom_emoji_html, date_time_html, escape_html, escape_markdown_v2,
        italic_html, link_html, mention_html, pre_html,
    };
    pub use crate::polling::Polling;
    pub use crate::requests::ChatAction;
    pub use crate::router::{
        Context, DispatchOutcome, Dispatcher, MemoryStorage, Next, Router, StateContext,
        StateStorage, async_filter, filter_fn, on,
    };
    pub use crate::types::{
        AllowedUpdate, BotCommand, ChatId, EphemeralMessageParameters, InlineKeyboardButton,
        InlineKeyboardMarkup, InputFile, InputRichMessage, InputSharedWidgetMessageContent,
        InputWidgetMessageContent, KeyboardButton, KeyboardButtonRequestManagedBot, Message,
        MessageId, ParseMode, ReplyKeyboardMarkup, ReplyMarkup, ReplyParameters, RichBlock,
        RichMessage, SharedWidgetQuery, Update, UpdateKind, User, UserId, Widget, WidgetQuery,
        WidgetQueryType, WidgetQueryWallet,
    };
    #[cfg(feature = "webapp")]
    pub use crate::webapp::{VerifiedInitData, WebAppUser, verify_init_data};
    #[cfg(feature = "webhook")]
    pub use crate::webhook::{WebhookConfig, serve_webhook, webhook_router};
}
