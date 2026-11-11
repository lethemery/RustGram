//! Telegram Business connection types.

use serde::{Deserialize, Serialize};

use super::{Chat, JsonMap, MessageId, User};

/// Rights granted to a bot connected to a Telegram Business account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BusinessBotRights {
    /// Allowed to reply to incoming messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_reply: Option<bool>,
    /// Allowed to mark incoming messages as read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_read_messages: Option<bool>,
    /// Allowed to delete outgoing messages sent by the bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_delete_sent_messages: Option<bool>,
    /// Allowed to delete all private messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_delete_all_messages: Option<bool>,
    /// Allowed to edit the business account's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_name: Option<bool>,
    /// Allowed to edit the business account's bio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_bio: Option<bool>,
    /// Allowed to edit the business account's profile photo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_profile_photo: Option<bool>,
    /// Allowed to edit the business account's username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_username: Option<bool>,
    /// Allowed to change gift privacy settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_change_gift_settings: Option<bool>,
    /// Allowed to view gifts and Stars balance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_view_gifts_and_stars: Option<bool>,
    /// Allowed to convert gifts to Telegram Stars.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_convert_gifts_to_stars: Option<bool>,
    /// Allowed to transfer and upgrade gifts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_transfer_and_upgrade_gifts: Option<bool>,
    /// Allowed to transfer Telegram Stars.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_transfer_stars: Option<bool>,
    /// Allowed to post, edit, and delete stories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_manage_stories: Option<bool>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Connection of the bot with a Telegram Business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection.
    pub id: String,
    /// Business account user that created the business connection.
    pub user: User,
    /// Identifier of a private chat with the user who created the connection.
    pub user_chat_id: i64,
    /// Date the connection was established in Unix time.
    pub date: i64,
    /// Rights of the business bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights: Option<BusinessBotRights>,
    /// `true`, if the connection is active.
    pub is_enabled: bool,
}

/// Messages deleted from a connected business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection.
    pub business_connection_id: String,
    /// Information about a chat in the business account.
    pub chat: Chat,
    /// List of identifiers of deleted messages in the chat of the business account.
    pub message_ids: Vec<MessageId>,
}
