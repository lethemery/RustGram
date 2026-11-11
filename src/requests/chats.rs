//! Chat administration, member tags (Bot API 9.5), forum topics, invite links, join request queries (Bot API 10.1), and bot configuration requests.

use serde::Serialize;

use crate::client::Bot;
use crate::types::{
    BotCommand, BotCommandScope, BotDescription, BotName, BotShortDescription,
    ChatAdministratorRights, ChatFullInfo, ChatId, ChatInviteLink, ChatMember, ChatPermissions,
    ForumTopic, MenuButton, MessageId, UserId, UserProfilePhotos,
};

// Local alias kept tidy for `setMyDefaultAdministratorRights`
#[doc(hidden)]
pub type BotAdministratorRightsExt = ChatAdministratorRights;

/// Payload for chat-only lookups (`getChat`, `getChatAdministrators`, `getChatMemberCount`, `leaveChat`, `deleteChatPhoto`, `unpinAllChatMessages`, `exportChatInviteLink`).
#[derive(Debug, Clone, Serialize)]
pub struct ChatOnlyPayload {
    pub chat_id: ChatId,
}

macro_rules! simple_chat_request {
    ($name:ident, $method:literal, $output:ty, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone)]
        pub struct $name {
            bot: Bot,
            pub payload: ChatOnlyPayload,
        }

        impl $name {
            /// Creates a new request targeting `chat_id`.
            pub fn new(bot: Bot, chat_id: impl Into<ChatId>) -> Self {
                Self {
                    bot,
                    payload: ChatOnlyPayload {
                        chat_id: chat_id.into(),
                    },
                }
            }
        }

        crate::impl_json_request!($name, $method, $output);
    };
}

simple_chat_request!(
    GetChat,
    "getChat",
    ChatFullInfo,
    "Request builder for `getChat`."
);
simple_chat_request!(
    GetChatAdministrators,
    "getChatAdministrators",
    Vec<ChatMember>,
    "Request builder for `getChatAdministrators`."
);
simple_chat_request!(
    GetChatMemberCount,
    "getChatMemberCount",
    u64,
    "Request builder for `getChatMemberCount`."
);
simple_chat_request!(
    LeaveChat,
    "leaveChat",
    bool,
    "Request builder for `leaveChat`."
);
simple_chat_request!(
    DeleteChatPhoto,
    "deleteChatPhoto",
    bool,
    "Request builder for `deleteChatPhoto`."
);
simple_chat_request!(
    UnpinAllChatMessages,
    "unpinAllChatMessages",
    bool,
    "Request builder for `unpinAllChatMessages`."
);
simple_chat_request!(
    ExportChatInviteLink,
    "exportChatInviteLink",
    String,
    "Request builder for `exportChatInviteLink`."
);

/// Payload for `(chat_id, user_id)` requests.
#[derive(Debug, Clone, Serialize)]
pub struct ChatUserPayload {
    pub chat_id: ChatId,
    pub user_id: UserId,
}

/// Request builder for `getChatMember`.
#[derive(Debug, Clone)]
pub struct GetChatMember {
    bot: Bot,
    pub payload: ChatUserPayload,
}

impl GetChatMember {
    /// Creates a new `getChatMember` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, user_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: ChatUserPayload {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
            },
        }
    }
}

crate::impl_json_request!(GetChatMember, "getChatMember", ChatMember);

/// Request builder for `approveChatJoinRequest`.
#[derive(Debug, Clone)]
pub struct ApproveChatJoinRequest {
    bot: Bot,
    pub payload: ChatUserPayload,
}

impl ApproveChatJoinRequest {
    /// Creates a new `approveChatJoinRequest` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, user_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: ChatUserPayload {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
            },
        }
    }
}

crate::impl_json_request!(ApproveChatJoinRequest, "approveChatJoinRequest", bool);

/// Request builder for `declineChatJoinRequest`.
#[derive(Debug, Clone)]
pub struct DeclineChatJoinRequest {
    bot: Bot,
    pub payload: ChatUserPayload,
}

impl DeclineChatJoinRequest {
    /// Creates a new `declineChatJoinRequest` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, user_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: ChatUserPayload {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
            },
        }
    }
}

crate::impl_json_request!(DeclineChatJoinRequest, "declineChatJoinRequest", bool);

