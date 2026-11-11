//! Bot API 10.4 HTML Widget request builders (`sendWidget`, `sendSharedWidget`, `answerWidgetQuery`, `editMessageWidgetData`, `editSharedWidgetData`).

use std::future::IntoFuture;

use serde::Serialize;

use crate::client::Bot;
use crate::error::Result;
use crate::requests::{EditedMessageResult, RequestFuture};
use crate::types::{
    ChatId, EphemeralMessageParameters, Message, MessageEntity, MessageId, ParseMode, ReplyMarkup,
    ReplyParameters, clamp_widget_cache_time, validate_widget_data, validate_widget_html,
};

/// Payload for Bot API 10.4 `sendWidget`.
#[derive(Debug, Clone, Serialize)]
pub struct SendWidgetPayload {
    pub chat_id: ChatId,
    pub html: String,
    pub width: i32,
    pub height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_message_parameters: Option<EphemeralMessageParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Request builder for Bot API 10.4 `sendWidget`.
#[derive(Debug, Clone)]
pub struct SendWidget {
    bot: Bot,
    pub payload: SendWidgetPayload,
}

impl SendWidget {
    /// Creates a new `sendWidget` request with automatic placeholder sizing (`0x0`).
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, html: impl Into<String>) -> Self {
        Self {
            bot,
            payload: SendWidgetPayload {
                chat_id: chat_id.into(),
                html: html.into(),
                width: 0,
                height: 0,
                business_connection_id: None,
                message_thread_id: None,
                direct_messages_topic_id: None,
                is_shared: None,
                public_data: None,
                chat_data: None,
                cache_time: None,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                ephemeral_message_parameters: None,
                disable_notification: None,
                protect_content: None,
                allow_paid_broadcast: None,
                message_effect_id: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }

    /// Sets placeholder width and height in logical pixels (`0` for automatic on each axis).
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.payload.width = width;
        self.payload.height = height;
        self
    }

    /// Marks the widget as a persistent shared widget whose `widget.id` can be reused.
    pub fn shared(mut self, is_shared: bool) -> Self {
        self.payload.is_shared = Some(is_shared);
        self
    }

    /// Sets `cache_time` in seconds (clamped to `[5, 3600]`).
    pub fn cache_time(mut self, seconds: i32) -> Self {
        self.payload.cache_time = Some(clamp_widget_cache_time(seconds));
        self
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        message_thread_id(id: i64) -> message_thread_id,
        direct_messages_topic_id(id: i64) -> direct_messages_topic_id,
        public_data(data: impl Into<String>) -> public_data; into,
        chat_data(data: impl Into<String>) -> chat_data; into,
        caption(caption: impl Into<String>) -> caption; into,
        parse_mode(mode: ParseMode) -> parse_mode,
        caption_entities(entities: Vec<MessageEntity>) -> caption_entities,
        show_caption_above_media(show: bool) -> show_caption_above_media,
        ephemeral(params: EphemeralMessageParameters) -> ephemeral_message_parameters,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        allow_paid_broadcast(allow: bool) -> allow_paid_broadcast,
        message_effect_id(id: impl Into<String>) -> message_effect_id; into,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }

    /// Validates widget constraints and executes `sendWidget`.
    pub async fn send(self) -> Result<Message> {
        validate_widget_html(&self.payload.html)?;
        if let Some(ref data) = self.payload.public_data {
            validate_widget_data(data, "public_data")?;
        }
        if let Some(ref data) = self.payload.chat_data {
            validate_widget_data(data, "chat_data")?;
        }
        self.bot.call_json("sendWidget", &self.payload).await
    }
}

impl IntoFuture for SendWidget {
    type Output = Result<Message>;
    type IntoFuture = RequestFuture<'static, Message>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Payload for Bot API 10.4 `sendSharedWidget`.
#[derive(Debug, Clone, Serialize)]
pub struct SendSharedWidgetPayload {
    pub chat_id: ChatId,
    pub widget_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Request builder for Bot API 10.4 `sendSharedWidget`.
#[derive(Debug, Clone)]
pub struct SendSharedWidget {
    bot: Bot,
    pub payload: SendSharedWidgetPayload,
}

impl SendSharedWidget {
    /// Creates a new `sendSharedWidget` request referencing an existing `widget_id`.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, widget_id: impl Into<String>) -> Self {
        Self {
            bot,
            payload: SendSharedWidgetPayload {
                chat_id: chat_id.into(),
                widget_id: widget_id.into(),
                message_thread_id: None,
                chat_data: None,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                disable_notification: None,
                protect_content: None,
                allow_paid_broadcast: None,
                message_effect_id: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        message_thread_id(id: i64) -> message_thread_id,
        chat_data(data: impl Into<String>) -> chat_data; into,
        caption(caption: impl Into<String>) -> caption; into,
        parse_mode(mode: ParseMode) -> parse_mode,
        caption_entities(entities: Vec<MessageEntity>) -> caption_entities,
        show_caption_above_media(show: bool) -> show_caption_above_media,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        allow_paid_broadcast(allow: bool) -> allow_paid_broadcast,
        message_effect_id(id: impl Into<String>) -> message_effect_id; into,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }

    /// Validates `chat_data` and executes `sendSharedWidget`.
    pub async fn send(self) -> Result<Message> {
        if let Some(ref data) = self.payload.chat_data {
            validate_widget_data(data, "chat_data")?;
        }
        self.bot.call_json("sendSharedWidget", &self.payload).await
    }
}

impl IntoFuture for SendSharedWidget {
    type Output = Result<Message>;
    type IntoFuture = RequestFuture<'static, Message>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Payload for Bot API 10.4 `answerWidgetQuery`.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerWidgetQueryPayload {
    pub widget_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// Request builder for Bot API 10.4 `answerWidgetQuery`.
#[derive(Debug, Clone)]
pub struct AnswerWidgetQuery {
    bot: Bot,
    pub payload: AnswerWidgetQueryPayload,
}

impl AnswerWidgetQuery {
    /// Creates a new `answerWidgetQuery` request.
    pub fn new(bot: Bot, widget_query_id: impl Into<String>) -> Self {
        Self {
            bot,
            payload: AnswerWidgetQueryPayload {
                widget_query_id: widget_query_id.into(),
                data: None,
            },
        }
    }

