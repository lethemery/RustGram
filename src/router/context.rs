//! Per-update execution context, shared dependency extensions, and ergonomic reply helpers.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use crate::client::Bot;
use crate::error::{Error, Result};
use crate::requests::{
    AnswerCallbackQuery, AnswerWidgetQuery, SendMessage, SendRichMessage, SendWidget,
};
use crate::router::fsm::{StateContext, StateKey, StateStorage};
use crate::types::{
    CallbackQuery, Chat, ChatId, EphemeralMessageParameters, InlineQueryResult, InputRichMessage,
    ManagedBotUpdated, Message, MessageGenerationStopped, SentGuestMessage, SharedWidgetQuery,
    Update, User, WidgetQuery,
};

/// Type-erased dependency container shared across handlers (`Dispatcher::manage`).
#[derive(Debug, Default, Clone)]
pub struct Extensions {
    map: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Extensions {
    /// Creates an empty [`Extensions`] map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a shared state value of type `T`.
    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Arc::new(value));
    }

    /// Inserts an already-wrapped `Arc<T>` into the container.
    pub fn insert_arc<T: Send + Sync + 'static>(&mut self, value: Arc<T>) {
        self.map.insert(TypeId::of::<T>(), value);
    }

    /// Retrieves a reference to a registered value of type `T`.
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|arc| arc.downcast_ref::<T>())
    }

    /// Retrieves a cloned `Arc<T>` if registered directly via [`Self::insert_arc`] or [`Self::insert`].
    pub fn get_arc<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|arc| Arc::clone(arc).downcast::<T>().ok())
    }
}

/// Execution context passed to route handlers and filters.
#[derive(Clone)]
pub struct Context {
    /// Bot API client.
    pub bot: Bot,
    /// Incoming update being processed.
    pub update: Arc<Update>,
    extensions: Arc<Extensions>,
}

impl Context {
    /// Constructs a new [`Context`].
    pub fn new(bot: Bot, update: Update, extensions: Arc<Extensions>) -> Self {
        Self {
            bot,
            update: Arc::new(update),
            extensions,
        }
    }