/// Payload for Bot API 10.1 `answerChatJoinRequestQuery`.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerChatJoinRequestQueryPayload {
    pub chat_join_request_query_id: String,
    pub status: String,
}

/// Request builder for Bot API 10.1 `answerChatJoinRequestQuery`.
#[derive(Debug, Clone)]
pub struct AnswerChatJoinRequestQuery {
    bot: Bot,
    pub payload: AnswerChatJoinRequestQueryPayload,
}

impl AnswerChatJoinRequestQuery {
    /// Creates a new `answerChatJoinRequestQuery` request (`status`: `"approve"` or `"decline"`).
    pub fn new(
        bot: Bot,
        chat_join_request_query_id: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            payload: AnswerChatJoinRequestQueryPayload {
                chat_join_request_query_id: chat_join_request_query_id.into(),
                status: status.into(),
            },
        }
    }
}

crate::impl_json_request!(
    AnswerChatJoinRequestQuery,
    "answerChatJoinRequestQuery",
    bool
);

/// Payload for `banChatMember`.
#[derive(Debug, Clone, Serialize)]
pub struct BanChatMemberPayload {
    pub chat_id: ChatId,
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

/// Request builder for `banChatMember`.
#[derive(Debug, Clone)]
pub struct BanChatMember {
    bot: Bot,
    pub payload: BanChatMemberPayload,
}

impl BanChatMember {
    /// Creates a new `banChatMember` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, user_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: BanChatMemberPayload {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
                until_date: None,
                revoke_messages: None,
            },
        }
    }

    crate::request_setters! {
        until_date(ts: i64) -> until_date,
        revoke_messages(revoke: bool) -> revoke_messages,
    }
}

crate::impl_json_request!(BanChatMember, "banChatMember", bool);

/// Payload for `unbanChatMember`.
#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatMemberPayload {
    pub chat_id: ChatId,
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

/// Request builder for `unbanChatMember`.
#[derive(Debug, Clone)]
pub struct UnbanChatMember {
    bot: Bot,
    pub payload: UnbanChatMemberPayload,
}

impl UnbanChatMember {
    /// Creates a new `unbanChatMember` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, user_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: UnbanChatMemberPayload {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
                only_if_banned: None,
            },
        }
    }

    crate::request_setters! {
        only_if_banned(only_if_banned: bool) -> only_if_banned,
    }
}

crate::impl_json_request!(UnbanChatMember, "unbanChatMember", bool);

/// Payload for `restrictChatMember`.
#[derive(Debug, Clone, Serialize)]
pub struct RestrictChatMemberPayload {
    pub chat_id: ChatId,
    pub user_id: UserId,
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

/// Request builder for `restrictChatMember`.
#[derive(Debug, Clone)]
pub struct RestrictChatMember {
    bot: Bot,
    pub payload: RestrictChatMemberPayload,
}

impl RestrictChatMember {
    /// Creates a new `restrictChatMember` request.
    pub fn new(
        bot: Bot,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<UserId>,
        permissions: ChatPermissions,
    ) -> Self {
        Self {
            bot,
            payload: RestrictChatMemberPayload {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
                permissions,
                use_independent_chat_permissions: None,
                until_date: None,
            },
        }
    }

    crate::request_setters! {
        use_independent_chat_permissions(independent: bool) -> use_independent_chat_permissions,
        until_date(ts: i64) -> until_date,
    }
}

crate::impl_json_request!(RestrictChatMember, "restrictChatMember", bool);

/// Payload for Bot API 9.5 `setChatMemberTag`.
#[derive(Debug, Clone, Serialize)]
pub struct SetChatMemberTagPayload {
    pub chat_id: ChatId,
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Request builder for Bot API 9.5 `setChatMemberTag`.
#[derive(Debug, Clone)]
pub struct SetChatMemberTag {
    bot: Bot,
    pub payload: SetChatMemberTagPayload,
}

impl SetChatMemberTag {
    /// Creates a new `setChatMemberTag` request (omit `tag` to remove the member's tag).
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, user_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: SetChatMemberTagPayload {
                chat_id: chat_id.into(),
                user_id: user_id.into(),
                tag: None,
            },
        }
    }

    crate::request_setters! {
        tag(tag: impl Into<String>) -> tag; into,
    }
}

