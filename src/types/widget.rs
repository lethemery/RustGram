//! Bot API 10.4 interactive HTML Widgets and TON wallet query types.

use serde::{Deserialize, Serialize};

use super::{
    Chat, InlineKeyboardMarkup, JsonMap, Message, MessageEntity, MessageId, ParseMode, User,
};
use crate::error::{Error, Result};

/// Maximum uncompressed UTF-8 HTML size allowed by Telegram for a widget (32 KiB).
pub const WIDGET_HTML_MAX_BYTES: usize = 32_768;

/// Maximum UTF-8 byte length for `public_data`, `chat_data`, and `answerWidgetQuery.data`.
pub const WIDGET_DATA_MAX_BYTES: usize = 1_024;

/// Minimum allowed `cache_time` (in seconds) for shared widgets.
pub const WIDGET_CACHE_TIME_MIN: i32 = 5;

/// Maximum allowed `cache_time` (in seconds) for shared widgets.
pub const WIDGET_CACHE_TIME_MAX: i32 = 3_600;

/// Default `cache_time` (in seconds) used by Telegram when omitted.
pub const WIDGET_CACHE_TIME_DEFAULT: i32 = 30;

/// Validates widget HTML source against Bot API 10.4 invariants (`<= 32_768` bytes, no NUL `U+0000`).
pub fn validate_widget_html(html: &str) -> Result<()> {
    if html.contains('\0') {
        return Err(Error::InvalidInput(
            "widget html must not contain NUL (U+0000) characters".into(),
        ));
    }
    if html.len() > WIDGET_HTML_MAX_BYTES {
        return Err(Error::InvalidInput(format!(
            "widget html exceeds maximum size of {WIDGET_HTML_MAX_BYTES} bytes (got {} bytes)",
            html.len()
        )));
    }
    Ok(())
}

/// Validates widget state/response data against the 1,024-byte Bot API 10.4 limit.
pub fn validate_widget_data(data: &str, field: &str) -> Result<()> {
    if data.len() > WIDGET_DATA_MAX_BYTES {
        return Err(Error::InvalidInput(format!(
            "widget {field} exceeds maximum size of {WIDGET_DATA_MAX_BYTES} bytes (got {} bytes)",
            data.len()
        )));
    }
    Ok(())
}

/// Clamps `cache_time` into Telegram's supported `[5, 3600]` second range.
#[inline]
pub fn clamp_widget_cache_time(seconds: i32) -> i32 {
    seconds.clamp(WIDGET_CACHE_TIME_MIN, WIDGET_CACHE_TIME_MAX)
}

/// Represents an HTML widget embedded in a [`Message`] (Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Widget {
    /// Unique identifier of the shared widget, if `is_shared` was enabled when created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// User's TON wallet card details attached to widget queries (Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WidgetQueryWallet {
    /// User-friendly formatted TON wallet address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Raw hexadecimal/workchain representation of the TON wallet address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_address: Option<String>,
    /// Network identifier (e.g. mainnet or testnet).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Public key of the user's wallet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Wallet application metadata or identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet_app: Option<String>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Type of a query originating from an interactive HTML widget (Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WidgetQueryType {
    /// Standard widget data request (`WidgetQueryTypeData`), optionally with wallet card.
    Data {
        /// `true`, if the user's TON wallet card was requested and attached.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        with_wallet: Option<bool>,
        /// User's TON wallet details when `with_wallet` is set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        wallet: Option<WidgetQueryWallet>,
    },
    /// Data cryptographically signed by the user's TON wallet (`WidgetQueryTypeSignData`).
    SignData {
        /// TON wallet card of the user who signed the payload.
        wallet: WidgetQueryWallet,
        /// Signed JSON payload (`DataJSON`).
        sign_data: serde_json::Value,
    },
    /// Notification about a transaction sent from the user's TON wallet (`WidgetQueryTypeSentTransaction`).
    SentTransaction {
        /// TON wallet card of the user who dispatched the transaction.
        wallet: WidgetQueryWallet,
        /// Normalized message hash of the sent TON transaction.
        msg_hash: String,
    },
    /// Fallback variant for forward compatibility with future widget query types.
    #[serde(other)]
    Unknown,
}

