//! Media attachments, polls, reactions, paid media, and gifts.

use serde::{Deserialize, Serialize};

use super::{Chat, JsonMap, MessageEntity, ParseMode, User};

/// One size of a photo or file/sticker thumbnail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier for this file.
    pub file_unique_id: String,
    /// Photo width.
    pub width: u32,
    /// Photo height.
    pub height: u32,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Animation {
    /// Identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file.
    pub file_unique_id: String,
    /// Video width.
    pub width: u32,
    /// Video height.
    pub height: u32,
    /// Duration of the video in seconds.
    pub duration: u32,
    /// Animation thumbnail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Original animation filename.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Audio file to be treated as music by Telegram clients.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audio {
    /// Identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds.
    pub duration: u32,
    /// Performer of the audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Title of the audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Original filename.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    /// Thumbnail of the album cover.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

/// General file (as opposed to photos, voice messages, and audio files).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    /// Identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file.
    pub file_unique_id: String,
    /// Document thumbnail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Original filename.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Video file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Video {
    /// Identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file.
    pub file_unique_id: String,
    /// Video width.
    pub width: u32,
    /// Video height.
    pub height: u32,
    /// Duration of the video in seconds.
    pub duration: u32,
    /// Video thumbnail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Available sizes of the cover of the video in the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<Vec<PhotoSize>>,
    /// Timestamp in seconds from which the video will play.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<u32>,
    /// Original filename.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoNote {
    /// Identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message).
    pub length: u32,
    /// Duration of the video in seconds.
    pub duration: u32,
    /// Video thumbnail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Voice note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Voice {
    /// Identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds.
    pub duration: u32,
    /// MIME type of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Sticker mask position on faces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaskPosition {
    /// Part of the face relative to which the mask should be placed (`forehead`, `eyes`, `mouth`, `chin`).
    pub point: String,
    /// Shift by X-axis measured in widths of the mask.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask.
    pub y_shift: f64,
    /// Mask scaling coefficient.
    pub scale: f64,
}

/// Sticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sticker {
    /// Identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file.
    pub file_unique_id: String,
    /// Type of the sticker (`regular`, `mask`, `custom_emoji`).
    #[serde(rename = "type")]
    pub sticker_type: String,
    /// Sticker width.
    pub width: u32,
    /// Sticker height.
    pub height: u32,
    /// `true`, if the sticker is animated.
    pub is_animated: bool,
    /// `true`, if the sticker is a video sticker.
    pub is_video: bool,
    /// Sticker thumbnail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Emoji associated with the sticker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Name of the sticker set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    /// For premium regular stickers, premium animation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<super::File>,
    /// For mask stickers, position where the mask should be placed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// For custom emoji stickers, unique identifier of the custom emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// `true`, if the sticker must be repainted to a text color in messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Phone contact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    /// Contact's phone number.
    pub phone_number: String,
    /// Contact's first name.
    pub first_name: String,
    /// Contact's last name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Additional data about the contact in vCard format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

/// Animated emoji displaying a random value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based.
    pub emoji: String,
    /// Value of the dice.
    pub value: u8,
}

/// Point on the map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Latitude as defined by sender.
    pub latitude: f64,
    /// Longitude as defined by sender.
    pub longitude: f64,
    /// Uncertainty radius in meters (0-1500).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Time relative to message sending during which the location can be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    /// Direction in which user is moving, in degrees (1-360).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// Maximum distance for proximity alerts about approaching another member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
}

/// Venue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    /// Venue location.
    pub location: Location,
    /// Name of the venue.
    pub title: String,
    /// Address of the venue.
    pub address: String,
    /// Foursquare identifier of the venue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

/// Poll answer option.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PollOption {
    /// Option text (1-100 characters).
    pub text: String,
    /// Special entities that appear in the option text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Number of users that voted for this option.
    pub voter_count: u64,
}

/// Poll option passed when sending a poll.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputPollOption {
    /// Option text (1-100 characters).
    pub text: String,
    /// Mode for parsing entities in the text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<ParseMode>,
    /// Special entities that appear in the option text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

impl InputPollOption {
    /// Creates a plain text poll option.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            text_parse_mode: None,
            text_entities: None,
        }
    }
}

/// Poll object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,
    /// Poll question (1-300 characters).
    pub question: String,
    /// Special entities that appear in the question.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
    /// List of poll options.
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll.
    pub total_voter_count: u64,
    /// `true`, if the poll is closed.
    pub is_closed: bool,
    /// `true`, if the poll is anonymous.
    pub is_anonymous: bool,
    /// Poll type (`"regular"` or `"quiz"`).
    #[serde(rename = "type")]
    pub poll_type: String,
    /// `true`, if the poll allows multiple answers.
    pub allows_multiple_answers: bool,
    /// 0-based identifier of the correct answer option for quizzes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<u32>,
    /// Text shown when a user chooses an incorrect answer in a quiz.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Special entities in the explanation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u32>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}

/// Answer of a user in a non-anonymous poll.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: String,
    /// Chat that changed the answer to the poll if the voter is anonymous.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,
    /// User that changed the answer to the poll.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// 0-based identifiers of chosen answer options.
    pub option_ids: Vec<u32>,
}

/// Type of a reaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    /// Standard emoji reaction.
    Emoji {
        /// Reaction emoji.
        emoji: String,
    },
    /// Custom emoji reaction.
    CustomEmoji {
        /// Custom emoji identifier.
        custom_emoji_id: String,
    },
    /// Paid Telegram Stars reaction.
    Paid,
}

impl ReactionType {
    /// Creates a standard emoji reaction.
    pub fn emoji(emoji: impl Into<String>) -> Self {
        Self::Emoji {
            emoji: emoji.into(),
        }
    }