crate::impl_json_request!(SetChatMemberTag, "setChatMemberTag", bool);

/// Payload for `setChatPermissions`.
#[derive(Debug, Clone, Serialize)]
pub struct SetChatPermissionsPayload {
    pub chat_id: ChatId,
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
}

/// Request builder for `setChatPermissions`.
#[derive(Debug, Clone)]
pub struct SetChatPermissions {
    bot: Bot,
    pub payload: SetChatPermissionsPayload,
}

impl SetChatPermissions {
    /// Creates a new `setChatPermissions` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, permissions: ChatPermissions) -> Self {
        Self {
            bot,
            payload: SetChatPermissionsPayload {
                chat_id: chat_id.into(),
                permissions,
                use_independent_chat_permissions: None,
            },
        }
    }

    crate::request_setters! {
        use_independent_chat_permissions(independent: bool) -> use_independent_chat_permissions,
    }
}

crate::impl_json_request!(SetChatPermissions, "setChatPermissions", bool);

/// Payload for `createChatInviteLink`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateChatInviteLinkPayload {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

/// Request builder for `createChatInviteLink`.
#[derive(Debug, Clone)]
pub struct CreateChatInviteLink {
    bot: Bot,
    pub payload: CreateChatInviteLinkPayload,
}

impl CreateChatInviteLink {
    /// Creates a new `createChatInviteLink` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            payload: CreateChatInviteLinkPayload {
                chat_id: chat_id.into(),
                name: None,
                expire_date: None,
                member_limit: None,
                creates_join_request: None,
            },
        }
    }

    crate::request_setters! {
        name(name: impl Into<String>) -> name; into,
        expire_date(ts: i64) -> expire_date,
        member_limit(limit: u32) -> member_limit,
        creates_join_request(req: bool) -> creates_join_request,
    }
}

crate::impl_json_request!(CreateChatInviteLink, "createChatInviteLink", ChatInviteLink);

/// Payload for `revokeChatInviteLink`.
#[derive(Debug, Clone, Serialize)]
pub struct RevokeChatInviteLinkPayload {
    pub chat_id: ChatId,
    pub invite_link: String,
}

/// Request builder for `revokeChatInviteLink`.
#[derive(Debug, Clone)]
pub struct RevokeChatInviteLink {
    bot: Bot,
    pub payload: RevokeChatInviteLinkPayload,
}

impl RevokeChatInviteLink {
    /// Creates a new `revokeChatInviteLink` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, invite_link: impl Into<String>) -> Self {
        Self {
            bot,
            payload: RevokeChatInviteLinkPayload {
                chat_id: chat_id.into(),
                invite_link: invite_link.into(),
            },
        }
    }
}

crate::impl_json_request!(RevokeChatInviteLink, "revokeChatInviteLink", ChatInviteLink);

/// Payload for `createForumTopic` (works in supergroup forums and private chats in Bot API 9.4/10.4).
#[derive(Debug, Clone, Serialize)]
pub struct CreateForumTopicPayload {
    pub chat_id: ChatId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Request builder for `createForumTopic`.
#[derive(Debug, Clone)]
pub struct CreateForumTopic {
    bot: Bot,
    pub payload: CreateForumTopicPayload,
}

impl CreateForumTopic {
    /// Creates a new `createForumTopic` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, name: impl Into<String>) -> Self {
        Self {
            bot,
            payload: CreateForumTopicPayload {
                chat_id: chat_id.into(),
                name: name.into(),
                icon_color: None,
                icon_custom_emoji_id: None,
            },
        }
    }

    crate::request_setters! {
        icon_color(color: i32) -> icon_color,
        icon_custom_emoji_id(id: impl Into<String>) -> icon_custom_emoji_id; into,
    }
}

crate::impl_json_request!(CreateForumTopic, "createForumTopic", ForumTopic);

/// Payload for `pinChatMessage`.
#[derive(Debug, Clone, Serialize)]
pub struct PinChatMessagePayload {
    pub chat_id: ChatId,
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

/// Request builder for `pinChatMessage`.
#[derive(Debug, Clone)]
pub struct PinChatMessage {
    bot: Bot,
    pub payload: PinChatMessagePayload,
}

impl PinChatMessage {
    /// Creates a new `pinChatMessage` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, message_id: impl Into<MessageId>) -> Self {
        Self {
            bot,
            payload: PinChatMessagePayload {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
                business_connection_id: None,
                disable_notification: None,
            },
        }
    }

