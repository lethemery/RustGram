//! Payments, Telegram Stars, invoices, shipping, pre-checkout queries, and subscriptions.

use serde::{Deserialize, Serialize};

use super::{JsonMap, User};

/// Portion of the price for goods or services.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabeledPrice {
    /// Portion label.
    pub label: String,
    /// Price of the product in the smallest units of the currency (integer, not float).
    pub amount: i64,
}

impl LabeledPrice {
    /// Creates a new [`LabeledPrice`].
    pub fn new(label: impl Into<String>, amount: i64) -> Self {
        Self {
            label: label.into(),
            amount,
        }
    }
}

/// Basic information about an invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invoice {
    /// Product name.
    pub title: String,
    /// Product description.
    pub description: String,
    /// Unique bot deep-linking parameter.
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code, or `"XTR"` for Telegram Stars.
    pub currency: String,
    /// Total price in the smallest units of the currency.
    pub total_amount: i64,
}

/// Shipping address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShippingAddress {
    /// Two-letter ISO 3166-1 alpha-2 country code.
    pub country_code: String,
    /// State, if applicable.
    pub state: String,
    /// City.
    pub city: String,
    /// First line for the address.
    pub street_line1: String,
    /// Second line for the address.
    pub street_line2: String,
    /// Address post code.
    pub post_code: String,
}

/// Information about an order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OrderInfo {
    /// User name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User's phone number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// User email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User shipping address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

/// One shipping option.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShippingOption {
    /// Shipping option identifier.
    pub id: String,
    /// Option title.
    pub title: String,
    /// List of price portions.
    pub prices: Vec<LabeledPrice>,
}

/// Basic information about a successful payment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code or `"XTR"`.
    pub currency: String,
    /// Total price in the smallest units of the currency.
    pub total_amount: i64,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// Expiration date of the subscription in Unix time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_expiration_date: Option<i64>,
    /// `true`, if the payment is a recurring payment for a subscription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
    /// `true`, if the payment is the first payment for a subscription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_first_recurring: Option<bool>,
    /// Identifier of the shipping option chosen by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Order information provided by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier.
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier.
    pub provider_payment_charge_id: String,
}

/// Basic information about a refunded payment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 currency code or `"XTR"`.
    pub currency: String,
    /// Total refunded price in the smallest units of the currency.
    pub total_amount: i64,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// Telegram payment identifier.
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}

/// Incoming shipping query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier.
    pub id: String,
    /// User who sent the query.
    pub from: User,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// User specified shipping address.
    pub shipping_address: ShippingAddress,
}

/// Incoming pre-checkout query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier.
    pub id: String,
    /// User who sent the query.
    pub from: User,
    /// Three-letter ISO 4217 currency code or `"XTR"`.
    pub currency: String,
    /// Total price in the smallest units of the currency.
    pub total_amount: i64,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Order information provided by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}

/// Telegram Star transaction record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarTransaction {
    /// Unique identifier of the transaction.
    pub id: String,
    /// Integer amount of Telegram Stars transferred.
    pub amount: i64,
    /// Number of 1/1000000000 shares of Telegram Stars transferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i64>,
    /// Date the transaction was created in Unix time.
    pub date: i64,
    /// Source of an incoming transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
    /// Receiver of an outgoing transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver: Option<serde_json::Value>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// List of Telegram Star transactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarTransactions {
    /// List of transactions.
    pub transactions: Vec<StarTransaction>,
}
