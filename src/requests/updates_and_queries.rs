//! Updates, webhooks, callbacks, inline queries, payments, Guest Mode (10.0), Managed Bots (9.6/10.4), and Personal Chat retrieval (10.2).

use std::future::IntoFuture;

use serde::Serialize;

use crate::client::Bot;
use crate::error::Result;
use crate::requests::{RequestFuture, execute_media_request};
use crate::types::{
    AllowedUpdate, BotAccessSettings, InlineQueryResult, InlineQueryResultsButton, InputFile,
    LabeledPrice, Message, SentGuestMessage, SentWebAppMessage, ShippingOption, StarTransactions,
    Update, UserId, WebhookInfo,
};

/// Payload for `getUpdates`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetUpdatesPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

/// Request builder for `getUpdates`.
#[derive(Debug, Clone)]
pub struct GetUpdates {
    bot: Bot,
    pub payload: GetUpdatesPayload,
}

impl GetUpdates {
    /// Creates a new `getUpdates` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: GetUpdatesPayload::default(),
        }
    }

    crate::request_setters! {
        offset(offset: i64) -> offset,
        limit(limit: u8) -> limit,
        timeout(timeout: u32) -> timeout,
        allowed_updates(updates: Vec<AllowedUpdate>) -> allowed_updates,
    }
}

crate::impl_json_request!(GetUpdates, "getUpdates", Vec<Update>);

/// Payload for `setWebhook`.
#[derive(Debug, Clone, Serialize)]
pub struct SetWebhookPayload {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

/// Request builder for `setWebhook`.
#[derive(Debug, Clone)]
pub struct SetWebhook {
    bot: Bot,
    certificate: Option<InputFile>,
    pub payload: SetWebhookPayload,
}

impl SetWebhook {
    /// Creates a new `setWebhook` request.
    pub fn new(bot: Bot, url: impl Into<String>) -> Self {
        Self {
            bot,
            certificate: None,
            payload: SetWebhookPayload {
                url: url.into(),
                ip_address: None,
                max_connections: None,
                allowed_updates: None,
                drop_pending_updates: None,
                secret_token: None,
            },
        }
    }

    /// Attaches a public key certificate for self-signed TLS webhooks.
    pub fn certificate(mut self, cert: InputFile) -> Self {
        self.certificate = Some(cert);
        self
    }

    crate::request_setters! {
        ip_address(ip: impl Into<String>) -> ip_address; into,
        max_connections(max: u8) -> max_connections,
        allowed_updates(updates: Vec<AllowedUpdate>) -> allowed_updates,
        drop_pending_updates(drop: bool) -> drop_pending_updates,
        secret_token(secret: impl Into<String>) -> secret_token; into,
    }

    /// Executes `setWebhook` (automatically uses multipart if a certificate file is attached).
    pub async fn send(self) -> Result<bool> {
        match self.certificate {
            Some(cert) => {
                execute_media_request(
                    &self.bot,
                    "setWebhook",
                    "certificate",
                    &cert,
                    None,
                    None,
                    &self.payload,
                )
                .await
            }
            None => self.bot.call_json("setWebhook", &self.payload).await,
        }
    }
}

impl IntoFuture for SetWebhook {
    type Output = Result<bool>;
    type IntoFuture = RequestFuture<'static, bool>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Payload for `deleteWebhook`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeleteWebhookPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

/// Request builder for `deleteWebhook`.
#[derive(Debug, Clone)]
pub struct DeleteWebhook {
    bot: Bot,
    pub payload: DeleteWebhookPayload,
}

impl DeleteWebhook {
    /// Creates a new `deleteWebhook` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: DeleteWebhookPayload::default(),
        }
    }

    crate::request_setters! {
        drop_pending_updates(drop: bool) -> drop_pending_updates,
    }
}

crate::impl_json_request!(DeleteWebhook, "deleteWebhook", bool);

/// Request builder for `getWebhookInfo`.
#[derive(Debug, Clone)]
pub struct GetWebhookInfo {
    bot: Bot,
    pub payload: crate::requests::EmptyPayload,
}

