//! Core Bot API message sending, media upload, editing, deletion, and file retrieval requests.

use std::future::IntoFuture;

use serde::Serialize;

use crate::client::Bot;
use crate::error::Result;
use crate::requests::{EditedMessageResult, RequestFuture, execute_media_request};
use crate::types::{
    ChatId, EphemeralMessageParameters, File, InlineKeyboardMarkup, InputChecklist, InputFile,
    InputMedia, InputPollOption, InputRichMessage, LinkPreviewOptions, Message, MessageEntity,
    MessageId, ParseMode, Poll, ReactionType, ReplyMarkup, ReplyParameters, User,
};

/// Payload for `getMe`, `logOut`, `close`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct EmptyPayload {}

/// Request builder for `getMe`.
#[derive(Debug, Clone)]
pub struct GetMe {
    bot: Bot,
    pub payload: EmptyPayload,
}

impl GetMe {
    /// Creates a new `getMe` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: EmptyPayload {},
        }
    }
}

crate::impl_json_request!(GetMe, "getMe", User);

/// Request builder for `logOut`.
#[derive(Debug, Clone)]
pub struct LogOut {
    bot: Bot,
    pub payload: EmptyPayload,
}

impl LogOut {
    /// Creates a new `logOut` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: EmptyPayload {},
        }
    }
}

crate::impl_json_request!(LogOut, "logOut", bool);

/// Request builder for `close`.
#[derive(Debug, Clone)]
pub struct Close {
    bot: Bot,
    pub payload: EmptyPayload,
}

impl Close {
    /// Creates a new `close` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: EmptyPayload {},
        }
    }
}

crate::impl_json_request!(Close, "close", bool);

/// Payload for `sendMessage` (Bot API 10.4).
#[derive(Debug, Clone, Serialize)]
pub struct SendMessagePayload {
    pub chat_id: ChatId,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
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

/// Request builder for `sendMessage`.
#[derive(Debug, Clone)]
pub struct SendMessage {
    bot: Bot,
    pub payload: SendMessagePayload,
}

impl SendMessage {
    /// Creates a new `sendMessage` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, text: impl Into<String>) -> Self {
        Self {
            bot,
            payload: SendMessagePayload {
                chat_id: chat_id.into(),
                text: text.into(),
                business_connection_id: None,
                message_thread_id: None,
                direct_messages_topic_id: None,
                parse_mode: None,
                entities: None,
                link_preview_options: None,
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

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        message_thread_id(id: i64) -> message_thread_id,
        direct_messages_topic_id(id: i64) -> direct_messages_topic_id,
        parse_mode(mode: ParseMode) -> parse_mode,
        entities(entities: Vec<MessageEntity>) -> entities,
        link_preview_options(opts: LinkPreviewOptions) -> link_preview_options,
        ephemeral(params: EphemeralMessageParameters) -> ephemeral_message_parameters,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        allow_paid_broadcast(allow: bool) -> allow_paid_broadcast,
        message_effect_id(id: impl Into<String>) -> message_effect_id; into,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }
}

crate::impl_json_request!(SendMessage, "sendMessage", Message);

/// Payload for Bot API 9.3–10.3 `sendMessageDraft`.
#[derive(Debug, Clone, Serialize)]
pub struct SendMessageDraftPayload {
    pub chat_id: ChatId,
    pub draft_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_on_stop: Option<bool>,
}

/// Request builder for streaming plaintext drafts via `sendMessageDraft` (Bot API 9.3–10.3).
#[derive(Debug, Clone)]
pub struct SendMessageDraft {
    bot: Bot,
    pub payload: SendMessageDraftPayload,
}

impl SendMessageDraft {
    /// Creates a new `sendMessageDraft` request (`draft_id` must be non-zero).
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, draft_id: i64) -> Self {
        Self {
            bot,
            payload: SendMessageDraftPayload {
                chat_id: chat_id.into(),
                draft_id,
                message_thread_id: None,
                text: None,
                parse_mode: None,
                entities: None,
                can_stop: None,
                keep_on_stop: None,
            },
        }
    }

