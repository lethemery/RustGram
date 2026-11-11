//! Strongly-typed fluent API request builders for Telegram Bot API 10.4.

pub mod chats;
pub mod messages;
pub mod rich_and_ephemeral;
pub mod updates_and_queries;
pub mod widgets;

use std::future::Future;
use std::pin::Pin;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::{Bot, MultipartBuilder};
use crate::error::Result;
use crate::types::{InputFile, Message};

pub use chats::*;
pub use messages::*;
pub use rich_and_ephemeral::*;
pub use updates_and_queries::*;
pub use widgets::*;

/// Result returned by message editing methods (`Message` for chat messages, `true` for inline messages).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditedMessageResult {
    /// Edited message inside a chat.
    Message(Box<Message>),
    /// `true`, when editing an inline message.
    Inline(bool),
}

/// Boxed future returned by [`IntoFuture`] implementations on request builders.
pub type RequestFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

/// Executes a request that contains a primary [`InputFile`] and optional thumbnail [`InputFile`].
///
/// Automatically chooses `application/json` when files are `file_id` or `url`,
/// and `multipart/form-data` when uploading local files or in-memory bytes.
pub(crate) async fn execute_media_request<R, P>(
    bot: &Bot,
    method: &str,
    file_field: &'static str,
    file: &InputFile,
    thumb_field: Option<&'static str>,
    thumbnail: Option<&InputFile>,
    payload: &P,
) -> Result<R>
where
    R: DeserializeOwned,
    P: Serialize,
{
    let needs_multipart =
        file.requires_multipart() || thumbnail.is_some_and(InputFile::requires_multipart);

    let mut map = match serde_json::to_value(payload)? {
        Value::Object(map) => map,
        other => {
            return Err(crate::error::Error::InvalidInput(format!(
                "expected object payload for `{method}`, got {other}"
            )));
        }
    };

    if !needs_multipart {
        if let Some(s) = input_file_to_str(file) {
            map.insert(file_field.to_owned(), Value::String(s.to_owned()));
        }
        if let (Some(tf), Some(thumb)) = (thumb_field, thumbnail)
            && let Some(s) = input_file_to_str(thumb)
        {
            map.insert(tf.to_owned(), Value::String(s.to_owned()));
        }
        return bot.call_json(method, &map).await;
    }

    let mut form = MultipartBuilder::new();
    for (key, value) in map {
        let serialized = match value {
            Value::Null => continue,
            Value::String(s) => s,
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            other => serde_json::to_string(&other)?,
        };
        let leaked_or_owned = serialized;
        form = push_dynamic_text(form, key, leaked_or_owned);
    }

    form.push_file(file_field, file).await?;
    if let (Some(tf), Some(thumb)) = (thumb_field, thumbnail) {
        form.push_file(tf, thumb).await?;
    }

    bot.call_multipart(method, form.finish()).await
}

fn input_file_to_str(file: &InputFile) -> Option<&str> {
    match file {
        InputFile::FileId(s) | InputFile::Url(s) => Some(s.as_str()),
        InputFile::Memory { .. } | InputFile::Path(_) => None,
    }
}

fn push_dynamic_text(
    mut builder: MultipartBuilder,
    key: String,
    value: String,
) -> MultipartBuilder {
    // `MultipartBuilder` expects &'static str in its helper, so handle dynamic keys via push_text.
    builder.push_owned_text(key, value);
    builder
}

/// Shared macro generating `.send()` and `IntoFuture` on a JSON request builder struct
/// that stores `#[serde(skip)] bot: Bot` and `payload: PayloadStruct`.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_json_request {
    ($builder:ident, $method:literal, $output:ty) => {
        impl $builder {
            /// Executes this Bot API request asynchronously.
            pub async fn send(self) -> $crate::error::Result<$output> {
                self.bot.call_json($method, &self.payload).await
            }
        }

        impl ::std::future::IntoFuture for $builder {
            type Output = $crate::error::Result<$output>;
            type IntoFuture = $crate::requests::RequestFuture<'static, $output>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send())
            }
        }
    };
}

/// Helper macro defining builder setters on `$builder` delegating into `self.payload.$field`.
#[macro_export]
#[doc(hidden)]
macro_rules! request_setters {
    ($(
        $(#[$meta:meta])*
        $fn_name:ident ($arg:ident : $ty:ty) -> $field:ident $(; into)?
    ),* $(,)?) => {
        $(
            $(#[$meta])*
            pub fn $fn_name(mut self, $arg: $ty) -> Self {
                self.payload.$field = Some($arg.into());
                self
            }
        )*
    };
}