    /// Creates a custom emoji reaction.
    pub fn custom_emoji(custom_emoji_id: impl Into<String>) -> Self {
        Self::CustomEmoji {
            custom_emoji_id: custom_emoji_id.into(),
        }
    }
}

/// Represents a reaction added to a message along with the number of times it was added.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionCount {
    /// Type of the reaction.
    #[serde(rename = "type")]
    pub reaction_type: ReactionType,
    /// Number of times the reaction was added.
    pub total_count: u64,
}

/// Change of a reaction on a message with non-anonymous reactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionUpdated {
    /// Chat containing the message.
    pub chat: Chat,
    /// Unique identifier of the message inside the chat.
    pub message_id: super::MessageId,
    /// User that changed the reaction, if the user isn't anonymous.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Chat on behalf of which the reaction was changed, if the user is anonymous.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
    /// Date of the change in Unix time.
    pub date: i64,
    /// Previous list of reaction types set by the user.
    pub old_reaction: Vec<ReactionType>,
    /// New list of reaction types set by the user.
    pub new_reaction: Vec<ReactionType>,
}

/// Reactions changes on a message with anonymous reactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    /// Chat containing the message.
    pub chat: Chat,
    /// Unique message identifier inside the chat.
    pub message_id: super::MessageId,
    /// Date of the change in Unix time.
    pub date: i64,
    /// List of reactions present on the message.
    pub reactions: Vec<ReactionCount>,
}

/// Paid media attached to a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaidMedia {
    /// Paid media preview.
    Preview {
        /// Media width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        /// Media height.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
        /// Duration of the media in seconds.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
    },
    /// Paid photo.
    Photo {
        /// The photo.
        photo: Vec<PhotoSize>,
    },
    /// Paid video.
    Video {
        /// The video.
        video: Box<Video>,
    },
}

/// Information about paid media.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaInfo {
    /// Number of Telegram Stars that must be paid to buy access to the media.
    pub star_count: u64,
    /// Information about the paid media.
    pub paid_media: Vec<PaidMedia>,
}

/// Information about a paid media purchase by a user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaidMediaPurchased {
    /// User who purchased the media.
    pub from: User,
    /// Bot-specified paid media payload.
    pub paid_media_payload: String,
}

/// Media item used in `sendMediaGroup` or `editMessageMedia`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia {
    /// Photo media item.
    Photo {
        /// File reference (`file_id`, URL, or `attach://<name>`).
        media: String,
        /// Optional caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Parse mode for caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Caption entities.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Show caption above media.
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        /// Pass `true` to cover with spoiler animation.
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    /// Video media item.
    Video {
        /// File reference (`file_id`, URL, or `attach://<name>`).
        media: String,
        /// Optional thumbnail reference.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        /// Optional cover reference.
        #[serde(skip_serializing_if = "Option::is_none")]
        cover: Option<String>,
        /// Start timestamp in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        start_timestamp: Option<u32>,
        /// Optional caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Parse mode for caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Caption entities.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Show caption above media.
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        /// Video width.
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        /// Video height.
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
        /// Video duration in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        /// Pass `true` if suitable for streaming.
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
        /// Pass `true` to cover with spoiler animation.
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    /// Animation (GIF/H.264) media item.
    Animation {
        /// File reference.
        media: String,
        /// Optional thumbnail reference.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        /// Optional caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Parse mode for caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Caption entities.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Show caption above media.
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        /// Animation width.
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        /// Animation height.
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
        /// Animation duration.
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        /// Pass `true` to cover with spoiler animation.
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    /// Audio media item.
    Audio {
        /// File reference.
        media: String,
        /// Optional thumbnail reference.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        /// Optional caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Parse mode for caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Caption entities.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Duration in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        /// Performer.
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        /// Track title.
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// Document media item.
    Document {
        /// File reference.
        media: String,
        /// Optional thumbnail reference.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        /// Optional caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Parse mode for caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Caption entities.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Disable automatic server-side content type detection.
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_content_type_detection: Option<bool>,
    },
}

/// Paid media item used in `sendPaidMedia`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPaidMedia {
    /// Paid photo item.
    Photo {
        /// File reference (`file_id`, URL, or `attach://<name>`).
        media: String,
    },
    /// Paid video item.
    Video {
        /// File reference.
        media: String,
        /// Optional thumbnail reference.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        /// Optional cover reference.
        #[serde(skip_serializing_if = "Option::is_none")]
        cover: Option<String>,
        /// Start timestamp in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        start_timestamp: Option<u32>,
        /// Video width.
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        /// Video height.
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
        /// Duration in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        /// Supports streaming.
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
    },
}

/// Telegram gift object (Bot API 9.2 adds `publisher_chat`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gift {
    /// Unique identifier of the gift.
    pub id: String,
    /// The sticker that represents the gift.
    pub sticker: Sticker,
    /// Number of Telegram Stars required to send the gift.
    pub star_count: u64,
    /// Number of Telegram Stars required to upgrade the gift to a unique gift.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrade_star_count: Option<u64>,
    /// Total number of gifts of this type that can be sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    /// Number of remaining gifts of this type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<u64>,
    /// Information about the chat that published the gift (Bot API 9.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Chat>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Unique collectible Telegram gift (Bot API 9.2 adds `publisher_chat`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueGift {
    /// Human-readable name of the regular gift from which this unique gift was upgraded.
    pub base_name: String,
    /// Unique name of the gift (can be used to construct `t.me/nft/...`).
    pub name: String,
    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift.
    pub number: u64,
    /// Information about the chat that published the gift (Bot API 9.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Chat>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}
