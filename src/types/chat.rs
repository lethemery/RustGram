//! User, chat, community, permissions, member, and bot command types.

use serde::{Deserialize, Serialize};

use super::{JsonMap, Location, Message, ReactionType, UserId};

/// Represents a Telegram user or bot (Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: UserId,
    /// `true`, if this user is a bot.
    pub is_bot: bool,
    /// User's or bot's first name.
    pub first_name: String,
    /// User's or bot's last name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// User's or bot's username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// IETF language tag of the user's language.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// `true`, if this user is a Telegram Premium user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// `true`, if this user added the bot to the attachment menu.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    /// `true`, if the bot can be invited to groups (returned in `getMe`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// `true`, if privacy mode is disabled for the bot (returned in `getMe`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// `true`, if the bot supports inline queries (returned in `getMe`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
    /// `true`, if the bot can be connected to a Telegram Business account (returned in `getMe`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<bool>,
    /// `true`, if the bot has a main Web App (returned in `getMe`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,
    /// `true`, if the bot has forum topic mode enabled in private chats (Bot API 9.3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_topics_enabled: Option<bool>,
    /// `true`, if the bot allows users to create and delete topics in private chats (Bot API 9.4/10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allows_users_to_create_topics: Option<bool>,
    /// `true`, if the bot can manage secondary bots (Bot API 9.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_manage_bots: Option<bool>,
    /// `true`, if the bot supports guest mode queries (Bot API 10.0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supports_guest_queries: Option<bool>,
    /// `true`, if the bot supports join request screening queries (Bot API 10.1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supports_join_request_queries: Option<bool>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

impl User {
    /// Returns the user's full display name (`first_name [last_name]`).
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last) if !last.is_empty() => format!("{} {last}", self.first_name),
            _ => self.first_name.clone(),
        }
    }

    /// Returns `@username` if present, otherwise falls back to [`User::full_name`].
    pub fn mention_or_name(&self) -> String {
        match &self.username {
            Some(username) => format!("@{username}"),
            None => self.full_name(),
        }
    }
}

/// Type of a Telegram chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    /// One-on-one private chat.
    Private,
    /// Legacy basic group chat.
    Group,
    /// Supergroup chat.
    Supergroup,
    /// Broadcast channel.
    Channel,
}

/// Compact chat object embedded in messages and updates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,
    /// Type of the chat.
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    /// Title, for supergroups, channels, and group chats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Username, for private chats, supergroups, and channels if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// First name of the other party in a private chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// `true`, if the supergroup chat is a forum (has topics enabled).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// `true`, if the chat is a channel direct messages supergroup (Bot API 9.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

impl Chat {
    /// Returns `true` if this is a private chat.
    #[inline]
    pub const fn is_private(&self) -> bool {
        matches!(self.chat_type, ChatType::Private)
    }

    /// Returns `true` if this is a group or supergroup.
    #[inline]
    pub const fn is_group(&self) -> bool {
        matches!(self.chat_type, ChatType::Group | ChatType::Supergroup)
    }

    /// Returns `true` if this is a channel.
    #[inline]
    pub const fn is_channel(&self) -> bool {
        matches!(self.chat_type, ChatType::Channel)
    }
}

/// Represents a Telegram Community (Bot API 10.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Community {
    /// Community identifier if provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Community title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Community public username if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Profile photo identifiers of a chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo.
    pub small_file_id: String,
    /// Unique file identifier of small chat photo.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo.
    pub big_file_id: String,
    /// Unique file identifier of big chat photo.
    pub big_file_unique_id: String,
}

/// User's birthdate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Birthdate {
    /// Day of the user's birth (1-31).
    pub day: u8,
    /// Month of the user's birth (1-12).
    pub month: u8,
    /// Year of the user's birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

/// Business intro message shown in an empty private chat with a business account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessIntro {
    /// Title text of the business intro.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Message text of the business intro.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Sticker of the business intro.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker: Option<serde_json::Value>,
}