    crate::request_setters! {
        data(data: impl Into<String>) -> data; into,
    }

    /// Validates response data length (`<= 1024` bytes) and executes `answerWidgetQuery`.
    pub async fn send(self) -> Result<bool> {
        if let Some(ref data) = self.payload.data {
            validate_widget_data(data, "data")?;
        }
        self.bot.call_json("answerWidgetQuery", &self.payload).await
    }
}

impl IntoFuture for AnswerWidgetQuery {
    type Output = Result<bool>;
    type IntoFuture = RequestFuture<'static, bool>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Payload for Bot API 10.4 `editMessageWidgetData`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct EditMessageWidgetDataPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,
}

/// Request builder for Bot API 10.4 `editMessageWidgetData`.
#[derive(Debug, Clone)]
pub struct EditMessageWidgetData {
    bot: Bot,
    pub payload: EditMessageWidgetDataPayload,
}

impl EditMessageWidgetData {
    /// Targets a widget in a regular chat message.
    pub fn for_message(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            bot,
            payload: EditMessageWidgetDataPayload {
                chat_id: Some(chat_id.into()),
                message_id: Some(message_id.into()),
                inline_message_id: None,
                chat_data: None,
            },
        }
    }

    /// Targets a widget in an inline message.
    pub fn for_inline_message(bot: Bot, inline_message_id: impl Into<String>) -> Self {
        Self {
            bot,
            payload: EditMessageWidgetDataPayload {
                chat_id: None,
                message_id: None,
                inline_message_id: Some(inline_message_id.into()),
                chat_data: None,
            },
        }
    }

    crate::request_setters! {
        chat_data(data: impl Into<String>) -> chat_data; into,
    }

