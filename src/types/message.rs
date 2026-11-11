//! Message objects, entities, replies, checklists, ephemeral parameters, guest mode, and community service messages.

use serde::{Deserialize, Serialize};

use super::{
    Animation, Audio, Chat, ChatId, Community, Contact, Dice, Document, InlineKeyboardMarkup,
    JsonMap, LinkPreviewOptions, Location, MessageId, PaidMediaInfo, PhotoSize, Poll, RichMessage,
    Sticker, User, UserId, Venue, Video, VideoNote, Voice, WebAppData, Widget,
};

/// Special entity inside a message text or caption (Bot API 9.5 adds `date_time`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity (`mention`, `hashtag`, `cashtag`, `bot_command`, `url`, `email`, `phone_number`,
    /// `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `blockquote`, `expandable_blockquote`,
    /// `code`, `pre`, `text_link`, `text_mention`, `custom_emoji`, `date_time`).
    #[serde(rename = "type")]
    pub entity_type: String,
    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: u32,
    /// Length of the entity in UTF-16 code units.
    pub length: u32,
    /// For `"text_link"` only, URL that will be opened after user taps on the text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// For `"text_mention"` only, the mentioned user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// For `"pre"` only, the programming language of the entity text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// For `"custom_emoji"` only, unique identifier of the custom emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// For `"date_time"` only (Bot API 9.5), Unix timestamp represented by the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unix_time: Option<i64>,
    /// For `"date_time"` only (Bot API 9.5), formatting specification string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<String>,
}

/// Information about the quote from the replied-to message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextQuote {
    /// Text of the quoted part of a message.
    pub text: String,
    /// Special entities that appear in the quote.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Approximate position of the quote in the original message in UTF-16 code units.
    pub position: u32,
    /// `true`, if the quote was chosen manually by the message sender.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}

/// Parameters describing the message being replied to (Bot API 9.2 adds `checklist_task_id`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat or `chat_id`.
    pub message_id: MessageId,
    /// If the message to be replied to is from a different chat, unique identifier for that chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Pass `true` if the message should be sent even if the specified message is not found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Quoted part of the message to be replied to (0-1024 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    /// Mode for parsing entities in the quote.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<super::ParseMode>,
    /// Special entities that appear in the quote.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,
    /// Position of the quote in the original message in UTF-16 code units.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<u32>,
    /// Identifier of the specific checklist task to be replied to (Bot API 9.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist_task_id: Option<i64>,
}

impl ReplyParameters {
    /// Creates reply parameters targeting `message_id` in the same chat.
    pub fn to_message(message_id: impl Into<MessageId>) -> Self {
        Self {
            message_id: message_id.into(),
            chat_id: None,
            allow_sending_without_reply: Some(true),
            quote: None,
            quote_parse_mode: None,
            quote_entities: None,
            quote_position: None,
            checklist_task_id: None,
        }
    }

    /// Replies to a specific checklist task inside the target message (Bot API 9.2).
    pub fn with_checklist_task(mut self, task_id: i64) -> Self {
        self.checklist_task_id = Some(task_id);
        self
    }
}

/// Parameters describing an ephemeral message visible only to a single user (Bot API 10.2–10.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EphemeralMessageParameters {
    /// Identifier of the user who will see the ephemeral message (optional in Bot API 10.4 when `callback_query_id` is passed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<UserId>,
    /// Identifier of the callback query that triggered the ephemeral reply (Bot API 10.2/10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_query_id: Option<String>,
    /// Pass `true` if the ephemeral message must replace the callback query's original message for the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace_callback_query_message: Option<bool>,
}

impl EphemeralMessageParameters {
    /// Targets a specific user with an ephemeral message.
    pub fn for_user(receiver_user_id: impl Into<UserId>) -> Self {
        Self {
            receiver_user_id: Some(receiver_user_id.into()),
            callback_query_id: None,
            replace_callback_query_message: None,
        }
    }

    /// Targets the sender of a callback query directly without needing `receiver_user_id` (Bot API 10.4).
    pub fn for_callback(callback_query_id: impl Into<String>) -> Self {
        Self {
            receiver_user_id: None,
            callback_query_id: Some(callback_query_id.into()),
            replace_callback_query_message: None,
        }
    }

    /// Sets `replace_callback_query_message`.
    pub fn replace_callback_message(mut self, replace: bool) -> Self {
        self.replace_callback_query_message = Some(replace);
        self
    }
}

/// Origin of a forwarded message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrigin {
    /// Originally sent by a known user.
    User {
        /// Date the message was sent originally in Unix time.
        date: i64,
        /// User that sent the message originally.
        sender_user: User,
    },
    /// Originally sent by a user with hidden forward privacy.
    HiddenUser {
        /// Date the message was sent originally.
        date: i64,
        /// Name of the user that sent the message originally.
        sender_user_name: String,
    },
    /// Originally sent on behalf of a chat.
    Chat {
        /// Date the message was sent originally.
        date: i64,
        /// Chat that sent the message originally.
        sender_chat: Chat,
        /// Original message author signature if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        author_signature: Option<String>,
    },
    /// Originally sent to a channel chat.
    Channel {
        /// Date the message was sent originally.
        date: i64,
        /// Channel chat to which the message was originally sent.
        chat: Chat,
        /// Unique message identifier inside the chat.
        message_id: MessageId,
        /// Signature of the original post author.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        author_signature: Option<String>,
    },
}