impl GetWebhookInfo {
    /// Creates a new `getWebhookInfo` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: crate::requests::EmptyPayload {},
        }
    }
}

crate::impl_json_request!(GetWebhookInfo, "getWebhookInfo", WebhookInfo);

/// Payload for `answerCallbackQuery`.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerCallbackQueryPayload {
    pub callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,
}

/// Request builder for `answerCallbackQuery`.
#[derive(Debug, Clone)]
pub struct AnswerCallbackQuery {
    bot: Bot,
    pub payload: AnswerCallbackQueryPayload,
}

impl AnswerCallbackQuery {
    /// Creates a new `answerCallbackQuery` request.
    pub fn new(bot: Bot, callback_query_id: impl Into<String>) -> Self {
        Self {
            bot,
            payload: AnswerCallbackQueryPayload {
                callback_query_id: callback_query_id.into(),
                text: None,
                show_alert: None,
                url: None,
                cache_time: None,
            },
        }
    }

    crate::request_setters! {
        text(text: impl Into<String>) -> text; into,
        show_alert(alert: bool) -> show_alert,
        url(url: impl Into<String>) -> url; into,
        cache_time(secs: u32) -> cache_time,
    }
}

crate::impl_json_request!(AnswerCallbackQuery, "answerCallbackQuery", bool);

/// Payload for `answerInlineQuery`.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerInlineQueryPayload {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<InlineQueryResultsButton>,
}

/// Request builder for `answerInlineQuery`.
#[derive(Debug, Clone)]
pub struct AnswerInlineQuery {
    bot: Bot,
    pub payload: AnswerInlineQueryPayload,
}

impl AnswerInlineQuery {
    /// Creates a new `answerInlineQuery` request.
    pub fn new(
        bot: Bot,
        inline_query_id: impl Into<String>,
        results: Vec<InlineQueryResult>,
    ) -> Self {
        Self {
            bot,
            payload: AnswerInlineQueryPayload {
                inline_query_id: inline_query_id.into(),
                results,
                cache_time: None,
                is_personal: None,
                next_offset: None,
                button: None,
            },
        }
    }

    crate::request_setters! {
        cache_time(secs: u32) -> cache_time,
        is_personal(personal: bool) -> is_personal,
        next_offset(offset: impl Into<String>) -> next_offset; into,
        button(btn: InlineQueryResultsButton) -> button,
    }
}

crate::impl_json_request!(AnswerInlineQuery, "answerInlineQuery", bool);

/// Payload for `answerWebAppQuery`.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerWebAppQueryPayload {
    pub web_app_query_id: String,
    pub result: InlineQueryResult,
}

/// Request builder for `answerWebAppQuery`.
#[derive(Debug, Clone)]
pub struct AnswerWebAppQuery {
    bot: Bot,
    pub payload: AnswerWebAppQueryPayload,
}

impl AnswerWebAppQuery {
    /// Creates a new `answerWebAppQuery` request.
    pub fn new(bot: Bot, web_app_query_id: impl Into<String>, result: InlineQueryResult) -> Self {
        Self {
            bot,
            payload: AnswerWebAppQueryPayload {
                web_app_query_id: web_app_query_id.into(),
                result,
            },
        }
    }
}

crate::impl_json_request!(AnswerWebAppQuery, "answerWebAppQuery", SentWebAppMessage);

/// Payload for Bot API 10.0 `answerGuestQuery`.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerGuestQueryPayload {
    pub guest_query_id: String,
    pub result: InlineQueryResult,
}

/// Request builder for Bot API 10.0 `answerGuestQuery` (Guest Mode).
#[derive(Debug, Clone)]
pub struct AnswerGuestQuery {
    bot: Bot,
    pub payload: AnswerGuestQueryPayload,
}

impl AnswerGuestQuery {
    /// Creates a new `answerGuestQuery` request to reply in a chat the bot has not joined.
    pub fn new(bot: Bot, guest_query_id: impl Into<String>, result: InlineQueryResult) -> Self {
        Self {
            bot,
            payload: AnswerGuestQueryPayload {
                guest_query_id: guest_query_id.into(),
                result,
            },
        }
    }
}