    /// Validates `chat_data` and executes `editMessageWidgetData`.
    pub async fn send(self) -> Result<EditedMessageResult> {
        if let Some(ref data) = self.payload.chat_data {
            validate_widget_data(data, "chat_data")?;
        }
        self.bot
            .call_json("editMessageWidgetData", &self.payload)
            .await
    }
}

impl IntoFuture for EditMessageWidgetData {
    type Output = Result<EditedMessageResult>;
    type IntoFuture = RequestFuture<'static, EditedMessageResult>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Payload for Bot API 10.4 `editSharedWidgetData`.
#[derive(Debug, Clone, Serialize)]
pub struct EditSharedWidgetDataPayload {
    pub widget_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
}

/// Request builder for Bot API 10.4 `editSharedWidgetData`.
#[derive(Debug, Clone)]
pub struct EditSharedWidgetData {
    bot: Bot,
    pub payload: EditSharedWidgetDataPayload,
}

impl EditSharedWidgetData {
    /// Creates a new `editSharedWidgetData` request.
    pub fn new(bot: Bot, widget_id: impl Into<String>) -> Self {
        Self {
            bot,
            payload: EditSharedWidgetDataPayload {
                widget_id: widget_id.into(),
                public_data: None,
                cache_time: None,
            },
        }
    }

    /// Sets the shared widget's `cache_time` in seconds (clamped to `[5, 3600]`).
    pub fn cache_time(mut self, seconds: i32) -> Self {
        self.payload.cache_time = Some(clamp_widget_cache_time(seconds));
        self
    }

    crate::request_setters! {
        public_data(data: impl Into<String>) -> public_data; into,
    }

    /// Validates `public_data` and executes `editSharedWidgetData`.
    pub async fn send(self) -> Result<bool> {
        if let Some(ref data) = self.payload.public_data {
            validate_widget_data(data, "public_data")?;
        }
        self.bot
            .call_json("editSharedWidgetData", &self.payload)
            .await
    }
}

impl IntoFuture for EditSharedWidgetData {
    type Output = Result<bool>;
    type IntoFuture = RequestFuture<'static, bool>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

impl Bot {
    /// Sends an interactive HTML widget into a chat (Bot API 10.4 `sendWidget`).
    pub fn send_widget(&self, chat_id: impl Into<ChatId>, html: impl Into<String>) -> SendWidget {
        SendWidget::new(self.clone(), chat_id, html)
    }

    /// Resends an existing shared HTML widget by its `widget_id` (Bot API 10.4 `sendSharedWidget`).
    pub fn send_shared_widget(
        &self,
        chat_id: impl Into<ChatId>,
        widget_id: impl Into<String>,
    ) -> SendSharedWidget {
        SendSharedWidget::new(self.clone(), chat_id, widget_id)
    }

    /// Answers an incoming [`WidgetQuery`](crate::types::WidgetQuery) or [`SharedWidgetQuery`](crate::types::SharedWidgetQuery) (Bot API 10.4 `answerWidgetQuery`).
    pub fn answer_widget_query(&self, widget_query_id: impl Into<String>) -> AnswerWidgetQuery {
        AnswerWidgetQuery::new(self.clone(), widget_query_id)
    }

    /// Edits the `chat_data` of a widget embedded in a chat message (Bot API 10.4 `editMessageWidgetData`).
    pub fn edit_message_widget_data(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> EditMessageWidgetData {
        EditMessageWidgetData::for_message(self.clone(), chat_id, message_id)
    }

    /// Edits the `chat_data` of a widget embedded in an inline message (Bot API 10.4 `editMessageWidgetData`).
    pub fn edit_inline_widget_data(
        &self,
        inline_message_id: impl Into<String>,
    ) -> EditMessageWidgetData {
        EditMessageWidgetData::for_inline_message(self.clone(), inline_message_id)
    }

    /// Edits the shared `public_data` and/or `cache_time` of a shared widget (Bot API 10.4 `editSharedWidgetData`).
    pub fn edit_shared_widget_data(&self, widget_id: impl Into<String>) -> EditSharedWidgetData {
        EditSharedWidgetData::new(self.clone(), widget_id)
    }
}