    /// Retrieves shared dependency state registered via `Dispatcher::manage`.
    pub fn state<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.extensions.get::<T>()
    }

    /// Retrieves a cloned `Arc<T>` shared dependency registered via `Dispatcher::manage`.
    pub fn state_arc<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.extensions.get_arc::<T>()
    }

    /// Returns an FSM [`StateContext`] for the current `(chat_id, user_id)` using registered storage.
    pub fn fsm<S>(&self) -> Result<StateContext<S>>
    where
        S: Clone + Send + Sync + 'static,
    {
        let storage = self
            .extensions
            .get_arc::<Arc<dyn StateStorage<S>>>()
            .map(|outer| Arc::clone(&*outer))
            .ok_or_else(|| {
                Error::State(
                    "FSM storage not registered on Dispatcher (call `.manage_fsm(...)`)".into(),
                )
            })?;
        let key = self.state_key().ok_or_else(|| {
            Error::State("current update has no associated chat_id for FSM".into())
        })?;
        Ok(StateContext::new(key, storage))
    }

    /// Computes the `(chat_id, user_id)` [`StateKey`] for the current update.
    pub fn state_key(&self) -> Option<StateKey> {
        let chat_id = self.chat()?.id;
        let user_id = self.user().map_or(0, |u| u.id.get());
        Some(StateKey::new(chat_id, user_id))
    }

    /// Returns the primary [`Message`] of this update, if any.
    #[inline]
    pub fn message(&self) -> Option<&Message> {
        self.update.primary_message()
    }

    /// Returns the incoming [`CallbackQuery`], if this is a callback query update.
    #[inline]
    pub fn callback_query(&self) -> Option<&CallbackQuery> {
        self.update.callback_query.as_deref()
    }

    /// Returns the incoming Bot API 10.4 [`WidgetQuery`], if present.
    #[inline]
    pub fn widget_query(&self) -> Option<&WidgetQuery> {
        self.update.widget_query.as_deref()
    }

    /// Returns the incoming Bot API 10.4 [`SharedWidgetQuery`], if present.
    #[inline]
    pub fn shared_widget_query(&self) -> Option<&SharedWidgetQuery> {
        self.update.shared_widget_query.as_deref()
    }

    /// Returns the incoming Bot API 10.0 Guest Mode [`Message`], if present.
    #[inline]
    pub fn guest_message(&self) -> Option<&Message> {
        self.update.guest_message.as_deref()
    }

    /// Returns the Bot API 10.2/10.3 [`MessageGenerationStopped`] payload, if present.
    #[inline]
    pub fn stopped_generation(&self) -> Option<&MessageGenerationStopped> {
        self.update.stopped_message_generation.as_ref()
    }

    /// Returns the Bot API 9.6/10.4 [`ManagedBotUpdated`] payload, if present.
    #[inline]
    pub fn managed_bot(&self) -> Option<&ManagedBotUpdated> {
        self.update.managed_bot.as_ref()
    }

    /// Returns the originating [`Chat`] for this update, if available.
    #[inline]
    pub fn chat(&self) -> Option<&Chat> {
        self.update.chat()
    }

    /// Returns the originating numeric [`ChatId`], if available.
    #[inline]
    pub fn chat_id(&self) -> Option<ChatId> {
        self.chat().map(|c| ChatId::Id(c.id))
    }

    /// Returns the acting [`User`] for this update, if available.
    #[inline]
    pub fn user(&self) -> Option<&User> {
        self.update.from_user()
    }

    /// Parses `(command, args)` from the current message text, if it starts with `/`.
    #[inline]
    pub fn command(&self) -> Option<(&str, &str)> {
        self.message().and_then(Message::parse_command)
    }

    /// Creates a [`SendMessage`] builder targeting the current chat.
    pub fn send_message(&self, text: impl Into<String>) -> Result<SendMessage> {
        let chat_id = self.require_chat_id()?;
        Ok(self.bot.send_message(chat_id, text))
    }

    /// Sends a plain reply message directly to the current chat.
    pub async fn reply(&self, text: impl Into<String>) -> Result<Message> {
        self.send_message(text)?.send().await
    }

    /// Creates a [`SendRichMessage`] builder targeting the current chat (Bot API 10.1+).
    pub fn send_rich(&self, rich: InputRichMessage) -> Result<SendRichMessage> {
        let chat_id = self.require_chat_id()?;
        Ok(self.bot.send_rich_message(chat_id, rich))
    }

    /// Sends a structured Rich Message directly to the current chat.
    pub async fn reply_rich(&self, rich: InputRichMessage) -> Result<Message> {
        self.send_rich(rich)?.send().await
    }

    /// Creates a [`SendWidget`] builder targeting the current chat (Bot API 10.4).
    pub fn send_widget(&self, html: impl Into<String>) -> Result<SendWidget> {
        let chat_id = self.require_chat_id()?;
        Ok(self.bot.send_widget(chat_id, html))
    }

    /// Sends an HTML widget directly to the current chat (Bot API 10.4).
    pub async fn reply_widget(&self, html: impl Into<String>) -> Result<Message> {
        self.send_widget(html)?.send().await
    }

    /// Sends an ephemeral message visible only to the triggering user (Bot API 10.2–10.4).
    ///
    /// When handling a `CallbackQuery`, automatically uses `callback_query_id` (Bot API 10.4)
    /// without needing `receiver_user_id`; otherwise targets `self.user()`.
    pub async fn reply_ephemeral(&self, text: impl Into<String>) -> Result<Message> {
        let params = if let Some(cq) = self.callback_query() {
            EphemeralMessageParameters::for_callback(&cq.id)
        } else if let Some(user) = self.user() {
            EphemeralMessageParameters::for_user(user.id)
        } else {
            return Err(Error::InvalidInput(
                "cannot determine target user or callback_query_id for ephemeral reply".into(),
            ));
        };

        self.send_message(text)?.ephemeral(params).send().await
    }

    /// Answers the active `CallbackQuery` if present.
    pub fn answer_callback(&self) -> Result<AnswerCallbackQuery> {
        let cq = self
            .callback_query()
            .ok_or_else(|| Error::InvalidInput("update is not a callback_query".into()))?;
        Ok(self.bot.answer_callback_query(&cq.id))
    }

    /// Answers the active [`WidgetQuery`] or [`SharedWidgetQuery`] (Bot API 10.4).
    pub fn answer_widget(&self) -> Result<AnswerWidgetQuery> {
        let id = self
            .widget_query()
            .map(|q| q.id.as_str())
            .or_else(|| self.shared_widget_query().map(|q| q.id.as_str()))
            .ok_or_else(|| {
                Error::InvalidInput("update is neither widget_query nor shared_widget_query".into())
            })?;
        Ok(self.bot.answer_widget_query(id))
    }

    /// Answers the active Guest Mode message query (Bot API 10.0).
    pub async fn answer_guest(&self, result: InlineQueryResult) -> Result<SentGuestMessage> {
        let guest_query_id = self
            .guest_message()
            .and_then(|m| m.guest_query_id.as_deref())
            .ok_or_else(|| {
                Error::InvalidInput(
                    "update does not contain a guest_message with guest_query_id".into(),
                )
            })?;
        self.bot
            .answer_guest_query(guest_query_id, result)
            .send()
            .await
    }

    fn require_chat_id(&self) -> Result<ChatId> {
        self.chat_id().ok_or_else(|| {
            Error::InvalidInput("cannot reply: update has no associated chat_id".into())
        })
    }
}