impl WidgetQueryType {
    /// Returns the attached TON wallet card, if this query contains one.
    pub const fn wallet(&self) -> Option<&WidgetQueryWallet> {
        match self {
            Self::Data { wallet, .. } => wallet.as_ref(),
            Self::SignData { wallet, .. } | Self::SentTransaction { wallet, .. } => Some(wallet),
            Self::Unknown => None,
        }
    }
}

/// Incoming query sent to the bot from a widget embedded in a message (Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WidgetQuery {
    /// Unique identifier for this query, used with [`answerWidgetQuery`](crate::Bot::answer_widget_query).
    pub id: String,
    /// Sender of the query.
    pub from: User,
    /// Message containing the widget, if sent directly by the bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
    /// Identifier of the inline message containing the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier corresponding to the chat containing the widget.
    pub chat_instance: String,
    /// Query kind and optional TON wallet payload.
    #[serde(rename = "type", alias = "widget_type")]
    pub query_type: WidgetQueryType,
    /// Optional custom data string passed by `TelegramWidget.request()`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// Incoming query sent from a forwarded message containing a shared widget (Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedWidgetQuery {
    /// Unique identifier for this query.
    pub id: String,
    /// Sender of the query.
    pub from: User,
    /// Chat where the forwarded shared widget resides.
    pub chat: Chat,
    /// Identifier of the message with the shared widget.
    pub message_id: MessageId,
    /// Unique identifier of the shared widget, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
    /// Query kind and optional TON wallet payload.
    #[serde(rename = "widget_type", alias = "type")]
    pub widget_type: WidgetQueryType,
    /// Optional custom data string passed by the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// Content of a new HTML widget message to be sent as an inline query result (Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputWidgetMessageContent {
    /// Self-contained HTML document (inline CSS/JS, up to 32,768 bytes, no NUL bytes).
    pub html: String,
    /// Placeholder width in logical pixels before render (`0` for automatic).
    pub width: i32,
    /// Placeholder height in logical pixels before render (`0` for automatic).
    pub height: i32,
    /// Pass `true` to create a persistent shared widget with a reusable `widget.id`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    /// Public state string visible across forwards/instances (up to 1,024 bytes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,
    /// Chat-specific state string (up to 1,024 bytes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,
    /// Refresh/poll period for shared widgets in seconds (`5..=3600`, default `30`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    /// Optional message caption (0-1024 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the caption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Special entities in the caption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Show caption above the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional inline keyboard attached to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl InputWidgetMessageContent {
    /// Creates an [`InputWidgetMessageContent`] with automatic dimensions (`0x0`).
    pub fn new(html: impl Into<String>) -> Self {
        Self {
            html: html.into(),
            width: 0,
            height: 0,
            is_shared: None,
            public_data: None,
            chat_data: None,
            cache_time: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            reply_markup: None,
        }
    }

    /// Sets explicit placeholder dimensions in logical pixels.
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Marks this widget as a shared widget.
    pub fn shared(mut self, is_shared: bool) -> Self {
        self.is_shared = Some(is_shared);
        self
    }

    /// Sets initial `public_data`.
    pub fn public_data(mut self, data: impl Into<String>) -> Self {
        self.public_data = Some(data.into());
        self
    }

    /// Sets initial `chat_data`.
    pub fn chat_data(mut self, data: impl Into<String>) -> Self {
        self.chat_data = Some(data.into());
        self
    }

    /// Validates HTML and data payload limits prior to sending.
    pub fn validate(&self) -> Result<()> {
        validate_widget_html(&self.html)?;
        if let Some(ref data) = self.public_data {
            validate_widget_data(data, "public_data")?;
        }
        if let Some(ref data) = self.chat_data {
            validate_widget_data(data, "chat_data")?;
        }
        Ok(())
    }
}

/// Content referencing an existing shared widget for inline query results (Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputSharedWidgetMessageContent {
    /// Unique identifier of an existing shared widget created by the bot.
    pub widget_id: String,
    /// Optional chat-specific state string (up to 1,024 bytes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,
    /// Optional caption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for caption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Caption entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Show caption above the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional inline keyboard attached to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl InputSharedWidgetMessageContent {
    /// Creates an [`InputSharedWidgetMessageContent`] referencing `widget_id`.
    pub fn new(widget_id: impl Into<String>) -> Self {
        Self {
            widget_id: widget_id.into(),
            chat_data: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            reply_markup: None,
        }
    }
}