    crate::request_setters! {
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
        disable_notification(disable: bool) -> disable_notification,
    }
}

crate::impl_json_request!(PinChatMessage, "pinChatMessage", bool);

/// Payload for `unpinChatMessage`.
#[derive(Debug, Clone, Serialize)]
pub struct UnpinChatMessagePayload {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
}

/// Request builder for `unpinChatMessage`.
#[derive(Debug, Clone)]
pub struct UnpinChatMessage {
    bot: Bot,
    pub payload: UnpinChatMessagePayload,
}

impl UnpinChatMessage {
    /// Creates a new `unpinChatMessage` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            payload: UnpinChatMessagePayload {
                chat_id: chat_id.into(),
                message_id: None,
                business_connection_id: None,
            },
        }
    }

    crate::request_setters! {
        message_id(id: impl Into<MessageId>) -> message_id; into,
        business_connection_id(id: impl Into<String>) -> business_connection_id; into,
    }
}

crate::impl_json_request!(UnpinChatMessage, "unpinChatMessage", bool);

/// Payload for `setChatTitle` / `setChatDescription`.
#[derive(Debug, Clone, Serialize)]
pub struct SetChatTitlePayload {
    pub chat_id: ChatId,
    pub title: String,
}

/// Request builder for `setChatTitle`.
#[derive(Debug, Clone)]
pub struct SetChatTitle {
    bot: Bot,
    pub payload: SetChatTitlePayload,
}

impl SetChatTitle {
    /// Creates a new `setChatTitle` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>, title: impl Into<String>) -> Self {
        Self {
            bot,
            payload: SetChatTitlePayload {
                chat_id: chat_id.into(),
                title: title.into(),
            },
        }
    }
}

crate::impl_json_request!(SetChatTitle, "setChatTitle", bool);

/// Payload for `setChatDescription`.
#[derive(Debug, Clone, Serialize)]
pub struct SetChatDescriptionPayload {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request builder for `setChatDescription`.
#[derive(Debug, Clone)]
pub struct SetChatDescription {
    bot: Bot,
    pub payload: SetChatDescriptionPayload,
}

impl SetChatDescription {
    /// Creates a new `setChatDescription` request.
    pub fn new(bot: Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            payload: SetChatDescriptionPayload {
                chat_id: chat_id.into(),
                description: None,
            },
        }
    }

    crate::request_setters! {
        description(desc: impl Into<String>) -> description; into,
    }
}

crate::impl_json_request!(SetChatDescription, "setChatDescription", bool);

/// Payload for `getUserProfilePhotos`.
#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfilePhotosPayload {
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Request builder for `getUserProfilePhotos`.
#[derive(Debug, Clone)]
pub struct GetUserProfilePhotos {
    bot: Bot,
    pub payload: GetUserProfilePhotosPayload,
}

impl GetUserProfilePhotos {
    /// Creates a new `getUserProfilePhotos` request.
    pub fn new(bot: Bot, user_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: GetUserProfilePhotosPayload {
                user_id: user_id.into(),
                offset: None,
                limit: None,
            },
        }
    }

    crate::request_setters! {
        offset(offset: u32) -> offset,
        limit(limit: u32) -> limit,
    }
}

crate::impl_json_request!(
    GetUserProfilePhotos,
    "getUserProfilePhotos",
    UserProfilePhotos
);

/// Payload for `setMyCommands`.
#[derive(Debug, Clone, Serialize)]
pub struct SetMyCommandsPayload {
    pub commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Request builder for `setMyCommands`.
#[derive(Debug, Clone)]
pub struct SetMyCommands {
    bot: Bot,
    pub payload: SetMyCommandsPayload,
}

impl SetMyCommands {
    /// Creates a new `setMyCommands` request.
    pub fn new(bot: Bot, commands: Vec<BotCommand>) -> Self {
        Self {
            bot,
            payload: SetMyCommandsPayload {
                commands,
                scope: None,
                language_code: None,
            },
        }
    }