/// Information about a message that is being replied to, which may come from another chat (Bot API 10.4 adds `widget`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message.
    pub origin: MessageOrigin,
    /// Chat the original message belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    /// Options used for link preview generation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Message is an animation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    /// Message is an audio file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// Message is a document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    /// Message contains paid media.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
    /// Message is a photo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    /// Message is a video.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// Message is a video note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    /// Message is a voice note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    /// Message is a contact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Message is a dice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    /// Message is a venue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    /// Message is a location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Message is a poll.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Message contains an HTML widget (Bot API 10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget: Option<Widget>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Interactive checklist task (Bot API 9.1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task.
    pub id: i64,
    /// Text of the task.
    pub text: String,
    /// Special entities that appear in the task text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// User that completed the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_by_user: Option<User>,
    /// Point in time (Unix timestamp) when the task was completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<i64>,
}

/// Input checklist task used in `sendChecklist` / `editMessageChecklist` (Bot API 9.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task (> 0).
    pub id: i64,
    /// Text of the task (1-100 characters).
    pub text: String,
    /// Mode for parsing entities in the text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<super::ParseMode>,
    /// Special entities in the task text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

/// Input checklist payload (Bot API 9.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputChecklist {
    /// Title of the checklist (1-255 characters).
    pub title: String,
    /// Mode for parsing entities in the title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<super::ParseMode>,
    /// Special entities in the title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_entities: Option<Vec<MessageEntity>>,
    /// List of 1-30 tasks in the checklist.
    pub tasks: Vec<InputChecklistTask>,
    /// Pass `true` if other users can add tasks to the checklist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,
    /// Pass `true` if other users can mark tasks as done/not done.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}

/// Checklist attached to a message (Bot API 9.1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Checklist {
    /// Title of the checklist.
    pub title: String,
    /// Special entities in the title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_entities: Option<Vec<MessageEntity>>,
    /// List of tasks in the checklist.
    pub tasks: Vec<ChecklistTask>,
    /// `true`, if users other than the creator can add tasks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,
    /// `true`, if users other than the creator can mark tasks as done.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}

/// Service message payload indicating a managed bot was created (Bot API 9.6 / 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagedBotCreated {
    /// Information about the newly created managed bot.
    pub bot: User,
}

/// Service message payload when a chat is added to a community (Bot API 10.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityChatAdded {
    /// Community the chat was added to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub community: Option<Community>,
    /// Chat that was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Service message payload when a chat joins from a community (Bot API 10.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityChatJoined {
    /// Community associated with the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub community: Option<Community>,
    /// Chat that joined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Service message payload when a chat is removed from a community (Bot API 10.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityChatRemoved {
    /// Community the chat was removed from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub community: Option<Community>,
    /// Chat that was removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Response returned by [`answerGuestQuery`](crate::Bot::answer_guest_query) (Bot API 10.0).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentGuestMessage {
    /// Identifier of the sent inline/guest message for subsequent edits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Represents a Telegram message (up through Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat.
    pub message_id: MessageId,
    /// Unique identifier of a message thread or forum topic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Sender of the message (empty for channel posts).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,
    /// Sender of the message when sent on behalf of a chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,
    /// If the sender of the message boosted the chat, the number of boosts added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<u32>,
    /// Tag of the sender in the group (Bot API 9.5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_tag: Option<String>,
    /// The bot that actually sent the message on behalf of the business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<User>,
    /// Date the message was sent in Unix time.
    pub date: i64,
    /// Unique identifier of the business connection from which the message was received.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Chat the message belongs to.
    pub chat: Chat,
    /// Information about the original message for forwarded messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<Box<MessageOrigin>>,
    /// `true`, if the message is sent to a forum topic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post automatically forwarded to a discussion group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    /// Identifier of the specific checklist task replied to (Bot API 9.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_checklist_task_id: Option<i64>,
    /// Information about the message being replied to from another chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<Box<ExternalReplyInfo>>,
    /// Quoted part of the replied-to message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,
    /// Bot through which the message was sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// `true`, if the message was sent by an implicit action (away/greeting business message).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,
    /// Unique identifier of a media message group this message belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels, or custom title of anonymous admin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    /// Number of Telegram Stars paid by the sender to send the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_star_count: Option<u64>,
    /// For text messages, the actual UTF-8 text of the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Special entities like usernames, URLs, bot commands, `date_time` in `text`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Options used for link preview generation for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Unique identifier of the message effect added to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<String>,
    /// Message is an animation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<Animation>>,
    /// Message is an audio file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<Audio>>,
    /// Message is a general file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<Document>>,
    /// Message contains paid media.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Box<PaidMediaInfo>>,
    /// Message is a photo (available sizes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<Sticker>>,
    /// Message is a video.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<Video>>,
    /// Message is a video note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    /// Message is a voice message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    /// Caption for the animation, audio, document, paid media, photo, video, voice, or widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Special entities in `caption`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// `true`, if the caption must be shown above the message media.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// `true`, if the message media is covered by a spoiler animation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Message is a checklist (Bot API 9.1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checklist: Option<Box<Checklist>>,
    /// Message is a shared contact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Message is a dice with random value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    /// Message is a venue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<Box<Venue>>,
    /// Message is a shared location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Message is a native poll.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<Poll>>,
    /// Message is a structured Rich Message (Bot API 10.1+).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rich_message: Option<Box<RichMessage>>,
    /// Message contains an interactive HTML widget (Bot API 10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget: Option<Widget>,
    /// Identifier of the ephemeral message visible only to `receiver_user` (Bot API 10.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_message_id: Option<MessageId>,
    /// Target user for an ephemeral message (Bot API 10.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_user: Option<User>,
    /// User who invoked the bot in Guest Mode (Bot API 10.0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_bot_caller_user: Option<User>,
    /// Chat on behalf of which Guest Mode was invoked (Bot API 10.0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_bot_caller_chat: Option<Chat>,
    /// Query identifier used with `answerGuestQuery` (Bot API 10.0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_query_id: Option<String>,
    /// New members added to the group or supergroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    /// Member removed from the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,
    /// New chat title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    /// New chat photo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: chat photo was deleted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,
    /// Service message: group has been created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
    /// Service message: supergroup has been created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,
    /// Service message: channel has been created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,
    /// Specified message was pinned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    /// Message is an invoice for a payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<super::Invoice>,
    /// Message is a service message about a successful payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<super::SuccessfulPayment>,
    /// Message is a service message about a refunded payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<super::RefundedPayment>,
    /// Data sent by a Web App.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,
    /// Service message: a managed bot was created (Bot API 9.6/10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_bot_created: Option<ManagedBotCreated>,
    /// Service message: chat was added to a community (Bot API 10.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub community_chat_added: Option<CommunityChatAdded>,
    /// Service message: chat joined from a community (Bot API 10.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub community_chat_joined: Option<CommunityChatJoined>,
    /// Service message: chat was removed from a community (Bot API 10.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub community_chat_removed: Option<CommunityChatRemoved>,
    /// Inline keyboard attached to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Forward-compatible extra fields (including additional service messages).
    #[serde(flatten)]
    pub extra: JsonMap,
}