crate::impl_json_request!(AnswerGuestQuery, "answerGuestQuery", SentGuestMessage);

/// Payload for `answerShippingQuery`.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerShippingQueryPayload {
    pub shipping_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Request builder for `answerShippingQuery`.
#[derive(Debug, Clone)]
pub struct AnswerShippingQuery {
    bot: Bot,
    pub payload: AnswerShippingQueryPayload,
}

impl AnswerShippingQuery {
    /// Creates an approval response with `shipping_options`.
    pub fn ok(
        bot: Bot,
        shipping_query_id: impl Into<String>,
        shipping_options: Vec<ShippingOption>,
    ) -> Self {
        Self {
            bot,
            payload: AnswerShippingQueryPayload {
                shipping_query_id: shipping_query_id.into(),
                ok: true,
                shipping_options: Some(shipping_options),
                error_message: None,
            },
        }
    }

    /// Creates a rejection response with `error_message`.
    pub fn err(
        bot: Bot,
        shipping_query_id: impl Into<String>,
        error_message: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            payload: AnswerShippingQueryPayload {
                shipping_query_id: shipping_query_id.into(),
                ok: false,
                shipping_options: None,
                error_message: Some(error_message.into()),
            },
        }
    }
}

crate::impl_json_request!(AnswerShippingQuery, "answerShippingQuery", bool);

/// Payload for `answerPreCheckoutQuery`.
#[derive(Debug, Clone, Serialize)]
pub struct AnswerPreCheckoutQueryPayload {
    pub pre_checkout_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Request builder for `answerPreCheckoutQuery`.
#[derive(Debug, Clone)]
pub struct AnswerPreCheckoutQuery {
    bot: Bot,
    pub payload: AnswerPreCheckoutQueryPayload,
}

impl AnswerPreCheckoutQuery {
    /// Confirms that the order can proceed.
    pub fn ok(bot: Bot, pre_checkout_query_id: impl Into<String>) -> Self {
        Self {
            bot,
            payload: AnswerPreCheckoutQueryPayload {
                pre_checkout_query_id: pre_checkout_query_id.into(),
                ok: true,
                error_message: None,
            },
        }
    }

    /// Rejects the checkout with a user-visible `error_message`.
    pub fn err(
        bot: Bot,
        pre_checkout_query_id: impl Into<String>,
        error_message: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            payload: AnswerPreCheckoutQueryPayload {
                pre_checkout_query_id: pre_checkout_query_id.into(),
                ok: false,
                error_message: Some(error_message.into()),
            },
        }
    }
}

crate::impl_json_request!(AnswerPreCheckoutQuery, "answerPreCheckoutQuery", bool);

/// Payload for `sendInvoice`.
#[derive(Debug, Clone, Serialize)]
pub struct SendInvoicePayload {
    pub chat_id: crate::types::ChatId,
    pub title: String,
    pub description: String,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}

/// Request builder for `sendInvoice`.
#[derive(Debug, Clone)]
pub struct SendInvoice {
    bot: Bot,
    pub payload: SendInvoicePayload,
}

impl SendInvoice {
    /// Creates a new `sendInvoice` request (for Telegram Stars, pass currency `"XTR"` and omit `provider_token`).
    pub fn new(
        bot: Bot,
        chat_id: impl Into<crate::types::ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        invoice_payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            bot,
            payload: SendInvoicePayload {
                chat_id: chat_id.into(),
                title: title.into(),
                description: description.into(),
                payload: invoice_payload.into(),
                provider_token: None,
                currency: currency.into(),
                prices,
                message_thread_id: None,
                subscription_period: None,
                start_parameter: None,
                need_name: None,
                need_phone_number: None,
                need_email: None,
                need_shipping_address: None,
                is_flexible: None,
                reply_markup: None,
            },
        }
    }

    crate::request_setters! {
        provider_token(token: impl Into<String>) -> provider_token; into,
        message_thread_id(id: i64) -> message_thread_id,
        subscription_period(secs: u32) -> subscription_period,
        start_parameter(param: impl Into<String>) -> start_parameter; into,
        need_name(need: bool) -> need_name,
        need_phone_number(need: bool) -> need_phone_number,
        need_email(need: bool) -> need_email,
        need_shipping_address(need: bool) -> need_shipping_address,
        is_flexible(flex: bool) -> is_flexible,
        reply_markup(markup: crate::types::InlineKeyboardMarkup) -> reply_markup,
    }
}