/// Address and geographic location of a Telegram Business.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessLocation {
    /// Address of the business.
    pub address: String,
    /// Location of the business.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// Single opening-hours interval in minutes from Monday 00:00.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessOpeningHoursInterval {
    /// Minute's sequence number in a week, starting on Monday.
    pub opening_minute: u32,
    /// Closing minute's sequence number in a week.
    pub closing_minute: u32,
}

/// Opening hours configuration of a Telegram Business.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined.
    pub time_zone_name: String,
    /// List of time intervals describing business opening hours.
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}

/// Geographic location linked to a supergroup chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected.
    pub location: Location,
    /// Location address (1-64 characters).
    pub address: String,
}

/// Accepted gift categories for a user or chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AcceptedGiftTypes {
    /// `true`, if unlimited regular gifts are accepted.
    #[serde(default)]
    pub unlimited_gifts: bool,
    /// `true`, if limited regular gifts are accepted.
    #[serde(default)]
    pub limited_gifts: bool,
    /// `true`, if unique gifts or gifts that can be upgraded to unique are accepted.
    #[serde(default)]
    pub unique_gifts: bool,
    /// `true`, if a Telegram Premium subscription is accepted.
    #[serde(default)]
    pub premium_subscription: bool,
}

/// Detailed chat information returned by [`getChat`](crate::Bot::get_chat).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfo {
    /// Unique identifier for this chat.
    pub id: i64,
    /// Type of the chat.
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    /// Title, for supergroups, channels, and group chats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Username, for private chats, supergroups, and channels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// First name of the other party in a private chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// `true`, if the supergroup chat is a forum.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// `true`, if the chat is a channel direct messages supergroup (Bot API 9.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
    /// Identifier of the accent color for the chat name and backgrounds.
    pub accent_color_id: i32,
    /// Maximum number of reactions that can be set on a message in the chat.
    pub max_reaction_count: i32,
    /// Chat photo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// List of all active chat usernames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    /// For private chats, the date of birth of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Birthdate>,
    /// For private chats with business accounts, the intro of the business.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,
    /// For private chats with business accounts, the location of the business.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_location: Option<BusinessLocation>,
    /// For private chats with business accounts, the opening hours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<BusinessOpeningHours>,
    /// For private chats, the personal channel of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Box<Chat>>,
    /// For direct messages chats, the parent channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_chat: Option<Box<Chat>>,
    /// List of available reactions allowed in the chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,
    /// Custom emoji identifier of the emoji chosen by the chat for the reply header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    /// Identifier of the accent color for the chat's profile background.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<i32>,
    /// Custom emoji identifier of the emoji chosen for the chat's profile background.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,
    /// Custom emoji identifier of the emoji status of the chat or user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status as a Unix timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
    /// Bio of the other party in a private chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// `true`, if privacy settings restrict forwarding links in private chats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// `true`, if sending voice and video note messages is restricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// `true`, if users need to join the supergroup before sending messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    /// `true`, if joining requires approval by supergroup administrators.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    /// Description, for groups, supergroups, and channel chats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Primary invite link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// The most recent pinned message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    /// Types of gifts accepted by the chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_gift_types: Option<AcceptedGiftTypes>,
    /// `true`, if paid media messages can be sent or forwarded to the channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,
    /// Minimum allowed delay between consecutive messages in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i32>,
    /// Minimum boost count required to ignore slow mode and restrictions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<i32>,
    /// Message auto-delete time in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i32>,
    /// `true`, if aggressive anti-spam checks are enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// `true`, if non-administrators cannot see the list of chat members.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// `true`, if messages from the chat cannot be forwarded to other chats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// `true`, if new chat members have access to messages sent before joining.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    /// Name of the group sticker set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// `true`, if the bot can change the group sticker set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    /// Name of the group custom emoji sticker set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,
    /// Identifier of the linked discussion/channel chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,
    /// Location to which the supergroup is connected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
    /// Community linked to the chat (Bot API 10.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub community: Option<Community>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Actions allowed for non-administrator users in a chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChatPermissions {
    /// Allowed to send text messages, contacts, giveaways, invoices, locations, and venues.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// Allowed to send audios.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    /// Allowed to send documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    /// Allowed to send photos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    /// Allowed to send videos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    /// Allowed to send video notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    /// Allowed to send voice notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    /// Allowed to send polls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// Allowed to send animations, games, stickers, and dice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// Allowed to add web page previews to messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// Allowed to change chat title, photo, and other settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Allowed to invite new users to the chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Allowed to pin messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Allowed to create forum topics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Allowed to edit their own member tag (Bot API 9.5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_tag: Option<bool>,
    /// Allowed to react to messages (Bot API 10.0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_react_to_messages: Option<bool>,
}

