//! Inline keyboards, reply keyboards, WebApp info, and Bot API 10.4 managed bot buttons.

use serde::{Deserialize, Serialize};

use super::ChatAdministratorRights;

/// Uniform wrapper for `reply_markup` in sending and editing methods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    /// Inline keyboard embedded inside the message bubble.
    InlineKeyboard(InlineKeyboardMarkup),
    /// Custom reply keyboard replacing the user's standard keyboard.
    ReplyKeyboard(ReplyKeyboardMarkup),
    /// Requests the client to remove the custom reply keyboard.
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    /// Prompts the user to reply directly to the bot's message.
    ForceReply(ForceReply),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(value: InlineKeyboardMarkup) -> Self {
        Self::InlineKeyboard(value)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(value: ReplyKeyboardMarkup) -> Self {
        Self::ReplyKeyboard(value)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(value: ReplyKeyboardRemove) -> Self {
        Self::ReplyKeyboardRemove(value)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(value: ForceReply) -> Self {
        Self::ForceReply(value)
    }
}

/// Inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an array of [`InlineKeyboardButton`] objects.
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Creates an inline keyboard from rows of buttons.
    pub fn new(rows: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self {
            inline_keyboard: rows,
        }
    }

    /// Appends a new row of buttons to the inline keyboard.
    pub fn row(mut self, buttons: Vec<InlineKeyboardButton>) -> Self {
        self.inline_keyboard.push(buttons);
        self
    }

    /// Appends a single-button row to the inline keyboard.
    pub fn single(self, button: InlineKeyboardButton) -> Self {
        self.row(vec![button])
    }
}

/// One button of an inline keyboard (Bot API 10.4 adds `icon_custom_emoji_id`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InlineKeyboardButton {
    /// Label text on the button.
    pub text: String,
    /// Unique identifier of a custom emoji shown before the text (Bot API 9.4/10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// HTTP or `tg://` URL to be opened when the button is pressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Data to be sent in a callback query to the bot (1-64 bytes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Description of the Web App launched when the user presses the button.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// HTTPS URL used to automatically authorize the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    /// Pressing the button inserts the bot's username and inline query in a chosen chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    /// Pressing the button inserts the bot's username and inline query in the current chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    /// Pressing the button prompts the user to select a chat of a specified type for inline mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    /// Description of the button that copies specified text to the clipboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<CopyTextButton>,
    /// Description of the game that will be launched when the user presses the button.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<serde_json::Value>,
    /// Specify `true` to send a Pay button.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

impl InlineKeyboardButton {
    /// Creates an inline callback button.
    pub fn callback(text: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            callback_data: Some(data.into()),
            ..Default::default()
        }
    }

    /// Creates an inline URL button.
    pub fn url(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    /// Creates an inline Web App button.
    pub fn web_app(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            web_app: Some(WebAppInfo::new(url)),
            ..Default::default()
        }
    }

    /// Creates an inline clipboard-copy button.
    pub fn copy_text(text: impl Into<String>, copy: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            copy_text: Some(CopyTextButton { text: copy.into() }),
            ..Default::default()
        }
    }

    /// Creates an inline Pay button.
    pub fn pay(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            pay: Some(true),
            ..Default::default()
        }
    }

    /// Attaches a custom emoji icon to the button (Bot API 9.4/10.4).
    pub fn icon_custom_emoji(mut self, emoji_id: impl Into<String>) -> Self {
        self.icon_custom_emoji_id = Some(emoji_id.into());
        self
    }
}

/// Custom keyboard with reply options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an array of [`KeyboardButton`] objects.
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Requests clients to always show the keyboard when the regular keyboard is hidden.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,
    /// Requests clients to resize the keyboard vertically for optimal fit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard as soon as it's been used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    /// Placeholder to be shown in the input field when the keyboard is active (1-64 chars).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Show the keyboard to specific users only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl ReplyKeyboardMarkup {
    /// Creates a new reply keyboard with optional resizing enabled.
    pub fn new(rows: Vec<Vec<KeyboardButton>>) -> Self {
        Self {
            keyboard: rows,
            resize_keyboard: Some(true),
            ..Default::default()
        }
    }

    /// Sets `one_time_keyboard`.
    pub fn one_time(mut self, one_time: bool) -> Self {
        self.one_time_keyboard = Some(one_time);
        self
    }

    /// Sets `input_field_placeholder`.
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.input_field_placeholder = Some(text.into());
        self
    }
}

/// One button of a reply keyboard (Bot API 9.6/10.4 includes `request_managed_bot` & `icon_custom_emoji_id`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KeyboardButton {
    /// Text of the button.
    pub text: String,
    /// Unique identifier of a custom emoji shown before the text (Bot API 9.4/10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// If specified, pressing the button opens a list of suitable users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,
    /// If specified, pressing the button opens a list of suitable chats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,
    /// If specified, pressing the button prompts the user to create/select a managed bot (Bot API 9.6/10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_managed_bot: Option<KeyboardButtonRequestManagedBot>,
    /// If `true`, the user's phone number is sent as a contact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    /// If `true`, the user's current location is sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    /// If specified, the user will be asked to create a poll and send it to the bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    /// Description of the Web App launched when the user presses the button.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

