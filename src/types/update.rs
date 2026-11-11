//! Incoming Telegram Bot API 10.4 `Update` types, `AllowedUpdate`, and `WebhookInfo`.

use serde::{Deserialize, Serialize};

use super::{
    BusinessConnection, BusinessMessagesDeleted, CallbackQuery, Chat, ChatBoostRemoved,
    ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, JsonMap,
    Message, MessageReactionCountUpdated, MessageReactionUpdated, PaidMediaPurchased, Poll,
    PollAnswer, PreCheckoutQuery, SharedWidgetQuery, ShippingQuery, User, WidgetQuery,
};

/// Update sent when a user stops ongoing message/draft generation (Bot API 10.2 / 10.3).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageGenerationStopped {
    /// Chat where message generation was stopped.
    pub chat: Chat,
    /// Forum/private topic identifier if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the draft whose generation was canceled (accepts string or integer).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_id: Option<serde_json::Value>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Update sent to a manager bot when a managed bot is created or its token/owner changes (Bot API 9.6 / 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagedBotUpdated {
    /// Telegram user who created or owns the managed bot.
    pub user: User,
    /// Information about the managed bot itself.
    pub bot: User,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Update describing a change in a user's payment subscription to the bot (Bot API 10.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotSubscriptionUpdated {
    /// User who subscribed or updated their subscription.
    pub user: User,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// New subscription state (e.g. `"active"`, `"canceled"`).
    pub state: String,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Incoming Telegram Bot API update (supports all fields up to Bot API 10.4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update's unique identifier.
    pub update_id: i64,
    /// New incoming message of any kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
    /// New version of a message that is known to the bot and was edited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Box<Message>>,
    /// New incoming channel post of any kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Box<Message>>,
    /// New version of a channel post that is known to the bot and was edited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Box<Message>>,
    /// Bot connected to or disconnected from a business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_connection: Option<BusinessConnection>,
    /// New message from a connected business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_message: Option<Box<Message>>,
    /// Edited message from a connected business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_business_message: Option<Box<Message>>,
    /// Messages deleted from a connected business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_business_messages: Option<BusinessMessagesDeleted>,
    /// A reaction to a message was changed by a user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<MessageReactionUpdated>,
    /// Reactions to a message with anonymous reactions were changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<MessageReactionCountUpdated>,
    /// New incoming inline query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,
    /// Result of an inline query chosen by a user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// New incoming callback query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<Box<CallbackQuery>>,
    /// New incoming shipping query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,
    /// New incoming pre-checkout query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// User purchased paid media with a non-empty payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchased_paid_media: Option<PaidMediaPurchased>,
    /// New poll state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// User changed their answer in a non-anonymous poll.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,
    /// Bot's chat member status was updated in a chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<Box<ChatMemberUpdated>>,
    /// A chat member's status was updated in a chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<Box<ChatMemberUpdated>>,
    /// A request to join the chat has been sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<Box<ChatJoinRequest>>,
    /// A chat boost was added or changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<Box<ChatBoostUpdated>>,
    /// A boost was removed from a chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<Box<ChatBoostRemoved>>,
    /// Managed bot created or updated (Bot API 9.6 / 10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_bot: Option<ManagedBotUpdated>,
    /// Guest mode message received in a chat the bot is not a member of (Bot API 10.0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_message: Option<Box<Message>>,
    /// User clicked the stop-generation control on a streaming draft (Bot API 10.2 / 10.3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopped_message_generation: Option<MessageGenerationStopped>,
    /// User payment subscription state changed (Bot API 10.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<BotSubscriptionUpdated>,
    /// Incoming query from an HTML widget in a message (Bot API 10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_query: Option<Box<WidgetQuery>>,
    /// Incoming query from a forwarded message containing a shared widget (Bot API 10.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_widget_query: Option<Box<SharedWidgetQuery>>,
    /// Forward-compatible unknown update payload fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Borrowed discriminated view over the active payload inside an [`Update`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpdateKind<'a> {
    /// Regular incoming message.
    Message(&'a Message),
    /// Edited message.
    EditedMessage(&'a Message),
    /// Channel post.
    ChannelPost(&'a Message),
    /// Edited channel post.
    EditedChannelPost(&'a Message),
    /// Business connection updated.
    BusinessConnection(&'a BusinessConnection),
    /// Incoming business message.
    BusinessMessage(&'a Message),
    /// Edited business message.
    EditedBusinessMessage(&'a Message),
    /// Deleted business messages.
    DeletedBusinessMessages(&'a BusinessMessagesDeleted),
    /// Non-anonymous message reaction updated.
    MessageReaction(&'a MessageReactionUpdated),
    /// Anonymous message reaction count updated.
    MessageReactionCount(&'a MessageReactionCountUpdated),
    /// Inline query.
    InlineQuery(&'a InlineQuery),
    /// Chosen inline result.
    ChosenInlineResult(&'a ChosenInlineResult),
    /// Callback query from an inline button.
    CallbackQuery(&'a CallbackQuery),
    /// Shipping query.
    ShippingQuery(&'a ShippingQuery),
    /// Pre-checkout query.
    PreCheckoutQuery(&'a PreCheckoutQuery),
    /// Paid media purchase event.
    PurchasedPaidMedia(&'a PaidMediaPurchased),
    /// Poll update.
    Poll(&'a Poll),
    /// Poll answer update.
    PollAnswer(&'a PollAnswer),
    /// Bot's own chat member status update.
    MyChatMember(&'a ChatMemberUpdated),
    /// Another chat member's status update.
    ChatMember(&'a ChatMemberUpdated),
    /// Chat join request.
    ChatJoinRequest(&'a ChatJoinRequest),
    /// Chat boost added or updated.
    ChatBoost(&'a ChatBoostUpdated),
    /// Chat boost removed.
    RemovedChatBoost(&'a ChatBoostRemoved),
    /// Managed bot created/updated (Bot API 9.6 / 10.4).
    ManagedBot(&'a ManagedBotUpdated),
    /// Guest mode message (Bot API 10.0).
    GuestMessage(&'a Message),
    /// User stopped streaming message generation (Bot API 10.2 / 10.3).
    StoppedMessageGeneration(&'a MessageGenerationStopped),
    /// Subscription status updated (Bot API 10.2).
    Subscription(&'a BotSubscriptionUpdated),
    /// Interactive widget query (Bot API 10.4).
    WidgetQuery(&'a WidgetQuery),
    /// Shared widget query from forwarded message (Bot API 10.4).
    SharedWidgetQuery(&'a SharedWidgetQuery),
    /// Unrecognized future update type.
    Unknown,
}

impl Update {
    /// Returns a borrowed [`UpdateKind`] matching whichever field is populated on this update.
    pub fn kind(&self) -> UpdateKind<'_> {
        if let Some(ref m) = self.message {
            UpdateKind::Message(m)
        } else if let Some(ref m) = self.edited_message {
            UpdateKind::EditedMessage(m)
        } else if let Some(ref m) = self.channel_post {
            UpdateKind::ChannelPost(m)
        } else if let Some(ref m) = self.edited_channel_post {
            UpdateKind::EditedChannelPost(m)
        } else if let Some(ref bc) = self.business_connection {
            UpdateKind::BusinessConnection(bc)
        } else if let Some(ref m) = self.business_message {
            UpdateKind::BusinessMessage(m)
        } else if let Some(ref m) = self.edited_business_message {
            UpdateKind::EditedBusinessMessage(m)
        } else if let Some(ref dbm) = self.deleted_business_messages {
            UpdateKind::DeletedBusinessMessages(dbm)
        } else if let Some(ref mr) = self.message_reaction {
            UpdateKind::MessageReaction(mr)
        } else if let Some(ref mrc) = self.message_reaction_count {
            UpdateKind::MessageReactionCount(mrc)
        } else if let Some(ref iq) = self.inline_query {
            UpdateKind::InlineQuery(iq)
        } else if let Some(ref cir) = self.chosen_inline_result {
            UpdateKind::ChosenInlineResult(cir)
        } else if let Some(ref cq) = self.callback_query {
            UpdateKind::CallbackQuery(cq)
        } else if let Some(ref sq) = self.shipping_query {
            UpdateKind::ShippingQuery(sq)
        } else if let Some(ref pcq) = self.pre_checkout_query {
            UpdateKind::PreCheckoutQuery(pcq)
        } else if let Some(ref ppm) = self.purchased_paid_media {
            UpdateKind::PurchasedPaidMedia(ppm)
        } else if let Some(ref p) = self.poll {
            UpdateKind::Poll(p)
        } else if let Some(ref pa) = self.poll_answer {
            UpdateKind::PollAnswer(pa)
        } else if let Some(ref mcm) = self.my_chat_member {
            UpdateKind::MyChatMember(mcm)
        } else if let Some(ref cm) = self.chat_member {
            UpdateKind::ChatMember(cm)
        } else if let Some(ref cjr) = self.chat_join_request {
            UpdateKind::ChatJoinRequest(cjr)
        } else if let Some(ref cb) = self.chat_boost {
            UpdateKind::ChatBoost(cb)
        } else if let Some(ref rcb) = self.removed_chat_boost {
            UpdateKind::RemovedChatBoost(rcb)
        } else if let Some(ref mb) = self.managed_bot {
            UpdateKind::ManagedBot(mb)
        } else if let Some(ref gm) = self.guest_message {
            UpdateKind::GuestMessage(gm)
        } else if let Some(ref smg) = self.stopped_message_generation {
            UpdateKind::StoppedMessageGeneration(smg)
        } else if let Some(ref sub) = self.subscription {
            UpdateKind::Subscription(sub)
        } else if let Some(ref wq) = self.widget_query {
            UpdateKind::WidgetQuery(wq)
        } else if let Some(ref swq) = self.shared_widget_query {
            UpdateKind::SharedWidgetQuery(swq)
        } else {
            UpdateKind::Unknown
        }
    }

    /// Returns the primary [`Message`] associated with this update, if any.
    pub fn primary_message(&self) -> Option<&Message> {
        match self.kind() {
            UpdateKind::Message(m)
            | UpdateKind::EditedMessage(m)
            | UpdateKind::ChannelPost(m)
            | UpdateKind::EditedChannelPost(m)
            | UpdateKind::BusinessMessage(m)
            | UpdateKind::EditedBusinessMessage(m)
            | UpdateKind::GuestMessage(m) => Some(m),
            UpdateKind::CallbackQuery(cq) => cq.message.as_ref().and_then(|m| m.as_message()),
            UpdateKind::WidgetQuery(wq) => wq.message.as_deref(),
            _ => None,
        }
    }

    /// Extracts the originating [`Chat`] associated with this update, if available.
    pub fn chat(&self) -> Option<&Chat> {
        match self.kind() {
            UpdateKind::Message(m)
            | UpdateKind::EditedMessage(m)
            | UpdateKind::ChannelPost(m)
            | UpdateKind::EditedChannelPost(m)
            | UpdateKind::BusinessMessage(m)
            | UpdateKind::EditedBusinessMessage(m)
            | UpdateKind::GuestMessage(m) => Some(&m.chat),
            UpdateKind::DeletedBusinessMessages(dbm) => Some(&dbm.chat),
            UpdateKind::MessageReaction(mr) => Some(&mr.chat),
            UpdateKind::MessageReactionCount(mrc) => Some(&mrc.chat),
            UpdateKind::CallbackQuery(cq) => cq.message.as_ref().map(|m| m.chat()),
            UpdateKind::MyChatMember(m) | UpdateKind::ChatMember(m) => Some(&m.chat),
            UpdateKind::ChatJoinRequest(cjr) => Some(&cjr.chat),
            UpdateKind::ChatBoost(cb) => Some(&cb.chat),
            UpdateKind::RemovedChatBoost(rcb) => Some(&rcb.chat),
            UpdateKind::StoppedMessageGeneration(smg) => Some(&smg.chat),
            UpdateKind::WidgetQuery(wq) => wq.message.as_ref().map(|m| &m.chat),
            UpdateKind::SharedWidgetQuery(swq) => Some(&swq.chat),
            _ => None,
        }
    }

    /// Extracts the acting [`User`] associated with this update, if available.
    pub fn from_user(&self) -> Option<&User> {
        match self.kind() {
            UpdateKind::Message(m)
            | UpdateKind::EditedMessage(m)
            | UpdateKind::ChannelPost(m)
            | UpdateKind::EditedChannelPost(m)
            | UpdateKind::BusinessMessage(m)
            | UpdateKind::EditedBusinessMessage(m) => m.from.as_ref(),
            UpdateKind::GuestMessage(m) => m.guest_bot_caller_user.as_ref().or(m.from.as_ref()),
            UpdateKind::BusinessConnection(bc) => Some(&bc.user),
            UpdateKind::MessageReaction(mr) => mr.user.as_ref(),
            UpdateKind::InlineQuery(iq) => Some(&iq.from),
            UpdateKind::ChosenInlineResult(cir) => Some(&cir.from),
            UpdateKind::CallbackQuery(cq) => Some(&cq.from),
            UpdateKind::ShippingQuery(sq) => Some(&sq.from),
            UpdateKind::PreCheckoutQuery(pcq) => Some(&pcq.from),
            UpdateKind::PurchasedPaidMedia(ppm) => Some(&ppm.from),
            UpdateKind::PollAnswer(pa) => pa.user.as_ref(),
            UpdateKind::MyChatMember(m) | UpdateKind::ChatMember(m) => Some(&m.from),
            UpdateKind::ChatJoinRequest(cjr) => Some(&cjr.from),
            UpdateKind::ManagedBot(mb) => Some(&mb.user),
            UpdateKind::Subscription(sub) => Some(&sub.user),
            UpdateKind::WidgetQuery(wq) => Some(&wq.from),
            UpdateKind::SharedWidgetQuery(swq) => Some(&swq.from),
            _ => None,
        }
    }
}

/// Update type names accepted by `allowed_updates` in `getUpdates` and `setWebhook`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    BusinessConnection,
    BusinessMessage,
    EditedBusinessMessage,
    DeletedBusinessMessages,
    MessageReaction,
    MessageReactionCount,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    PurchasedPaidMedia,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    ChatJoinRequest,
    ChatBoost,
    RemovedChatBoost,
    ManagedBot,
    GuestMessage,
    StoppedMessageGeneration,
    Subscription,
    WidgetQuery,
    SharedWidgetQuery,
}

impl AllowedUpdate {
    /// Returns every update type supported in Bot API 10.4.
    pub const fn all() -> &'static [Self] {
        &[
            Self::Message,
            Self::EditedMessage,
            Self::ChannelPost,
            Self::EditedChannelPost,
            Self::BusinessConnection,
            Self::BusinessMessage,
            Self::EditedBusinessMessage,
            Self::DeletedBusinessMessages,
            Self::MessageReaction,
            Self::MessageReactionCount,
            Self::InlineQuery,
            Self::ChosenInlineResult,
            Self::CallbackQuery,
            Self::ShippingQuery,
            Self::PreCheckoutQuery,
            Self::PurchasedPaidMedia,
            Self::Poll,
            Self::PollAnswer,
            Self::MyChatMember,
            Self::ChatMember,
            Self::ChatJoinRequest,
            Self::ChatBoost,
            Self::RemovedChatBoost,
            Self::ManagedBot,
            Self::GuestMessage,
            Self::StoppedMessageGeneration,
            Self::Subscription,
            Self::WidgetQuery,
            Self::SharedWidgetQuery,
        ]
    }
}

/// Current status of a webhook returned by [`getWebhookInfo`](crate::Bot::get_webhook_info).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up.
    pub url: String,
    /// `true`, if a custom certificate was provided for webhook certificate checks.
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery.
    pub pending_update_count: u64,
    /// Currently used webhook IP address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Unix time for the most recent error that happened when trying to deliver an update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,
    /// Error message in human-readable format for the most recent webhook error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// Unix time of the most recent error that happened when trying to synchronize available updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    /// List of update types the bot is subscribed to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}