    crate::request_setters! {
        scope(scope: BotCommandScope) -> scope,
        language_code(lang: impl Into<String>) -> language_code; into,
    }
}

crate::impl_json_request!(SetMyCommands, "setMyCommands", bool);

/// Payload for `getMyCommands` / `deleteMyCommands`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct CommandsScopePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Request builder for `getMyCommands`.
#[derive(Debug, Clone)]
pub struct GetMyCommands {
    bot: Bot,
    pub payload: CommandsScopePayload,
}

impl GetMyCommands {
    /// Creates a new `getMyCommands` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: CommandsScopePayload::default(),
        }
    }

    crate::request_setters! {
        scope(scope: BotCommandScope) -> scope,
        language_code(lang: impl Into<String>) -> language_code; into,
    }
}

crate::impl_json_request!(GetMyCommands, "getMyCommands", Vec<BotCommand>);

/// Request builder for `deleteMyCommands`.
#[derive(Debug, Clone)]
pub struct DeleteMyCommands {
    bot: Bot,
    pub payload: CommandsScopePayload,
}

impl DeleteMyCommands {
    /// Creates a new `deleteMyCommands` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: CommandsScopePayload::default(),
        }
    }

    crate::request_setters! {
        scope(scope: BotCommandScope) -> scope,
        language_code(lang: impl Into<String>) -> language_code; into,
    }
}

crate::impl_json_request!(DeleteMyCommands, "deleteMyCommands", bool);

/// Payload for `getMyName` / `getMyDescription` / `getMyShortDescription`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct LanguageCodePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Request builder for `getMyName`.
#[derive(Debug, Clone)]
pub struct GetMyName {
    bot: Bot,
    pub payload: LanguageCodePayload,
}

impl GetMyName {
    /// Creates a new `getMyName` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: LanguageCodePayload::default(),
        }
    }

    crate::request_setters! {
        language_code(lang: impl Into<String>) -> language_code; into,
    }
}

crate::impl_json_request!(GetMyName, "getMyName", BotName);

/// Payload for `setMyName`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct SetMyNamePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// Request builder for `setMyName`.
#[derive(Debug, Clone)]
pub struct SetMyName {
    bot: Bot,
    pub payload: SetMyNamePayload,
}

impl SetMyName {
    /// Creates a new `setMyName` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: SetMyNamePayload::default(),
        }
    }

    crate::request_setters! {
        name(name: impl Into<String>) -> name; into,
        language_code(lang: impl Into<String>) -> language_code; into,
    }
}

crate::impl_json_request!(SetMyName, "setMyName", bool);

/// Request builder for `getMyDescription`.
#[derive(Debug, Clone)]
pub struct GetMyDescription {
    bot: Bot,
    pub payload: LanguageCodePayload,
}

impl GetMyDescription {
    /// Creates a new `getMyDescription` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: LanguageCodePayload::default(),
        }
    }

    crate::request_setters! {
        language_code(lang: impl Into<String>) -> language_code; into,
    }
}

crate::impl_json_request!(GetMyDescription, "getMyDescription", BotDescription);

/// Request builder for `getMyShortDescription`.
#[derive(Debug, Clone)]
pub struct GetMyShortDescription {
    bot: Bot,
    pub payload: LanguageCodePayload,
}

impl GetMyShortDescription {
    /// Creates a new `getMyShortDescription` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: LanguageCodePayload::default(),
        }
    }

    crate::request_setters! {
        language_code(lang: impl Into<String>) -> language_code; into,
    }
}

crate::impl_json_request!(
    GetMyShortDescription,
    "getMyShortDescription",
    BotShortDescription
);

/// Payload for `setChatMenuButton`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct SetChatMenuButtonPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

/// Request builder for `setChatMenuButton`.
#[derive(Debug, Clone)]
pub struct SetChatMenuButton {
    bot: Bot,
    pub payload: SetChatMenuButtonPayload,
}

impl SetChatMenuButton {
    /// Creates a new `setChatMenuButton` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: SetChatMenuButtonPayload::default(),
        }
    }

    crate::request_setters! {
        chat_id(id: i64) -> chat_id,
        menu_button(btn: MenuButton) -> menu_button,
    }
}

crate::impl_json_request!(SetChatMenuButton, "setChatMenuButton", bool);

/// Payload for `setMyDefaultAdministratorRights`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct SetMyDefaultAdministratorRightsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<BotAdministratorRightsExt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