/// Rights of a chat administrator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChatAdministratorRights {
    /// `true`, if the user's presence in the chat is hidden.
    pub is_anonymous: bool,
    /// `true`, if the administrator can access chat event logs, stats, and boosts.
    pub can_manage_chat: bool,
    /// `true`, if the administrator can delete messages of other users.
    pub can_delete_messages: bool,
    /// `true`, if the administrator can manage video chats.
    pub can_manage_video_chats: bool,
    /// `true`, if the administrator can restrict, ban, or unban chat members.
    pub can_restrict_members: bool,
    /// `true`, if the administrator can add new administrators.
    pub can_promote_members: bool,
    /// `true`, if the administrator can change chat title, photo, and other settings.
    pub can_change_info: bool,
    /// `true`, if the administrator can invite new users to the chat.
    pub can_invite_users: bool,
    /// `true`, if the administrator can post stories to the chat.
    pub can_post_stories: bool,
    /// `true`, if the administrator can edit stories posted by other users.
    pub can_edit_stories: bool,
    /// `true`, if the administrator can delete stories posted by other users.
    pub can_delete_stories: bool,
    /// `true`, if the administrator can post messages in the channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// `true`, if the administrator can edit messages of other users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// `true`, if the administrator can pin messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// `true`, if the administrator can manage forum topics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// `true`, if the administrator can manage direct messages of a channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
    /// `true`, if the administrator can manage chat member tags (Bot API 9.5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_manage_tags: Option<bool>,
}