    crate::request_setters! {
        message_thread_id(id: i64) -> message_thread_id,
        text(text: impl Into<String>) -> text; into,
        parse_mode(mode: ParseMode) -> parse_mode,
        entities(entities: Vec<MessageEntity>) -> entities,
        can_stop(can_stop: bool) -> can_stop,
        keep_on_stop(keep: bool) -> keep_on_stop,
    }
}

crate::impl_json_request!(SendMessageDraft, "sendMessageDraft", bool);

/// Payload for `forwardMessage`.
#[derive(Debug, Clone, Serialize)]
pub struct ForwardMessagePayload {
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_start_timestamp: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

/// Request builder for `forwardMessage`.
#[derive(Debug, Clone)]
pub struct ForwardMessage {
    bot: Bot,
    pub payload: ForwardMessagePayload,
}

impl ForwardMessage {
    /// Creates a new `forwardMessage` request.
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            bot,
            payload: ForwardMessagePayload {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_id: message_id.into(),
                message_thread_id: None,
                video_start_timestamp: None,
                disable_notification: None,
                protect_content: None,
            },
        }
    }

    crate::request_setters! {
        message_thread_id(id: i64) -> message_thread_id,
        video_start_timestamp(secs: u32) -> video_start_timestamp,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
    }
}

crate::impl_json_request!(ForwardMessage, "forwardMessage", Message);

/// Returned ID wrapper from `copyMessage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub struct SentMessageId {
    /// Identifier of the newly copied message.
    pub message_id: MessageId,
}