crate::impl_json_request!(SendInvoice, "sendInvoice", Message);

/// Payload for `refundStarPayment`.
#[derive(Debug, Clone, Serialize)]
pub struct RefundStarPaymentPayload {
    pub user_id: UserId,
    pub telegram_payment_charge_id: String,
}

/// Request builder for `refundStarPayment`.
#[derive(Debug, Clone)]
pub struct RefundStarPayment {
    bot: Bot,
    pub payload: RefundStarPaymentPayload,
}

impl RefundStarPayment {
    /// Creates a new `refundStarPayment` request.
    pub fn new(
        bot: Bot,
        user_id: impl Into<UserId>,
        telegram_payment_charge_id: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            payload: RefundStarPaymentPayload {
                user_id: user_id.into(),
                telegram_payment_charge_id: telegram_payment_charge_id.into(),
            },
        }
    }
}

crate::impl_json_request!(RefundStarPayment, "refundStarPayment", bool);

/// Payload for `getStarTransactions`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetStarTransactionsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

/// Request builder for `getStarTransactions`.
#[derive(Debug, Clone)]
pub struct GetStarTransactions {
    bot: Bot,
    pub payload: GetStarTransactionsPayload,
}

impl GetStarTransactions {
    /// Creates a new `getStarTransactions` request.
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            payload: GetStarTransactionsPayload::default(),
        }
    }

    crate::request_setters! {
        offset(offset: u32) -> offset,
        limit(limit: u8) -> limit,
    }
}

crate::impl_json_request!(GetStarTransactions, "getStarTransactions", StarTransactions);

/// Payload referencing a managed bot (`user_id` of the managed bot).
#[derive(Debug, Clone, Serialize)]
pub struct ManagedBotIdPayload {
    pub user_id: UserId,
}

/// Request builder for Bot API 9.6 `getManagedBotToken`.
#[derive(Debug, Clone)]
pub struct GetManagedBotToken {
    bot: Bot,
    pub payload: ManagedBotIdPayload,
}

impl GetManagedBotToken {
    /// Creates a new `getManagedBotToken` request for the given managed bot user ID.
    pub fn new(bot: Bot, managed_bot_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: ManagedBotIdPayload {
                user_id: managed_bot_id.into(),
            },
        }
    }
}

crate::impl_json_request!(GetManagedBotToken, "getManagedBotToken", String);

/// Request builder for Bot API 9.6 `replaceManagedBotToken`.
#[derive(Debug, Clone)]
pub struct ReplaceManagedBotToken {
    bot: Bot,
    pub payload: ManagedBotIdPayload,
}

impl ReplaceManagedBotToken {
    /// Revokes the old token of the managed bot and issues a new token.
    pub fn new(bot: Bot, managed_bot_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: ManagedBotIdPayload {
                user_id: managed_bot_id.into(),
            },
        }
    }
}

crate::impl_json_request!(ReplaceManagedBotToken, "replaceManagedBotToken", String);

/// Request builder for Bot API 10.2 `getManagedBotAccessSettings`.
#[derive(Debug, Clone)]
pub struct GetManagedBotAccessSettings {
    bot: Bot,
    pub payload: ManagedBotIdPayload,
}

impl GetManagedBotAccessSettings {
    /// Fetches current access control settings for a managed bot.
    pub fn new(bot: Bot, managed_bot_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: ManagedBotIdPayload {
                user_id: managed_bot_id.into(),
            },
        }
    }
}

crate::impl_json_request!(
    GetManagedBotAccessSettings,
    "getManagedBotAccessSettings",
    BotAccessSettings
);

/// Payload for Bot API 10.2 `setManagedBotAccessSettings`.
#[derive(Debug, Clone, Serialize)]
pub struct SetManagedBotAccessSettingsPayload {
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_restricted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_user_ids: Option<Vec<UserId>>,
}