/// Information about one member of a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ChatMember {
    /// Chat owner (`status = "creator"`).
    Creator {
        /// Information about the user.
        user: User,
        /// `true`, if the user's presence in the chat is hidden.
        is_anonymous: bool,
        /// Custom title for this user.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        custom_title: Option<String>,
    },
    /// Chat administrator (`status = "administrator"`).
    Administrator {
        /// Information about the user.
        user: User,
        /// `true`, if the bot is allowed to edit administrator privileges of that user.
        can_be_edited: bool,
        /// `true`, if the user's presence is hidden.
        is_anonymous: bool,
        /// `true`, if the administrator can manage the chat.
        can_manage_chat: bool,
        /// `true`, if the administrator can delete messages.
        can_delete_messages: bool,
        /// `true`, if the administrator can manage video chats.
        can_manage_video_chats: bool,
        /// `true`, if the administrator can restrict members.
        can_restrict_members: bool,
        /// `true`, if the administrator can promote members.
        can_promote_members: bool,
        /// `true`, if the administrator can change chat info.
        can_change_info: bool,
        /// `true`, if the administrator can invite users.
        can_invite_users: bool,
        /// `true`, if the administrator can post stories.
        can_post_stories: bool,
        /// `true`, if the administrator can edit stories.
        can_edit_stories: bool,
        /// `true`, if the administrator can delete stories.
        can_delete_stories: bool,
        /// `true`, if the administrator can post messages in channels.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        can_post_messages: Option<bool>,
        /// `true`, if the administrator can edit messages in channels.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        can_edit_messages: Option<bool>,
        /// `true`, if the administrator can pin messages.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        can_pin_messages: Option<bool>,
        /// `true`, if the administrator can manage forum topics.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        can_manage_topics: Option<bool>,
        /// `true`, if the administrator can manage channel direct messages.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        can_manage_direct_messages: Option<bool>,
        /// `true`, if the administrator can manage member tags (Bot API 9.5).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        can_manage_tags: Option<bool>,
        /// Custom title for this user.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        custom_title: Option<String>,
    },
    /// Regular chat member (`status = "member"`).
    Member {
        /// Information about the user.
        user: User,
        /// Custom member tag (Bot API 9.5).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
        /// Date when the subscription will expire (Unix timestamp).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        until_date: Option<i64>,
    },
    /// Restricted chat member (`status = "restricted"`).
    Restricted {
        /// Information about the user.
        user: User,
        /// Custom member tag (Bot API 9.5).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
        /// `true`, if the user is a member of the chat at the moment of the request.
        is_member: bool,
        /// Permissions granted to the restricted member.
        #[serde(flatten)]
        permissions: ChatPermissions,
        /// Date when restrictions will be lifted (Unix timestamp).
        until_date: i64,
    },
    /// User who left the chat (`status = "left"`).
    Left {
        /// Information about the user.
        user: User,
    },
    /// Banned chat member (`status = "kicked"`).
    Kicked {
        /// Information about the user.
        user: User,
        /// Date when restrictions will be lifted (Unix timestamp).
        until_date: i64,
    },
}

impl ChatMember {
    /// Returns the underlying [`User`] for any [`ChatMember`] variant.
    pub const fn user(&self) -> &User {
        match self {
            Self::Creator { user, .. }
            | Self::Administrator { user, .. }
            | Self::Member { user, .. }
            | Self::Restricted { user, .. }
            | Self::Left { user }
            | Self::Kicked { user, .. } => user,
        }
    }
}

/// Invite link for a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link URL.
    pub invite_link: String,
    /// Creator of the link.
    pub creator: User,
    /// `true`, if users joining via the link need approval.
    pub creates_join_request: bool,
    /// `true`, if the link is primary.
    pub is_primary: bool,
    /// `true`, if the link is revoked.
    pub is_revoked: bool,
    /// Invite link name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Maximum number of simultaneous chat members joined via this link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
    /// Number of pending join requests created using this link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i32>,
    /// Subscription period in seconds for subscription links.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i32>,
    /// Telegram Stars price per subscription period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_price: Option<i32>,
}

/// Changes in the status of a chat member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to.
    pub chat: Chat,
    /// Performer of the action that resulted in the change.
    pub from: User,
    /// Date the change was done in Unix time.
    pub date: i64,
    /// Previous information about the chat member.
    pub old_chat_member: ChatMember,
    /// New information about the chat member.
    pub new_chat_member: ChatMember,
    /// Chat invite link used by the user to join.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    /// `true`, if the user joined via a chat folder invite link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
    /// `true`, if the user joined after sending a direct join request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
}

/// Join request sent to a chat (Bot API 10.1 includes `query_id`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent.
    pub chat: Chat,
    /// User that sent the join request.
    pub from: User,
    /// Identifier of a private chat with the user who sent the join request.
    pub user_chat_id: i64,
    /// Date the request was sent in Unix time.
    pub date: i64,
    /// Bio of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Chat invite link used by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    /// Unique identifier of the join request query (Bot API 10.1), usable with `answerChatJoinRequestQuery`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
}

