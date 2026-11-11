//! Inline queries, callback queries, inline query results, and Bot API 10.4 widget inline contents.

use serde::{Deserialize, Serialize};

use super::{
    InlineKeyboardMarkup, InputSharedWidgetMessageContent, InputWidgetMessageContent, LabeledPrice,
    LinkPreviewOptions, Location, MaybeInaccessibleMessage, MessageEntity, ParseMode, User,
    WebAppInfo,
};

/// Incoming callback query from an inline keyboard button.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query.
    pub id: String,
    /// Sender.
    pub from: User,
    /// Message sent by the bot with the callback button that originated the query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<MaybeInaccessibleMessage>,
    /// Identifier of the message sent via the bot in inline mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier corresponding to the chat to which the message was sent.
    pub chat_instance: String,
    /// Data associated with the callback button.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Short name of a Game to be returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

/// Incoming inline query (`@bot ...`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query.
    pub id: String,
    /// Sender.
    pub from: User,
    /// Text of the query (up to 256 characters).
    pub query: String,
    /// Offset of the results to be returned.
    pub offset: String,
    /// Type of the chat from which the inline query was sent (`sender`, `private`, `group`, `supergroup`, `channel`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// Sender location, only for bots that request user location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// Represents a result of an inline query that was chosen by the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// Unique identifier for the result that was chosen.
    pub result_id: String,
    /// The user that chose the result.
    pub from: User,
    /// Sender location, only for bots that require user location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Identifier of the sent inline message (available if inline keyboard is attached).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result.
    pub query: String,
}

/// Button to be shown above inline query results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineQueryResultsButton {
    /// Label text on the button.
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// Deep-linking parameter for the `/start` message sent to the bot when a user presses the button.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}

/// Information about an inline message sent by a Web App on behalf of a user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// Content of a message to be sent as a result of an inline query (Bot API 10.4 includes Widgets).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Text message content (`InputTextMessageContent`).
    Text {
        /// Text of the message to be sent (1-4096 characters).
        message_text: String,
        /// Mode for parsing entities in the message text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Special entities in the text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        entities: Option<Vec<MessageEntity>>,
        /// Link preview generation options.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        link_preview_options: Option<LinkPreviewOptions>,
    },
    /// New HTML widget message content (`InputWidgetMessageContent`, Bot API 10.4).
    Widget(InputWidgetMessageContent),
    /// Existing shared HTML widget message content (`InputSharedWidgetMessageContent`, Bot API 10.4).
    SharedWidget(InputSharedWidgetMessageContent),
    /// Location message content (`InputLocationMessageContent`).
    Location {
        /// Latitude of the location in degrees.
        latitude: f64,
        /// Longitude of the location in degrees.
        longitude: f64,
        /// Uncertainty radius in meters.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        horizontal_accuracy: Option<f64>,
        /// Live period in seconds.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        live_period: Option<u32>,
        /// Heading in degrees.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        heading: Option<u16>,
        /// Proximity alert radius in meters.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        proximity_alert_radius: Option<u32>,
    },
    /// Venue message content (`InputVenueMessageContent`).
    Venue {
        /// Latitude of the venue.
        latitude: f64,
        /// Longitude of the venue.
        longitude: f64,
        /// Name of the venue.
        title: String,
        /// Address of the venue.
        address: String,
        /// Foursquare identifier.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        foursquare_id: Option<String>,
        /// Foursquare type.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        foursquare_type: Option<String>,
        /// Google Places identifier.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        google_place_id: Option<String>,
        /// Google Places type.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        google_place_type: Option<String>,
    },
    /// Contact message content (`InputContactMessageContent`).
    Contact {
        /// Contact's phone number.
        phone_number: String,
        /// Contact's first name.
        first_name: String,
        /// Contact's last name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        /// Additional vCard data.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        vcard: Option<String>,
    },
    /// Invoice message content (`InputInvoiceMessageContent`).
    Invoice {
        /// Product name (1-32 characters).
        title: String,
        /// Product description (1-255 characters).
        description: String,
        /// Bot-defined invoice payload.
        payload: String,
        /// Payment provider token (empty for Telegram Stars `XTR`).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        provider_token: Option<String>,
        /// Three-letter ISO 4217 currency code or `"XTR"`.
        currency: String,
        /// Price breakdown.
        prices: Vec<LabeledPrice>,
    },
}

impl InputMessageContent {
    /// Creates a plain text [`InputMessageContent`].
    pub fn text(message_text: impl Into<String>) -> Self {
        Self::Text {
            message_text: message_text.into(),
            parse_mode: None,
            entities: None,
            link_preview_options: None,
        }
    }
}

/// One result of an inline query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InlineQueryResult {
    /// Link to an article or web page (or widget via `input_message_content`).
    Article {
        /// Unique identifier for this result (1-64 bytes).
        id: String,
        /// Title of the result.
        title: String,
        /// Content of the message to be sent.
        input_message_content: InputMessageContent,
        /// Inline keyboard attached to the message.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        /// URL of the result.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        /// Short description of the result.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// URL of the thumbnail for the result.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thumbnail_url: Option<String>,
        /// Thumbnail width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thumbnail_width: Option<u32>,
        /// Thumbnail height.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thumbnail_height: Option<u32>,
    },
    /// Link to a photo.
    Photo {
        /// Unique identifier for this result.
        id: String,
        /// Valid URL of the photo.
        photo_url: String,
        /// URL of the thumbnail for the photo.
        thumbnail_url: String,
        /// Width of the photo.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        photo_width: Option<u32>,
        /// Height of the photo.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        photo_height: Option<u32>,
        /// Title for the result.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Short description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Caption of the photo to be sent.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Parse mode for caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Inline keyboard attached to the message.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Override message content.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// Link to an animated GIF.
    Gif {
        /// Unique identifier for this result.
        id: String,
        /// Valid URL for the GIF file.
        gif_url: String,
        /// URL of the static or animated thumbnail.
        thumbnail_url: String,
        /// Title for the result.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Caption of the GIF.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Inline keyboard attached to the message.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Override message content.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// Link to a file/document.
    Document {
        /// Unique identifier for this result.
        id: String,
        /// Title for the result.
        title: String,
        /// Valid URL for the file.
        document_url: String,
        /// MIME type of the content (`application/pdf` or `application/zip`).
        mime_type: String,
        /// Caption of the document.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Short description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Inline keyboard attached to the message.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        /// Override message content.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
}