/// Request builder for Bot API 10.2 `setManagedBotAccessSettings`.
#[derive(Debug, Clone)]
pub struct SetManagedBotAccessSettings {
    bot: Bot,
    pub payload: SetManagedBotAccessSettingsPayload,
}

impl SetManagedBotAccessSettings {
    /// Updates access control settings for a managed bot.
    pub fn new(bot: Bot, managed_bot_id: impl Into<UserId>) -> Self {
        Self {
            bot,
            payload: SetManagedBotAccessSettingsPayload {
                user_id: managed_bot_id.into(),
                is_restricted: None,
                allowed_user_ids: None,
            },
        }
    }

    crate::request_setters! {
        restricted(is_restricted: bool) -> is_restricted,
        allowed_user_ids(ids: Vec<UserId>) -> allowed_user_ids,
    }
}

crate::impl_json_request!(
    SetManagedBotAccessSettings,
    "setManagedBotAccessSettings",
    bool
);

/// Payload for Bot API 10.2 `getUserPersonalChatMessages`.
#[derive(Debug, Clone, Serialize)]
pub struct GetUserPersonalChatMessagesPayload {
    pub user_id: UserId,
    pub limit: u8,
}

/// Request builder for Bot API 10.2 `getUserPersonalChatMessages`.
#[derive(Debug, Clone)]
pub struct GetUserPersonalChatMessages {
    bot: Bot,
    pub payload: GetUserPersonalChatMessagesPayload,
}

impl GetUserPersonalChatMessages {
    /// Fetches up to `limit` (1-20) recent messages from the user's personal chat.
    pub fn new(bot: Bot, user_id: impl Into<UserId>, limit: u8) -> Self {
        Self {
            bot,
            payload: GetUserPersonalChatMessagesPayload {
                user_id: user_id.into(),
                limit: limit.clamp(1, 20),
            },
        }
    }
}

crate::impl_json_request!(
    GetUserPersonalChatMessages,
    "getUserPersonalChatMessages",
    Vec<Message>
);

impl Bot {
    /// Long-polls for incoming updates (`getUpdates`).
    pub fn get_updates(&self) -> GetUpdates {
        GetUpdates::new(self.clone())
    }

    /// Configures an HTTPS webhook URL for update delivery (`setWebhook`).
    pub fn set_webhook(&self, url: impl Into<String>) -> SetWebhook {
        SetWebhook::new(self.clone(), url)
    }

    /// Removes webhook integration (`deleteWebhook`).
    pub fn delete_webhook(&self) -> DeleteWebhook {
        DeleteWebhook::new(self.clone())
    }

    /// Retrieves current webhook configuration and pending count (`getWebhookInfo`).
    pub fn get_webhook_info(&self) -> GetWebhookInfo {
        GetWebhookInfo::new(self.clone())
    }

    /// Sends an answer to a callback query (`answerCallbackQuery`).
    pub fn answer_callback_query(
        &self,
        callback_query_id: impl Into<String>,
    ) -> AnswerCallbackQuery {
        AnswerCallbackQuery::new(self.clone(), callback_query_id)
    }

    /// Sends results for an inline query (`answerInlineQuery`).
    pub fn answer_inline_query(
        &self,
        inline_query_id: impl Into<String>,
        results: Vec<InlineQueryResult>,
    ) -> AnswerInlineQuery {
        AnswerInlineQuery::new(self.clone(), inline_query_id, results)
    }

    /// Sets the result of an interaction with a Web App (`answerWebAppQuery`).
    pub fn answer_web_app_query(
        &self,
        web_app_query_id: impl Into<String>,
        result: InlineQueryResult,
    ) -> AnswerWebAppQuery {
        AnswerWebAppQuery::new(self.clone(), web_app_query_id, result)
    }

    /// Replies to a Guest Mode query in a chat where the bot is not a member (Bot API 10.0 `answerGuestQuery`).
    pub fn answer_guest_query(
        &self,
        guest_query_id: impl Into<String>,
        result: InlineQueryResult,
    ) -> AnswerGuestQuery {
        AnswerGuestQuery::new(self.clone(), guest_query_id, result)
    }