/// Source of a chat boost.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ChatBoostSource {
    /// Obtained by subscribing to Telegram Premium or gifting Telegram Premium.
    Premium {
        /// User that boosted the chat.
        user: User,
    },
    /// Obtained by creation of Telegram Premium gift codes.
    GiftCode {
        /// User for which the gift code was created.
        user: User,
    },
    /// Obtained by creation of a Telegram Premium giveaway.
    Giveaway {
        /// Identifier of a message in the chat with the giveaway.
        giveaway_message_id: i64,
        /// User that won the prize in the giveaway if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        user: Option<User>,
        /// `true`, if the giveaway was completed without any winners.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        is_unclaimed: Option<bool>,
    },
}

/// Information about a boost added to a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatBoost {
    /// Unique identifier of the boost.
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the chat was boosted.
    pub add_date: i64,
    /// Point in time (Unix timestamp) when the boost expires.
    pub expiration_date: i64,
    /// Source of the added boost.
    pub source: ChatBoostSource,
}

/// Update representing a boost added to or changed in a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted.
    pub chat: Chat,
    /// Information about the chat boost.
    pub boost: ChatBoost,
}

/// Update representing a boost removed from a chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostRemoved {
    /// Chat which was boosted.
    pub chat: Chat,
    /// Unique identifier of the boost.
    pub boost_id: String,
    /// Point in time (Unix timestamp) when the boost was removed.
    pub remove_date: i64,
    /// Source of the removed boost.
    pub source: ChatBoostSource,
}

/// Forum topic metadata returned by [`createForumTopic`](crate::Bot::create_forum_topic).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic.
    pub message_thread_id: i64,
    /// Name of the topic.
    pub name: String,
    /// Color of the topic icon in RGB format.
    pub icon_color: i32,
    /// Unique identifier of the custom emoji shown as the topic icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Represents a bot command (Bot API 10.2 adds `is_ephemeral`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotCommand {
    /// Text of the command (1-32 characters, lowercase letters, digits, underscores).
    pub command: String,
    /// Description of the command (1-256 characters).
    pub description: String,
    /// `true`, if replies to this command should be ephemeral (Bot API 10.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_ephemeral: Option<bool>,
}

impl BotCommand {
    /// Creates a new [`BotCommand`].
    pub fn new(command: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
            is_ephemeral: None,
        }
    }

    /// Marks the command as ephemeral (Bot API 10.2).
    pub fn ephemeral(mut self, is_ephemeral: bool) -> Self {
        self.is_ephemeral = Some(is_ephemeral);
        self
    }
}

/// Scope to which bot commands apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BotCommandScope {
    /// Default scope for all users.
    Default,
    /// All private chats.
    AllPrivateChats,
    /// All group and supergroup chats.
    AllGroupChats,
    /// All group and supergroup chat administrators.
    AllChatAdministrators,
    /// Specific chat.
    Chat {
        /// Target chat identifier.
        chat_id: super::ChatId,
    },
    /// Administrators of a specific chat.
    ChatAdministrators {
        /// Target chat identifier.
        chat_id: super::ChatId,
    },
    /// Specific member in a specific group or supergroup chat.
    ChatMember {
        /// Target chat identifier.
        chat_id: super::ChatId,
        /// Target user identifier.
        user_id: UserId,
    },
}

/// Bot's menu button in a private chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    /// Displays the list of bot commands.
    Commands,
    /// Launches a Web App.
    WebApp {
        /// Text on the button.
        text: String,
        /// Description of the Web App launched when pressed.
        web_app: super::WebAppInfo,
    },
    /// Default behavior configured by the bot.
    Default,
}

/// Bot display name returned by `getMyName`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotName {
    /// The bot's name.
    pub name: String,
}

/// Bot description returned by `getMyDescription`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotDescription {
    /// The bot's description.
    pub description: String,
}

/// Bot short description returned by `getMyShortDescription`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotShortDescription {
    /// The bot's short description.
    pub short_description: String,
}

/// Access settings for a managed bot (Bot API 9.6 / 10.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BotAccessSettings {
    /// Whether public access to the managed bot is restricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_restricted: Option<bool>,
    /// Identifiers of users permitted to interact with the managed bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_user_ids: Option<Vec<UserId>>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}