impl Message {
    /// Returns the text or caption of this message, if present.
    #[inline]
    pub fn text_or_caption(&self) -> Option<&str> {
        self.text.as_deref().or(self.caption.as_deref())
    }

    /// Returns `true` if this message is an ephemeral message (Bot API 10.2+).
    #[inline]
    pub const fn is_ephemeral(&self) -> bool {
        self.ephemeral_message_id.is_some()
    }

    /// Returns `true` if this message carries an HTML widget (Bot API 10.4).
    #[inline]
    pub const fn has_widget(&self) -> bool {
        self.widget.is_some()
    }

    /// Returns `true` if this message carries a structured Rich Message (Bot API 10.1+).
    #[inline]
    pub const fn has_rich_message(&self) -> bool {
        self.rich_message.is_some()
    }

    /// Parses a bot command (e.g. `"/start arg1 arg2"` or `"/start@MyBot arg1"`) from the message text.
    ///
    /// Returns `(command_without_slash_and_botname, raw_args)`.
    pub fn parse_command(&self) -> Option<(&str, &str)> {
        let text = self.text.as_deref()?.trim();
        let rest = text.strip_prefix('/')?;
        let (head, args) = match rest.find(char::is_whitespace) {
            Some(idx) => (&rest[..idx], rest[idx..].trim_start()),
            None => (rest, ""),
        };
        let cmd = head.split('@').next().unwrap_or(head);
        if cmd.is_empty() {
            None
        } else {
            Some((cmd, args))
        }
    }
}

/// Message inaccessible to the bot (for example, deleted or too old).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to.
    pub chat: Chat,
    /// Unique message identifier inside the chat.
    pub message_id: MessageId,
    /// Always `0` for inaccessible messages.
    pub date: i64,
}

/// Either a regular [`Message`] or an [`InaccessibleMessage`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    /// Accessible message (`date != 0`).
    Message(Box<Message>),
    /// Deleted/inaccessible message (`date == 0`).
    Inaccessible(InaccessibleMessage),
}

impl MaybeInaccessibleMessage {
    /// Returns the chat of the message regardless of accessibility.
    pub const fn chat(&self) -> &Chat {
        match self {
            Self::Message(m) => &m.chat,
            Self::Inaccessible(m) => &m.chat,
        }
    }

    /// Returns the message ID regardless of accessibility.
    pub const fn message_id(&self) -> MessageId {
        match self {
            Self::Message(m) => m.message_id,
            Self::Inaccessible(m) => m.message_id,
        }
    }

    /// Returns the inner [`Message`] if accessible.
    pub const fn as_message(&self) -> Option<&Message> {
        match self {
            Self::Message(m) => Some(m),
            Self::Inaccessible(_) => None,
        }
    }
}