impl KeyboardButton {
    /// Creates a plain text reply keyboard button.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Creates a button requesting creation/selection of a managed bot (Bot API 9.6/10.4).
    pub fn request_managed_bot(
        text: impl Into<String>,
        request: KeyboardButtonRequestManagedBot,
    ) -> Self {
        Self {
            text: text.into(),
            request_managed_bot: Some(request),
            ..Default::default()
        }
    }

    /// Attaches a custom emoji icon to the button.
    pub fn icon_custom_emoji(mut self, emoji_id: impl Into<String>) -> Self {
        self.icon_custom_emoji_id = Some(emoji_id.into());
        self
    }
}

/// Criteria used to request creation or selection of a managed bot (Bot API 9.6 / 10.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request, unique within the message.
    pub request_id: i32,
    /// Suggested display name for the managed bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_name: Option<String>,
    /// Suggested username (e.g. ending in `_ai`, `_agent`, `_app`, `_game` for Premium) for the managed bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_username: Option<String>,
}

impl KeyboardButtonRequestManagedBot {
    /// Creates a new managed bot request button payload.
    pub fn new(request_id: i32) -> Self {
        Self {
            request_id,
            suggested_name: None,
            suggested_username: None,
        }
    }

    /// Suggests a bot display name.
    pub fn suggested_name(mut self, name: impl Into<String>) -> Self {
        self.suggested_name = Some(name.into());
        self
    }

    /// Suggests a bot username.
    pub fn suggested_username(mut self, username: impl Into<String>) -> Self {
        self.suggested_username = Some(username.into());
        self
    }
}

/// Criteria used to request suitable users.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request.
    pub request_id: i32,
    /// Pass `true` to request bots, `false` to request regular users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    /// Pass `true` to request premium users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    /// Maximum number of users to be selected (1-10).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<u8>,
    /// Pass `true` to request the users' first and last names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    /// Pass `true` to request the users' usernames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// Pass `true` to request the users' photos.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// Criteria used to request a suitable chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request.
    pub request_id: i32,
    /// Pass `true` to request a channel chat, `false` to request a group or supergroup chat.
    pub chat_is_channel: bool,
    /// Pass `true` to request a forum supergroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    /// Pass `true` to request a supergroup or channel with a username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    /// Pass `true` to request a chat owned by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    /// Required user administrator rights in the chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    /// Required bot administrator rights in the chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    /// Pass `true` to request a chat with the bot as a member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
    /// Pass `true` to request the chat's title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,
    /// Pass `true` to request the chat's username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// Pass `true` to request the chat's photo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// Type of poll a user is allowed to create via a keyboard button.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KeyboardButtonPollType {
    /// If `"quiz"`, only quiz polls can be created; if `"regular"`, only regular polls.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub poll_type: Option<String>,
}

/// Requests Telegram clients to remove the current custom reply keyboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Must be `true`.
    pub remove_keyboard: bool,
    /// Remove keyboard only for specific users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl Default for ReplyKeyboardRemove {
    fn default() -> Self {
        Self {
            remove_keyboard: true,
            selective: None,
        }
    }
}

/// Forces the user to reply directly to the message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForceReply {
    /// Shows reply interface to the user.
    pub force_reply: bool,
    /// Placeholder to be shown in the input field (1-64 chars).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Force reply from specific users only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl Default for ForceReply {
    fn default() -> Self {
        Self {
            force_reply: true,
            input_field_placeholder: None,
            selective: None,
        }
    }
}

/// Describes a Telegram Mini App (Web App).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// HTTPS URL of a Web App to be opened.
    pub url: String,
}

impl WebAppInfo {
    /// Creates a new [`WebAppInfo`].
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

/// Data sent from a Web App to the bot via `Telegram.WebApp.sendData`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebAppData {
    /// The dataPayload.
    pub data: String,
    /// Text of the `web_app` keyboard button from which the Web App was opened.
    pub button_text: String,
}

/// Inline keyboard button that copies text to the user's clipboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyTextButton {
    /// Text to be copied to the clipboard (1-256 characters).
    pub text: String,
}

/// Parameters of an HTTPS URL used for seamless Telegram Login.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginUrl {
    /// HTTPS URL to be opened with user authorization data added to the query string.
    pub url: String,
    /// New text of the button in forwarded messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    /// Username of a bot which will be used for user authorization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    /// Pass `true` to request permission for your bot to send messages to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

/// Inline button configuration to switch to inline mode in a chosen chat type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SwitchInlineQueryChosenChat {
    /// Default inline query to be inserted in the input field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// `true`, if private chats with users can be chosen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    /// `true`, if private chats with bots can be chosen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    /// `true`, if group and supergroup chats can be chosen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    /// `true`, if channel chats can be chosen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}