/// Payload for `copyMessage`.
#[derive(Debug, Clone, Serialize)]
pub struct CopyMessagePayload {
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
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
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Request builder for `copyMessage`.
#[derive(Debug, Clone)]
pub struct CopyMessage {
    bot: Bot,
    pub payload: CopyMessagePayload,
}

impl CopyMessage {
    /// Creates a new `copyMessage` request.
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            bot,
            payload: CopyMessagePayload {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_id: message_id.into(),
                message_thread_id: None,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        message_thread_id(id: i64) -> message_thread_id,
        caption(caption: impl Into<String>) -> caption; into,
        parse_mode(mode: ParseMode) -> parse_mode,
        caption_entities(entities: Vec<MessageEntity>) -> caption_entities,
        show_caption_above_media(show: bool) -> show_caption_above_media,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }
}

crate::impl_json_request!(CopyMessage, "copyMessage", SentMessageId);

/// Shared metadata payload used across media-sending methods (`sendPhoto`, `sendDocument`, etc.).
#[derive(Debug, Clone, Serialize)]
pub struct MediaMessagePayload {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
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

/// Unified request builder for uploading or referencing a media message
/// (`sendPhoto`, `sendDocument`, `sendVideo`, `sendAudio`, `sendAnimation`, `sendVoice`, `sendVideoNote`).
#[derive(Debug, Clone)]
pub struct SendMediaFile {
    bot: Bot,
    method: &'static str,
    file_field: &'static str,
    file: InputFile,
    thumbnail: Option<InputFile>,
    pub payload: MediaMessagePayload,
}

impl SendMediaFile {
    pub(crate) fn new(
        bot: Bot,
        method: &'static str,
        file_field: &'static str,
        chat_id: impl Into<ChatId>,
        file: InputFile,
    ) -> Self {
        Self {
            bot,
            method,
            file_field,
            file,
            thumbnail: None,
            payload: MediaMessagePayload {
                chat_id: chat_id.into(),
                business_connection_id: None,
                message_thread_id: None,
                direct_messages_topic_id: None,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                has_spoiler: None,
                duration: None,
                width: None,
                height: None,
                length: None,
                performer: None,
                title: None,
                supports_streaming: None,
                disable_content_type_detection: None,
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

    /// Attaches an optional thumbnail file.
    pub fn thumbnail(mut self, thumb: InputFile) -> Self {
        self.thumbnail = Some(thumb);
        self
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        message_thread_id(id: i64) -> message_thread_id,
        direct_messages_topic_id(id: i64) -> direct_messages_topic_id,
        caption(caption: impl Into<String>) -> caption; into,
        parse_mode(mode: ParseMode) -> parse_mode,
        caption_entities(entities: Vec<MessageEntity>) -> caption_entities,
        show_caption_above_media(show: bool) -> show_caption_above_media,
        has_spoiler(spoiler: bool) -> has_spoiler,
        duration(secs: u32) -> duration,
        width(width: u32) -> width,
        height(height: u32) -> height,
        length(length: u32) -> length,
        performer(performer: impl Into<String>) -> performer; into,
        title(title: impl Into<String>) -> title; into,
        supports_streaming(supports: bool) -> supports_streaming,
        disable_content_type_detection(disable: bool) -> disable_content_type_detection,
        ephemeral(params: EphemeralMessageParameters) -> ephemeral_message_parameters,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        allow_paid_broadcast(allow: bool) -> allow_paid_broadcast,
        message_effect_id(id: impl Into<String>) -> message_effect_id; into,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }

    /// Executes the media upload or JSON send request.
    pub async fn send(self) -> Result<Message> {
        execute_media_request(
            &self.bot,
            self.method,
            self.file_field,
            &self.file,
            Some("thumbnail"),
            self.thumbnail.as_ref(),
            &self.payload,
        )
        .await
    }
}

impl IntoFuture for SendMediaFile {
    type Output = Result<Message>;
    type IntoFuture = RequestFuture<'static, Message>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Payload for `sendMediaGroup`.
#[derive(Debug, Clone, Serialize)]
pub struct SendMediaGroupPayload {
    pub chat_id: ChatId,
    pub media: Vec<InputMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
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
}

/// Request builder for `sendMediaGroup`.
#[derive(Debug, Clone)]
pub struct SendMediaGroup {
    bot: Bot,
    pub payload: SendMediaGroupPayload,
}

impl SendMediaGroup {
    /// Creates a new `sendMediaGroup` request (2-10 items).
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, media: Vec<InputMedia>) -> Self {
        Self {
            bot,
            payload: SendMediaGroupPayload {
                chat_id: chat_id.into(),
                media,
                business_connection_id: None,
                message_thread_id: None,
                disable_notification: None,
                protect_content: None,
                allow_paid_broadcast: None,
                message_effect_id: None,
                reply_parameters: None,
            },
        }
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        message_thread_id(id: i64) -> message_thread_id,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        allow_paid_broadcast(allow: bool) -> allow_paid_broadcast,
        message_effect_id(id: impl Into<String>) -> message_effect_id; into,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
    }
}

crate::impl_json_request!(SendMediaGroup, "sendMediaGroup", Vec<Message>);

/// Payload for `sendLocation`.
#[derive(Debug, Clone, Serialize)]
pub struct SendLocationPayload {
    pub chat_id: ChatId,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Request builder for `sendLocation`.
#[derive(Debug, Clone)]
pub struct SendLocation {
    bot: Bot,
    pub payload: SendLocationPayload,
}

impl SendLocation {
    /// Creates a new `sendLocation` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, latitude: f64, longitude: f64) -> Self {
        Self {
            bot,
            payload: SendLocationPayload {
                chat_id: chat_id.into(),
                latitude,
                longitude,
                message_thread_id: None,
                horizontal_accuracy: None,
                live_period: None,
                heading: None,
                proximity_alert_radius: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        message_thread_id(id: i64) -> message_thread_id,
        horizontal_accuracy(meters: f64) -> horizontal_accuracy,
        live_period(secs: u32) -> live_period,
        heading(degrees: u16) -> heading,
        proximity_alert_radius(meters: u32) -> proximity_alert_radius,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }
}

crate::impl_json_request!(SendLocation, "sendLocation", Message);

/// Payload for `sendPoll`.
#[derive(Debug, Clone, Serialize)]
pub struct SendPollPayload {
    pub chat_id: ChatId,
    pub question: String,
    pub options: Vec<InputPollOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub poll_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Request builder for `sendPoll`.
#[derive(Debug, Clone)]
pub struct SendPoll {
    bot: Bot,
    pub payload: SendPollPayload,
}

impl SendPoll {
    /// Creates a new `sendPoll` request.
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<InputPollOption>,
    ) -> Self {
        Self {
            bot,
            payload: SendPollPayload {
                chat_id: chat_id.into(),
                question: question.into(),
                options,
                message_thread_id: None,
                question_parse_mode: None,
                is_anonymous: None,
                poll_type: None,
                allows_multiple_answers: None,
                correct_option_id: None,
                explanation: None,
                explanation_parse_mode: None,
                open_period: None,
                close_date: None,
                is_closed: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        message_thread_id(id: i64) -> message_thread_id,
        question_parse_mode(mode: ParseMode) -> question_parse_mode,
        is_anonymous(anon: bool) -> is_anonymous,
        poll_type(kind: impl Into<String>) -> poll_type; into,
        allows_multiple_answers(allow: bool) -> allows_multiple_answers,
        correct_option_id(idx: u32) -> correct_option_id,
        explanation(text: impl Into<String>) -> explanation; into,
        explanation_parse_mode(mode: ParseMode) -> explanation_parse_mode,
        open_period(secs: u32) -> open_period,
        close_date(ts: i64) -> close_date,
        is_closed(closed: bool) -> is_closed,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }
}

crate::impl_json_request!(SendPoll, "sendPoll", Message);

/// Payload for Bot API 9.1 `sendChecklist`.
#[derive(Debug, Clone, Serialize)]
pub struct SendChecklistPayload {
    pub business_connection_id: String,
    pub chat_id: i64,
    pub checklist: InputChecklist,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Request builder for `sendChecklist` (Bot API 9.1).
#[derive(Debug, Clone)]
pub struct SendChecklist {
    bot: Bot,
    pub payload: SendChecklistPayload,
}

impl SendChecklist {
    /// Creates a new `sendChecklist` request.
    pub fn new(
        bot: Bot,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        checklist: InputChecklist,
    ) -> Self {
        Self {
            bot,
            payload: SendChecklistPayload {
                business_connection_id: business_connection_id.into(),
                chat_id,
                checklist,
                disable_notification: None,
                protect_content: None,
                message_effect_id: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        message_effect_id(id: impl Into<String>) -> message_effect_id; into,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(SendChecklist, "sendChecklist", Message);

/// Payload for `sendDice`.
#[derive(Debug, Clone, Serialize)]
pub struct SendDicePayload {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// Request builder for `sendDice`.
#[derive(Debug, Clone)]
pub struct SendDice {
    bot: Bot,
    pub payload: SendDicePayload,
}

impl SendDice {
    /// Creates a new `sendDice` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            payload: SendDicePayload {
                chat_id: chat_id.into(),
                message_thread_id: None,
                emoji: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        message_thread_id(id: i64) -> message_thread_id,
        emoji(emoji: impl Into<String>) -> emoji; into,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }
}

crate::impl_json_request!(SendDice, "sendDice", Message);

/// Chat action displayed in the header (e.g. `"typing"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordVoice,
    UploadVoice,
    UploadDocument,
    ChooseSticker,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

/// Payload for `sendChatAction`.
#[derive(Debug, Clone, Serialize)]
pub struct SendChatActionPayload {
    pub chat_id: ChatId,
    pub action: ChatAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
}

/// Request builder for `sendChatAction`.
#[derive(Debug, Clone)]
pub struct SendChatAction {
    bot: Bot,
    pub payload: SendChatActionPayload,
}

impl SendChatAction {
    /// Creates a new `sendChatAction` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, action: ChatAction) -> Self {
        Self {
            bot,
            payload: SendChatActionPayload {
                chat_id: chat_id.into(),
                action,
                business_connection_id: None,
                message_thread_id: None,
            },
        }
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        message_thread_id(id: i64) -> message_thread_id,
    }
}

crate::impl_json_request!(SendChatAction, "sendChatAction", bool);

/// Payload for `setMessageReaction`.
#[derive(Debug, Clone, Serialize)]
pub struct SetMessageReactionPayload {
    pub chat_id: ChatId,
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Vec<ReactionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

/// Request builder for `setMessageReaction`.
#[derive(Debug, Clone)]
pub struct SetMessageReaction {
    bot: Bot,
    pub payload: SetMessageReactionPayload,
}

impl SetMessageReaction {
    /// Creates a new `setMessageReaction` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, message_id: impl Into<MessageId>) -> Self {
        Self {
            bot,
            payload: SetMessageReactionPayload {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
                reaction: None,
                is_big: None,
            },
        }
    }

    crate::request_setters! {
        reaction(reactions: Vec<ReactionType>) -> reaction,
        is_big(is_big: bool) -> is_big,
    }
}

crate::impl_json_request!(SetMessageReaction, "setMessageReaction", bool);

/// Payload for `editMessageText` (Bot API 10.2+ also accepts `rich_message`).
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageTextPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_message: Option<InputRichMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Request builder for `editMessageText`.
#[derive(Debug, Clone)]
pub struct EditMessageText {
    bot: Bot,
    pub payload: EditMessageTextPayload,
}

impl EditMessageText {
    /// Edits a regular chat message's text.
    pub fn for_message(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            payload: EditMessageTextPayload {
                chat_id: Some(chat_id.into()),
                message_id: Some(message_id.into()),
                inline_message_id: None,
                business_connection_id: None,
                text: Some(text.into()),
                rich_message: None,
                parse_mode: None,
                entities: None,
                link_preview_options: None,
                reply_markup: None,
            },
        }
    }

    /// Edits an inline/guest message's text.
    pub fn for_inline_message(
        bot: Bot,
        inline_message_id: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            payload: EditMessageTextPayload {
                chat_id: None,
                message_id: None,
                inline_message_id: Some(inline_message_id.into()),
                business_connection_id: None,
                text: Some(text.into()),
                rich_message: None,
                parse_mode: None,
                entities: None,
                link_preview_options: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        rich_message(rich: InputRichMessage) -> rich_message,
        parse_mode(mode: ParseMode) -> parse_mode,
        entities(entities: Vec<MessageEntity>) -> entities,
        link_preview_options(opts: LinkPreviewOptions) -> link_preview_options,
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(EditMessageText, "editMessageText", EditedMessageResult);

/// Payload for `editMessageCaption`.
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageCaptionPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Request builder for `editMessageCaption`.
#[derive(Debug, Clone)]
pub struct EditMessageCaption {
    bot: Bot,
    pub payload: EditMessageCaptionPayload,
}

impl EditMessageCaption {
    /// Targets a chat message.
    pub fn for_message(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            bot,
            payload: EditMessageCaptionPayload {
                chat_id: Some(chat_id.into()),
                message_id: Some(message_id.into()),
                inline_message_id: None,
                business_connection_id: None,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        caption(caption: impl Into<String>) -> caption; into,
        parse_mode(mode: ParseMode) -> parse_mode,
        caption_entities(entities: Vec<MessageEntity>) -> caption_entities,
        show_caption_above_media(show: bool) -> show_caption_above_media,
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(
    EditMessageCaption,
    "editMessageCaption",
    EditedMessageResult
);

/// Payload for `editMessageReplyMarkup`.
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageReplyMarkupPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Request builder for `editMessageReplyMarkup`.
#[derive(Debug, Clone)]
pub struct EditMessageReplyMarkup {
    bot: Bot,
    pub payload: EditMessageReplyMarkupPayload,
}

impl EditMessageReplyMarkup {
    /// Targets a chat message.
    pub fn for_message(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            bot,
            payload: EditMessageReplyMarkupPayload {
                chat_id: Some(chat_id.into()),
                message_id: Some(message_id.into()),
                inline_message_id: None,
                business_connection_id: None,
                reply_markup: None,
            },
        }
    }

    /// Targets an inline message.
    pub fn for_inline_message(bot: Bot, inline_message_id: impl Into<String>) -> Self {
        Self {
            bot,
            payload: EditMessageReplyMarkupPayload {
                chat_id: None,
                message_id: None,
                inline_message_id: Some(inline_message_id.into()),
                business_connection_id: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(
    EditMessageReplyMarkup,
    "editMessageReplyMarkup",
    EditedMessageResult
);

/// Payload for `stopPoll`.
#[derive(Debug, Clone, Serialize)]
pub struct StopPollPayload {
    pub chat_id: ChatId,
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Request builder for `stopPoll`.
#[derive(Debug, Clone)]
pub struct StopPoll {
    bot: Bot,
    pub payload: StopPollPayload,
}

impl StopPoll {
    /// Creates a new `stopPoll` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, message_id: impl Into<MessageId>) -> Self {
        Self {
            bot,
            payload: StopPollPayload {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
                business_connection_id: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(StopPoll, "stopPoll", Poll);

/// Payload for `deleteMessage`.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteMessagePayload {
    pub chat_id: ChatId,
    pub message_id: MessageId,
}

/// Request builder for `deleteMessage`.
#[derive(Debug, Clone)]
pub struct DeleteMessage {
    bot: Bot,
    pub payload: DeleteMessagePayload,
}

impl DeleteMessage {
    /// Creates a new `deleteMessage` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, message_id: impl Into<MessageId>) -> Self {
        Self {
            bot,
            payload: DeleteMessagePayload {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
            },
        }
    }
}

crate::impl_json_request!(DeleteMessage, "deleteMessage", bool);

/// Payload for `deleteMessages`.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteMessagesPayload {
    pub chat_id: ChatId,
    pub message_ids: Vec<MessageId>,
}

/// Request builder for `deleteMessages`.
#[derive(Debug, Clone)]
pub struct DeleteMessages {
    bot: Bot,
    pub payload: DeleteMessagesPayload,
}

impl DeleteMessages {
    /// Creates a new `deleteMessages` request (1-100 message IDs).
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, message_ids: Vec<MessageId>) -> Self {
        Self {
            bot,
            payload: DeleteMessagesPayload {
                chat_id: chat_id.into(),
                message_ids,
            },
        }
    }
}

crate::impl_json_request!(DeleteMessages, "deleteMessages", bool);

/// Payload for `getFile`.
#[derive(Debug, Clone, Serialize)]
pub struct GetFilePayload {
    pub file_id: String,
}

/// Request builder for `getFile`.
#[derive(Debug, Clone)]
pub struct GetFile {
    bot: Bot,
    pub payload: GetFilePayload,
}

impl GetFile {
    /// Creates a new `getFile` request.
    pub fn new(bot: Bot, file_id: impl Into<String>) -> Self {
        Self {
            bot,
            payload: GetFilePayload {
                file_id: file_id.into(),
            },
        }
    }
}

crate::impl_json_request!(GetFile, "getFile", File);

impl Bot {
    /// Tests the bot's auth token and returns basic information about the bot (`getMe`).
    pub fn get_me(&self) -> GetMe {
        GetMe::new(self.clone())
    }

    /// Logs out from the cloud Bot API server before launching locally (`logOut`).
    pub fn log_out(&self) -> LogOut {
        LogOut::new(self.clone())
    }

    /// Closes the bot instance before moving to another local server (`close`).
    pub fn close(&self) -> Close {
        Close::new(self.clone())
    }

    /// Sends a text message (`sendMessage`).
    pub fn send_message(&self, chat_id: impl Into<ChatId>, text: impl Into<String>) -> SendMessage {
        SendMessage::new(self.clone(), chat_id, text)
    }

    /// Streams a partial plaintext message draft (Bot API 9.3–10.3 `sendMessageDraft`).
    pub fn send_message_draft(
        &self,
        chat_id: impl Into<ChatId>,
        draft_id: i64,
    ) -> SendMessageDraft {
        SendMessageDraft::new(self.clone(), chat_id, draft_id)
    }

    /// Forwards a message (`forwardMessage`).
    pub fn forward_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> ForwardMessage {
        ForwardMessage::new(self.clone(), chat_id, from_chat_id, message_id)
    }

    /// Copies a message (`copyMessage`).
    pub fn copy_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> CopyMessage {
        CopyMessage::new(self.clone(), chat_id, from_chat_id, message_id)
    }

    /// Sends a photo (`sendPhoto`).
    pub fn send_photo(&self, chat_id: impl Into<ChatId>, photo: InputFile) -> SendMediaFile {
        SendMediaFile::new(self.clone(), "sendPhoto", "photo", chat_id, photo)
    }

    /// Sends a general file (`sendDocument`).
    pub fn send_document(&self, chat_id: impl Into<ChatId>, document: InputFile) -> SendMediaFile {
        SendMediaFile::new(self.clone(), "sendDocument", "document", chat_id, document)
    }

    /// Sends a video (`sendVideo`).
    pub fn send_video(&self, chat_id: impl Into<ChatId>, video: InputFile) -> SendMediaFile {
        SendMediaFile::new(self.clone(), "sendVideo", "video", chat_id, video)
    }

    /// Sends an audio track (`sendAudio`).
    pub fn send_audio(&self, chat_id: impl Into<ChatId>, audio: InputFile) -> SendMediaFile {
        SendMediaFile::new(self.clone(), "sendAudio", "audio", chat_id, audio)
    }

    /// Sends an animation (`sendAnimation`).
    pub fn send_animation(
        &self,
        chat_id: impl Into<ChatId>,
        animation: InputFile,
    ) -> SendMediaFile {
        SendMediaFile::new(
            self.clone(),
            "sendAnimation",
            "animation",
            chat_id,
            animation,
        )
    }

    /// Sends a voice note (`sendVoice`).
    pub fn send_voice(&self, chat_id: impl Into<ChatId>, voice: InputFile) -> SendMediaFile {
        SendMediaFile::new(self.clone(), "sendVoice", "voice", chat_id, voice)
    }

    /// Sends a round video message (`sendVideoNote`).
    pub fn send_video_note(
        &self,
        chat_id: impl Into<ChatId>,
        video_note: InputFile,
    ) -> SendMediaFile {
        SendMediaFile::new(
            self.clone(),
            "sendVideoNote",
            "video_note",
            chat_id,
            video_note,
        )
    }

    /// Sends a group of photos, videos, documents, or audios as an album (`sendMediaGroup`).
    pub fn send_media_group(
        &self,
        chat_id: impl Into<ChatId>,
        media: Vec<InputMedia>,
    ) -> SendMediaGroup {
        SendMediaGroup::new(self.clone(), chat_id, media)
    }

    /// Sends a point on the map (`sendLocation`).
    pub fn send_location(
        &self,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
    ) -> SendLocation {
        SendLocation::new(self.clone(), chat_id, latitude, longitude)
    }

    /// Sends a native poll (`sendPoll`).
    pub fn send_poll(
        &self,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<InputPollOption>,
    ) -> SendPoll {
        SendPoll::new(self.clone(), chat_id, question, options)
    }

    /// Sends an interactive checklist on behalf of a business connection (Bot API 9.1 `sendChecklist`).
    pub fn send_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        checklist: InputChecklist,
    ) -> SendChecklist {
        SendChecklist::new(self.clone(), business_connection_id, chat_id, checklist)
    }

    /// Sends an animated emoji displaying a random value (`sendDice`).
    pub fn send_dice(&self, chat_id: impl Into<ChatId>) -> SendDice {
        SendDice::new(self.clone(), chat_id)
    }

    /// Broadcasts a temporary status indicator such as typing (`sendChatAction`).
    pub fn send_chat_action(
        &self,
        chat_id: impl Into<ChatId>,
        action: ChatAction,
    ) -> SendChatAction {
        SendChatAction::new(self.clone(), chat_id, action)
    }

    /// Sets reactions on a message (`setMessageReaction`).
    pub fn set_message_reaction(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> SetMessageReaction {
        SetMessageReaction::new(self.clone(), chat_id, message_id)
    }

    /// Edits the text of a chat message (`editMessageText`).
    pub fn edit_message_text(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
        text: impl Into<String>,
    ) -> EditMessageText {
        EditMessageText::for_message(self.clone(), chat_id, message_id, text)
    }

    /// Edits the text of an inline or guest message (`editMessageText`).
    pub fn edit_inline_message_text(
        &self,
        inline_message_id: impl Into<String>,
        text: impl Into<String>,
    ) -> EditMessageText {
        EditMessageText::for_inline_message(self.clone(), inline_message_id, text)
    }

    /// Edits the caption of a message (`editMessageCaption`).
    pub fn edit_message_caption(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> EditMessageCaption {
        EditMessageCaption::for_message(self.clone(), chat_id, message_id)
    }

    /// Edits the inline keyboard of a chat message (`editMessageReplyMarkup`).
    pub fn edit_message_reply_markup(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::for_message(self.clone(), chat_id, message_id)
    }

    /// Edits the inline keyboard of an inline message (`editMessageReplyMarkup`).
    pub fn edit_inline_message_reply_markup(
        &self,
        inline_message_id: impl Into<String>,
    ) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::for_inline_message(self.clone(), inline_message_id)
    }

    /// Stops a poll sent by the bot (`stopPoll`).
    pub fn stop_poll(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> StopPoll {
        StopPoll::new(self.clone(), chat_id, message_id)
    }

    /// Deletes a single message (`deleteMessage`).
    pub fn delete_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> DeleteMessage {
        DeleteMessage::new(self.clone(), chat_id, message_id)
    }

    /// Deletes multiple messages in a single call (`deleteMessages`).
    pub fn delete_messages(
        &self,
        chat_id: impl Into<ChatId>,
        message_ids: Vec<MessageId>,
    ) -> DeleteMessages {
        DeleteMessages::new(self.clone(), chat_id, message_ids)
    }

    /// Fetches file download metadata (`getFile`).
    pub fn get_file(&self, file_id: impl Into<String>) -> GetFile {
        GetFile::new(self.clone(), file_id)
    }
}