    /// Approves a shipping query (`answerShippingQuery`).
    pub fn approve_shipping_query(
        &self,
        shipping_query_id: impl Into<String>,
        shipping_options: Vec<ShippingOption>,
    ) -> AnswerShippingQuery {
        AnswerShippingQuery::ok(self.clone(), shipping_query_id, shipping_options)
    }

    /// Rejects a shipping query (`answerShippingQuery`).
    pub fn reject_shipping_query(
        &self,
        shipping_query_id: impl Into<String>,
        error_message: impl Into<String>,
    ) -> AnswerShippingQuery {
        AnswerShippingQuery::err(self.clone(), shipping_query_id, error_message)
    }

    /// Approves a pre-checkout query (`answerPreCheckoutQuery`).
    pub fn approve_pre_checkout_query(
        &self,
        pre_checkout_query_id: impl Into<String>,
    ) -> AnswerPreCheckoutQuery {
        AnswerPreCheckoutQuery::ok(self.clone(), pre_checkout_query_id)
    }

    /// Rejects a pre-checkout query (`answerPreCheckoutQuery`).
    pub fn reject_pre_checkout_query(
        &self,
        pre_checkout_query_id: impl Into<String>,
        error_message: impl Into<String>,
    ) -> AnswerPreCheckoutQuery {
        AnswerPreCheckoutQuery::err(self.clone(), pre_checkout_query_id, error_message)
    }

    /// Sends a payment or Telegram Stars invoice (`sendInvoice`).
    pub fn send_invoice(
        &self,
        chat_id: impl Into<crate::types::ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> SendInvoice {
        SendInvoice::new(
            self.clone(),
            chat_id,
            title,
            description,
            payload,
            currency,
            prices,
        )
    }

    /// Refunds a successful Telegram Stars payment (`refundStarPayment`).
    pub fn refund_star_payment(
        &self,
        user_id: impl Into<UserId>,
        telegram_payment_charge_id: impl Into<String>,
    ) -> RefundStarPayment {
        RefundStarPayment::new(self.clone(), user_id, telegram_payment_charge_id)
    }

    /// Fetches the bot's Telegram Stars transaction history (`getStarTransactions`).
    pub fn get_star_transactions(&self) -> GetStarTransactions {
        GetStarTransactions::new(self.clone())
    }

    /// Retrieves the token of a managed bot (Bot API 9.6 `getManagedBotToken`).
    pub fn get_managed_bot_token(&self, managed_bot_id: impl Into<UserId>) -> GetManagedBotToken {
        GetManagedBotToken::new(self.clone(), managed_bot_id)
    }

    /// Revokes and replaces the token of a managed bot (Bot API 9.6 `replaceManagedBotToken`).
    pub fn replace_managed_bot_token(
        &self,
        managed_bot_id: impl Into<UserId>,
    ) -> ReplaceManagedBotToken {
        ReplaceManagedBotToken::new(self.clone(), managed_bot_id)
    }

    /// Gets access settings for a managed bot (Bot API 10.2 `getManagedBotAccessSettings`).
    pub fn get_managed_bot_access_settings(
        &self,
        managed_bot_id: impl Into<UserId>,
    ) -> GetManagedBotAccessSettings {
        GetManagedBotAccessSettings::new(self.clone(), managed_bot_id)
    }

    /// Updates access settings for a managed bot (Bot API 10.2 `setManagedBotAccessSettings`).
    pub fn set_managed_bot_access_settings(
        &self,
        managed_bot_id: impl Into<UserId>,
    ) -> SetManagedBotAccessSettings {
        SetManagedBotAccessSettings::new(self.clone(), managed_bot_id)
    }

    /// Fetches recent messages from a user's personal chat (Bot API 10.2 `getUserPersonalChatMessages`).
    pub fn get_user_personal_chat_messages(
        &self,
        user_id: impl Into<UserId>,
        limit: u8,
    ) -> GetUserPersonalChatMessages {
        GetUserPersonalChatMessages::new(self.clone(), user_id, limit)
    }
}
