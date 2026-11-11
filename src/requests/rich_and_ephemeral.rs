//! Bot API 10.1–10.4 Rich Messages and Ephemeral Message request builders.

use serde::Serialize;

use crate::client::Bot;
use crate::types::{
    ChatId, EphemeralMessageParameters, InlineKeyboardMarkup, InputMedia, InputRichMessage,
    LinkPreviewOptions, Message, MessageEntity, MessageId, ParseMode, ReplyMarkup, ReplyParameters,
    UserId,
};

/// Payload for Bot API 10.1 `sendRichMessage`.
#[derive(Debug, Clone, Serialize)]
pub struct SendRichMessagePayload {
    pub chat_id: ChatId,
    pub rich_message: InputRichMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
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

/// Request builder for `sendRichMessage` (Bot API 10.1–10.4).
#[derive(Debug, Clone)]
pub struct SendRichMessage {
    bot: Bot,
    pub payload: SendRichMessagePayload,
}

impl SendRichMessage {
    /// Creates a new `sendRichMessage` builder.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, rich_message: InputRichMessage) -> Self {
        Self {
            bot,
            payload: SendRichMessagePayload {
                chat_id: chat_id.into(),
                rich_message,
                business_connection_id: None,
                message_thread_id: None,
                direct_messages_topic_id: None,
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
        ephemeral(params: EphemeralMessageParameters) -> ephemeral_message_parameters,
        disable_notification(disable: bool) -> disable_notification,
        protect_content(protect: bool) -> protect_content,
        allow_paid_broadcast(allow: bool) -> allow_paid_broadcast,
        message_effect_id(id: impl Into<String>) -> message_effect_id; into,
        reply_parameters(params: ReplyParameters) -> reply_parameters,
        reply_markup(markup: impl Into<ReplyMarkup>) -> reply_markup; into,
    }
}

crate::impl_json_request!(SendRichMessage, "sendRichMessage", Message);

/// Payload for Bot API 10.1 `sendRichMessageDraft`.
#[derive(Debug, Clone, Serialize)]
pub struct SendRichMessageDraftPayload {
    pub chat_id: ChatId,
    pub draft_id: i64,
    pub rich_message: InputRichMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_on_stop: Option<bool>,
}

/// Request builder for streaming rich drafts via `sendRichMessageDraft` (Bot API 10.1–10.3).
#[derive(Debug, Clone)]
pub struct SendRichMessageDraft {
    bot: Bot,
    pub payload: SendRichMessageDraftPayload,
}

impl SendRichMessageDraft {
    /// Creates a new `sendRichMessageDraft` builder (`draft_id` must be non-zero).
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        draft_id: i64,
        rich_message: InputRichMessage,
    ) -> Self {
        Self {
            bot,
            payload: SendRichMessageDraftPayload {
                chat_id: chat_id.into(),
                draft_id,
                rich_message,
                message_thread_id: None,
                can_stop: None,
                keep_on_stop: None,
            },
        }
    }

    crate::request_setters! {
        message_thread_id(id: i64) -> message_thread_id,
        can_stop(can_stop: bool) -> can_stop,
        keep_on_stop(keep: bool) -> keep_on_stop,
    }
}

crate::impl_json_request!(SendRichMessageDraft, "sendRichMessageDraft", bool);

/// Payload for Bot API 10.2 `editEphemeralMessageText`.
#[derive(Debug, Clone, Serialize)]
pub struct EditEphemeralMessageTextPayload {
    pub chat_id: ChatId,
    pub receiver_user_id: UserId,
    pub ephemeral_message_id: MessageId,
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

/// Request builder for `editEphemeralMessageText`.
#[derive(Debug, Clone)]
pub struct EditEphemeralMessageText {
    bot: Bot,
    pub payload: EditEphemeralMessageTextPayload,
}

impl EditEphemeralMessageText {
    /// Edits the text of an ephemeral message.
    pub fn text(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            payload: EditEphemeralMessageTextPayload {
                chat_id: chat_id.into(),
                receiver_user_id: receiver_user_id.into(),
                ephemeral_message_id: ephemeral_message_id.into(),
                text: Some(text.into()),
                rich_message: None,
                parse_mode: None,
                entities: None,
                link_preview_options: None,
                reply_markup: None,
            },
        }
    }

