//! Strongly-typed Telegram Bot API 10.4 data models.

pub mod business;
pub mod chat;
pub mod inline;
pub mod keyboard;
pub mod media;
pub mod message;
pub mod payment;
pub mod rich;
pub mod update;
pub mod widget;

use std::fmt;
use std::path::PathBuf;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

pub use business::*;
pub use chat::*;
pub use inline::*;
pub use keyboard::*;
pub use media::*;
pub use message::*;
pub use payment::*;
pub use rich::*;
pub use update::*;
pub use widget::*;

/// Unique identifier of a Telegram user or bot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub i64);

impl UserId {
    /// Returns the raw 64-bit Telegram identifier.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl From<i64> for UserId {
    #[inline]
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<UserId> for i64 {
    #[inline]
    fn from(value: UserId) -> Self {
        value.0
    }
}

impl PartialEq<i64> for UserId {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier of a message within a chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId(pub i64);

impl MessageId {
    /// Returns the raw message ID integer.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl From<i64> for MessageId {
    #[inline]
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<i32> for MessageId {
    #[inline]
    fn from(value: i32) -> Self {
        Self(i64::from(value))
    }
}

impl From<MessageId> for i64 {
    #[inline]
    fn from(value: MessageId) -> Self {
        value.0
    }
}

impl PartialEq<i64> for MessageId {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identifier of a forum topic / message thread in a supergroup or private chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ThreadId(pub i64);

impl From<i64> for ThreadId {
    #[inline]
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<ThreadId> for i64 {
    #[inline]
    fn from(value: ThreadId) -> Self {
        value.0
    }
}

impl fmt::Display for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Target chat identifier: either a numeric ID or a `@channelusername`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    /// Numeric chat, group, supergroup, channel, or user ID.
    Id(i64),
    /// Public `@username` of a channel or supergroup.
    Username(String),
}

impl ChatId {
    /// Creates a numeric [`ChatId`].
    #[inline]
    pub const fn id(id: i64) -> Self {
        Self::Id(id)
    }

    /// Creates a username-based [`ChatId`], automatically adding `@` if missing.
    pub fn username(username: impl Into<String>) -> Self {
        let s = username.into();
        if s.starts_with('@') {
            Self::Username(s)
        } else {
            Self::Username(format!("@{s}"))
        }
    }

    /// Returns the numeric chat ID if this is [`ChatId::Id`].
    #[inline]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Id(id) => Some(*id),
            Self::Username(_) => None,
        }
    }

    /// Returns `true` if this represents a private user chat (positive ID).
    #[inline]
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::Id(id) if *id > 0)
    }

    /// Returns `true` if this represents a group, supergroup, or channel (negative ID).
    #[inline]
    pub const fn is_group_or_channel(&self) -> bool {
        matches!(self, Self::Id(id) if *id < 0)
    }
}

impl From<i64> for ChatId {
    #[inline]
    fn from(value: i64) -> Self {
        Self::Id(value)
    }
}

impl From<UserId> for ChatId {
    #[inline]
    fn from(value: UserId) -> Self {
        Self::Id(value.0)
    }
}

impl From<String> for ChatId {
    #[inline]
    fn from(value: String) -> Self {
        Self::Username(value)
    }
}

impl From<&str> for ChatId {
    #[inline]
    fn from(value: &str) -> Self {
        Self::Username(value.to_owned())
    }
}

impl fmt::Display for ChatId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{id}"),
            Self::Username(u) => write!(f, "{u}"),
        }
    }
}

/// Formatting mode for text and captions in Telegram Bot API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParseMode {
    /// Legacy Telegram Markdown formatting.
    Markdown,
    /// Strict Telegram MarkdownV2 formatting.
    MarkdownV2,
    /// Telegram HTML subset formatting.
    #[serde(rename = "HTML")]
    Html,
}

/// Contents of a file to be uploaded or referenced via the Bot API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputFile {
    /// Existing Telegram `file_id` stored on Telegram servers.
    FileId(String),
    /// Public HTTP(S) URL for Telegram to download.
    Url(String),
    /// In-memory file payload uploaded via `multipart/form-data`.
    Memory {
        /// File name sent in the multipart `filename` header.
        filename: String,
        /// Raw bytes of the file.
        data: Bytes,
    },
    /// Local file path read asynchronously when building a multipart request.
    Path(PathBuf),
}

impl InputFile {
    /// Creates an [`InputFile`] from an existing Telegram `file_id`.
    pub fn file_id(id: impl Into<String>) -> Self {
        Self::FileId(id.into())
    }

    /// Creates an [`InputFile`] from an HTTP URL.
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// Creates an [`InputFile`] from an in-memory buffer.
    pub fn memory(filename: impl Into<String>, data: impl Into<Bytes>) -> Self {
        Self::Memory {
            filename: filename.into(),
            data: data.into(),
        }
    }

    /// Creates an [`InputFile`] from a local file path.
    pub fn path(path: impl Into<PathBuf>) -> Self {
        Self::Path(path.into())
    }

    /// Returns `true` if sending this file requires `multipart/form-data`.
    #[inline]
    pub const fn requires_multipart(&self) -> bool {
        matches!(self, Self::Memory { .. } | Self::Path(_))
    }
}

impl From<PathBuf> for InputFile {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

/// File metadata returned by [`getFile`](crate::Bot::get_file).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier for this file across time and bots.
    pub file_unique_id: String,
    /// File size in bytes, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    /// File path relative to `https://api.telegram.org/file/bot<token>/`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

/// Link preview generation options for messages containing URLs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LinkPreviewOptions {
    /// Pass `true` to disable link previews for links in the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    /// Explicit URL to generate a preview for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Pass `true` to shrink the media inside the preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    /// Pass `true` to enlarge the media inside the preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    /// Pass `true` to render the link preview above the message text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

impl LinkPreviewOptions {
    /// Convenience constructor to disable link previews.
    pub fn disabled() -> Self {
        Self {
            is_disabled: Some(true),
            ..Default::default()
        }
    }
}

/// List of profile pictures belonging to a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: u64,
    /// Requested profile pictures (in up to 4 sizes each).
    pub photos: Vec<Vec<PhotoSize>>,
}

/// Background color/chat accent info and arbitrary extra fields placeholder.
pub type JsonMap = serde_json::Map<String, serde_json::Value>;