/// Request builder for `setMyDefaultAdministratorRights`.
#[derive(Debug, Clone)]
pub struct SetMyDefaultAdministratorRights {
    bot: Bot,
    pub payload: SetMyDefaultAdministratorRightsPayload,
}

impl SetMyDefaultAdministratorRights {
    /// Creates a new `setMyDefaultAdministratorRights` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: SetMyDefaultAdministratorRightsPayload::default(),
        }
    }

    crate::request_setters! {
        rights(rights: ChatAdministratorRights) -> rights,
        for_channels(for_channels: bool) -> for_channels,
    }
}

crate::impl_json_request!(
    SetMyDefaultAdministratorRights,
    "setMyDefaultAdministratorRights",
    bool
);

impl Bot {
    /// Fetches up-to-date information about a chat (`getChat`).
    pub fn get_chat(&self, chat_id: impl Into<ChatId>) -> GetChat {
        GetChat::new(self.clone(), chat_id)
    }

    /// Lists administrators of a chat (`getChatAdministrators`).
    pub fn get_chat_administrators(&self, chat_id: impl Into<ChatId>) -> GetChatAdministrators {
        GetChatAdministrators::new(self.clone(), chat_id)
    }

    /// Returns the number of members in a chat (`getChatMemberCount`).
    pub fn get_chat_member_count(&self, chat_id: impl Into<ChatId>) -> GetChatMemberCount {
        GetChatMemberCount::new(self.clone(), chat_id)
    }