    /// Replaces the ephemeral message content with an [`InputRichMessage`].
    pub fn rich_message(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
        rich_message: InputRichMessage,
    ) -> Self {
        Self {
            bot,
            payload: EditEphemeralMessageTextPayload {
                chat_id: chat_id.into(),
                receiver_user_id: receiver_user_id.into(),
                ephemeral_message_id: ephemeral_message_id.into(),
                text: None,
                rich_message: Some(rich_message),
                parse_mode: None,
                entities: None,
                link_preview_options: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        parse_mode(mode: ParseMode) -> parse_mode,
        entities(entities: Vec<MessageEntity>) -> entities,
        link_preview_options(opts: LinkPreviewOptions) -> link_preview_options,
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(
    EditEphemeralMessageText,
    "editEphemeralMessageText",
    Message
);

/// Payload for Bot API 10.2 `editEphemeralMessageCaption`.
#[derive(Debug, Clone, Serialize)]
pub struct EditEphemeralMessageCaptionPayload {
    pub chat_id: ChatId,
    pub receiver_user_id: UserId,
    pub ephemeral_message_id: MessageId,
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

/// Request builder for `editEphemeralMessageCaption`.
#[derive(Debug, Clone)]
pub struct EditEphemeralMessageCaption {
    bot: Bot,
    pub payload: EditEphemeralMessageCaptionPayload,
}

impl EditEphemeralMessageCaption {
    /// Creates a new `editEphemeralMessageCaption` builder.
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            bot,
            payload: EditEphemeralMessageCaptionPayload {
                chat_id: chat_id.into(),
                receiver_user_id: receiver_user_id.into(),
                ephemeral_message_id: ephemeral_message_id.into(),
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        caption(caption: impl Into<String>) -> caption; into,
        parse_mode(mode: ParseMode) -> parse_mode,
        caption_entities(entities: Vec<MessageEntity>) -> caption_entities,
        show_caption_above_media(show: bool) -> show_caption_above_media,
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(
    EditEphemeralMessageCaption,
    "editEphemeralMessageCaption",
    Message
);

/// Payload for Bot API 10.2 `editEphemeralMessageMedia`.
#[derive(Debug, Clone, Serialize)]
pub struct EditEphemeralMessageMediaPayload {
    pub chat_id: ChatId,
    pub receiver_user_id: UserId,
    pub ephemeral_message_id: MessageId,
    pub media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Request builder for `editEphemeralMessageMedia`.
#[derive(Debug, Clone)]
pub struct EditEphemeralMessageMedia {
    bot: Bot,
    pub payload: EditEphemeralMessageMediaPayload,
}

impl EditEphemeralMessageMedia {
    /// Creates a new `editEphemeralMessageMedia` builder.
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
        media: InputMedia,
    ) -> Self {
        Self {
            bot,
            payload: EditEphemeralMessageMediaPayload {
                chat_id: chat_id.into(),
                receiver_user_id: receiver_user_id.into(),
                ephemeral_message_id: ephemeral_message_id.into(),
                media,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(
    EditEphemeralMessageMedia,
    "editEphemeralMessageMedia",
    Message
);

/// Payload for Bot API 10.2 `editEphemeralMessageReplyMarkup`.
#[derive(Debug, Clone, Serialize)]
pub struct EditEphemeralMessageReplyMarkupPayload {
    pub chat_id: ChatId,
    pub receiver_user_id: UserId,
    pub ephemeral_message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Request builder for `editEphemeralMessageReplyMarkup`.
#[derive(Debug, Clone)]
pub struct EditEphemeralMessageReplyMarkup {
    bot: Bot,
    pub payload: EditEphemeralMessageReplyMarkupPayload,
}

impl EditEphemeralMessageReplyMarkup {
    /// Creates a new `editEphemeralMessageReplyMarkup` builder.
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            bot,
            payload: EditEphemeralMessageReplyMarkupPayload {
                chat_id: chat_id.into(),
                receiver_user_id: receiver_user_id.into(),
                ephemeral_message_id: ephemeral_message_id.into(),
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        reply_markup(markup: InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(
    EditEphemeralMessageReplyMarkup,
    "editEphemeralMessageReplyMarkup",
    Message
);

/// Payload for Bot API 10.2 `deleteEphemeralMessage`.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteEphemeralMessagePayload {
    pub chat_id: ChatId,
    pub receiver_user_id: UserId,
    pub ephemeral_message_id: MessageId,
}

/// Request builder for `deleteEphemeralMessage`.
#[derive(Debug, Clone)]
pub struct DeleteEphemeralMessage {
    bot: Bot,
    pub payload: DeleteEphemeralMessagePayload,
}

impl DeleteEphemeralMessage {
    /// Creates a new `deleteEphemeralMessage` builder.
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            bot,
            payload: DeleteEphemeralMessagePayload {
                chat_id: chat_id.into(),
                receiver_user_id: receiver_user_id.into(),
                ephemeral_message_id: ephemeral_message_id.into(),
            },
        }
    }
}

crate::impl_json_request!(DeleteEphemeralMessage, "deleteEphemeralMessage", bool);

impl Bot {
    /// Sends a structured Rich Message (Bot API 10.1–10.4 `sendRichMessage`).
    pub fn send_rich_message(
        &self,
        chat_id: impl Into<ChatId>,
        rich_message: InputRichMessage,
    ) -> SendRichMessage {
        SendRichMessage::new(self.clone(), chat_id, rich_message)
    }

    /// Streams a partial Rich Message draft with optional stop button (Bot API 10.1–10.3 `sendRichMessageDraft`).
    pub fn send_rich_message_draft(
        &self,
        chat_id: impl Into<ChatId>,
        draft_id: i64,
        rich_message: InputRichMessage,
    ) -> SendRichMessageDraft {
        SendRichMessageDraft::new(self.clone(), chat_id, draft_id, rich_message)
    }

    /// Edits the text of an ephemeral message (Bot API 10.2 `editEphemeralMessageText`).
    pub fn edit_ephemeral_message_text(
        &self,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
        text: impl Into<String>,
    ) -> EditEphemeralMessageText {
        EditEphemeralMessageText::text(
            self.clone(),
            chat_id,
            receiver_user_id,
            ephemeral_message_id,
            text,
        )
    }

    /// Edits the caption of an ephemeral message (Bot API 10.2 `editEphemeralMessageCaption`).
    pub fn edit_ephemeral_message_caption(
        &self,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
    ) -> EditEphemeralMessageCaption {
        EditEphemeralMessageCaption::new(
            self.clone(),
            chat_id,
            receiver_user_id,
            ephemeral_message_id,
        )
    }

    /// Edits the media of an ephemeral message (Bot API 10.2 `editEphemeralMessageMedia`).
    pub fn edit_ephemeral_message_media(
        &self,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
        media: InputMedia,
    ) -> EditEphemeralMessageMedia {
        EditEphemeralMessageMedia::new(
            self.clone(),
            chat_id,
            receiver_user_id,
            ephemeral_message_id,
            media,
        )
    }

    /// Edits the inline keyboard of an ephemeral message (Bot API 10.2 `editEphemeralMessageReplyMarkup`).
    pub fn edit_ephemeral_message_reply_markup(
        &self,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
    ) -> EditEphemeralMessageReplyMarkup {
        EditEphemeralMessageReplyMarkup::new(
            self.clone(),
            chat_id,
            receiver_user_id,
            ephemeral_message_id,
        )
    }

    /// Deletes an ephemeral message visible to `receiver_user_id` (Bot API 10.2 `deleteEphemeralMessage`).
    pub fn delete_ephemeral_message(
        &self,
        chat_id: impl Into<ChatId>,
        receiver_user_id: impl Into<UserId>,
        ephemeral_message_id: impl Into<MessageId>,
    ) -> DeleteEphemeralMessage {
        DeleteEphemeralMessage::new(
            self.clone(),
            chat_id,
            receiver_user_id,
            ephemeral_message_id,
        )
    }
}