    /// Returns information about a member of a chat (`getChatMember`).
    pub fn get_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<UserId>,
    ) -> GetChatMember {
        GetChatMember::new(self.clone(), chat_id, user_id)
    }

    /// Bans a user in a group, supergroup, or channel (`banChatMember`).
    pub fn ban_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<UserId>,
    ) -> BanChatMember {
        BanChatMember::new(self.clone(), chat_id, user_id)
    }

    /// Unbans a previously banned user in a supergroup or channel (`unbanChatMember`).
    pub fn unban_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<UserId>,
    ) -> UnbanChatMember {
        UnbanChatMember::new(self.clone(), chat_id, user_id)
    }

    /// Restricts a user in a supergroup (`restrictChatMember`).
    pub fn restrict_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<UserId>,
        permissions: ChatPermissions,
    ) -> RestrictChatMember {
        RestrictChatMember::new(self.clone(), chat_id, user_id, permissions)
    }

    /// Sets or clears a custom member tag in a group (Bot API 9.5 `setChatMemberTag`).
    pub fn set_chat_member_tag(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<UserId>,
    ) -> SetChatMemberTag {
        SetChatMemberTag::new(self.clone(), chat_id, user_id)
    }

    /// Sets default chat permissions for all members (`setChatPermissions`).
    pub fn set_chat_permissions(
        &self,
        chat_id: impl Into<ChatId>,
        permissions: ChatPermissions,
    ) -> SetChatPermissions {
        SetChatPermissions::new(self.clone(), chat_id, permissions)
    }

    /// Generates a new primary invite link (`exportChatInviteLink`).
    pub fn export_chat_invite_link(&self, chat_id: impl Into<ChatId>) -> ExportChatInviteLink {
        ExportChatInviteLink::new(self.clone(), chat_id)
    }

    /// Creates an additional invite link (`createChatInviteLink`).
    pub fn create_chat_invite_link(&self, chat_id: impl Into<ChatId>) -> CreateChatInviteLink {
        CreateChatInviteLink::new(self.clone(), chat_id)
    }

    /// Revokes an invite link (`revokeChatInviteLink`).
    pub fn revoke_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> RevokeChatInviteLink {
        RevokeChatInviteLink::new(self.clone(), chat_id, invite_link)
    }

    /// Approves a chat join request (`approveChatJoinRequest`).
    pub fn approve_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<UserId>,
    ) -> ApproveChatJoinRequest {
        ApproveChatJoinRequest::new(self.clone(), chat_id, user_id)
    }

    /// Declines a chat join request (`declineChatJoinRequest`).
    pub fn decline_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: impl Into<UserId>,
    ) -> DeclineChatJoinRequest {
        DeclineChatJoinRequest::new(self.clone(), chat_id, user_id)
    }

    /// Answers a chat join request screening query (Bot API 10.1 `answerChatJoinRequestQuery`).
    pub fn answer_chat_join_request_query(
        &self,
        chat_join_request_query_id: impl Into<String>,
        status: impl Into<String>,
    ) -> AnswerChatJoinRequestQuery {
        AnswerChatJoinRequestQuery::new(self.clone(), chat_join_request_query_id, status)
    }

    /// Creates a forum topic in a supergroup or private chat (`createForumTopic`).
    pub fn create_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> CreateForumTopic {
        CreateForumTopic::new(self.clone(), chat_id, name)
    }

    /// Pins a message in a chat (`pinChatMessage`).
    pub fn pin_chat_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<MessageId>,
    ) -> PinChatMessage {
        PinChatMessage::new(self.clone(), chat_id, message_id)
    }

    /// Unpins a message in a chat (`unpinChatMessage`).
    pub fn unpin_chat_message(&self, chat_id: impl Into<ChatId>) -> UnpinChatMessage {
        UnpinChatMessage::new(self.clone(), chat_id)
    }

    /// Unpins all messages in a chat (`unpinAllChatMessages`).
    pub fn unpin_all_chat_messages(&self, chat_id: impl Into<ChatId>) -> UnpinAllChatMessages {
        UnpinAllChatMessages::new(self.clone(), chat_id)
    }

    /// Leaves a group, supergroup, or channel (`leaveChat`).
    pub fn leave_chat(&self, chat_id: impl Into<ChatId>) -> LeaveChat {
        LeaveChat::new(self.clone(), chat_id)
    }

    /// Changes the title of a chat (`setChatTitle`).
    pub fn set_chat_title(
        &self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
    ) -> SetChatTitle {
        SetChatTitle::new(self.clone(), chat_id, title)
    }

    /// Changes the description of a chat (`setChatDescription`).
    pub fn set_chat_description(&self, chat_id: impl Into<ChatId>) -> SetChatDescription {
        SetChatDescription::new(self.clone(), chat_id)
    }

    /// Deletes a chat's profile photo (`deleteChatPhoto`).
    pub fn delete_chat_photo(&self, chat_id: impl Into<ChatId>) -> DeleteChatPhoto {
        DeleteChatPhoto::new(self.clone(), chat_id)
    }

    /// Returns a list of profile pictures for a user (`getUserProfilePhotos`).
    pub fn get_user_profile_photos(&self, user_id: impl Into<UserId>) -> GetUserProfilePhotos {
        GetUserProfilePhotos::new(self.clone(), user_id)
    }

    /// Sets the bot's command list (`setMyCommands`).
    pub fn set_my_commands(&self, commands: Vec<BotCommand>) -> SetMyCommands {
        SetMyCommands::new(self.clone(), commands)
    }

    /// Gets the bot's current command list (`getMyCommands`).
    pub fn get_my_commands(&self) -> GetMyCommands {
        GetMyCommands::new(self.clone())
    }

    /// Deletes the bot's command list (`deleteMyCommands`).
    pub fn delete_my_commands(&self) -> DeleteMyCommands {
        DeleteMyCommands::new(self.clone())
    }

    /// Gets the bot's name (`getMyName`).
    pub fn get_my_name(&self) -> GetMyName {
        GetMyName::new(self.clone())
    }

    /// Sets the bot's name (`setMyName`).
    pub fn set_my_name(&self) -> SetMyName {
        SetMyName::new(self.clone())
    }

    /// Gets the bot's description (`getMyDescription`).
    pub fn get_my_description(&self) -> GetMyDescription {
        GetMyDescription::new(self.clone())
    }

    /// Gets the bot's short description (`getMyShortDescription`).
    pub fn get_my_short_description(&self) -> GetMyShortDescription {
        GetMyShortDescription::new(self.clone())
    }

    /// Changes the bot's menu button (`setChatMenuButton`).
    pub fn set_chat_menu_button(&self) -> SetChatMenuButton {
        SetChatMenuButton::new(self.clone())
    }

    /// Sets default administrator rights requested when adding the bot (`setMyDefaultAdministratorRights`).
    pub fn set_my_default_administrator_rights(&self) -> SetMyDefaultAdministratorRights {
        SetMyDefaultAdministratorRights::new(self.clone())
    }
}
