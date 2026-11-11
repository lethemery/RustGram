//! Bot API 10.1–10.3 Rich Messages, Rich Blocks, tables, expandable quotes, buttons, and thinking blocks.

use serde::{Deserialize, Serialize};

use super::{
    Animation, Audio, Document, InputMedia, JsonMap, Location, MessageEntity, PhotoSize, Video,
    Voice,
};

/// Structured rich formatted message embedded inside a [`Message`](super::Message) (Bot API 10.1+).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RichMessage {
    /// Ordered sequence of layout blocks that form the rich message body.
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
    /// `true`, if the message layout should be displayed right-to-left.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Interactive button inside a [`RichBlock::Buttons`] or [`RichTextButton`] (Bot API 10.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RichMessageButton {
    /// Visible label of the button.
    pub text: String,
    /// Optional callback payload when clicked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Optional URL opened when clicked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// `true`, if the button is rendered in a disabled state (Bot API 10.3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

impl RichMessageButton {
    /// Creates a callback rich message button.
    pub fn callback(text: impl Into<String>, callback_data: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            callback_data: Some(callback_data.into()),
            ..Default::default()
        }
    }

    /// Creates a URL rich message button.
    pub fn url(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    /// Marks the button as disabled (Bot API 10.3).
    pub fn disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = Some(is_disabled);
        self
    }
}

/// Rich inline text-button element (Bot API 10.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RichTextButton {
    /// Text of the button.
    pub text: String,
    /// Optional callback data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Optional URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// `true`, if the button is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}

/// Text span accompanied by optional formatting entities inside rich blocks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RichTextContent {
    /// Plain text content.
    #[serde(default)]
    pub text: String,
    /// Optional message entities describing inline formatting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
}

impl RichTextContent {
    /// Creates a plain [`RichTextContent`].
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            entities: None,
        }
    }
}

/// Individual cell inside a [`RichBlock::Table`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RichBlockTableCell {
    /// Cell text content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Entities inside the cell text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Nested rich blocks if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<RichBlock>>,
    /// `true`, if this cell is a table header cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_header: Option<bool>,
    /// Horizontal alignment (`"left"`, `"center"`, `"right"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    /// Column span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colspan: Option<u32>,
    /// Row span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<u32>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

impl RichBlockTableCell {
    /// Creates a plain body cell.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..Default::default()
        }
    }

    /// Creates a header cell.
    pub fn header(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            is_header: Some(true),
            ..Default::default()
        }
    }
}

/// Individual item inside a [`RichBlock::List`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RichBlockListItem {
    /// Optional marker text (for example `"1."` or `"•"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Item text if simple.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Nested blocks inside the list item.
    #[serde(default)]
    pub blocks: Vec<RichBlock>,
    /// Forward-compatible extra fields.
    #[serde(flatten)]
    pub extra: JsonMap,
}

/// Single structural block in a [`RichMessage`] (Bot API 10.1–10.3).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichBlock {
    /// Standard text paragraph (`RichBlockParagraph`).
    Paragraph {
        /// Paragraph text.
        #[serde(default)]
        text: String,
        /// Inline entities.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        entities: Option<Vec<MessageEntity>>,
    },
    /// Section heading (`RichBlockSectionHeading`).
    SectionHeading {
        /// Heading text.
        #[serde(default)]
        text: String,
        /// Heading level (1-6).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        level: Option<u8>,
        /// Inline entities.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        entities: Option<Vec<MessageEntity>>,
    },
    /// Preformatted code block (`RichBlockPreformatted`).
    Preformatted {
        /// Source or preformatted text.
        #[serde(default)]
        text: String,
        /// Optional syntax highlighting language.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        language: Option<String>,
    },
    /// Footer block (`RichBlockFooter`).
    Footer {
        /// Footer text.
        #[serde(default)]
        text: String,
        /// Inline entities.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        entities: Option<Vec<MessageEntity>>,
    },
    /// Horizontal visual separator (`RichBlockDivider`).
    Divider,
    /// Standalone mathematical expression (`RichBlockMathematicalExpression`).
    MathematicalExpression {
        /// LaTeX / math expression string.
        #[serde(default)]
        expression: String,
    },
    /// Navigation anchor point (`RichBlockAnchor`).
    Anchor {
        /// Anchor identifier.
        name: String,
        /// Optional blocks wrapped by the anchor.
        #[serde(default)]
        blocks: Vec<RichBlock>,
    },
    /// Ordered or unordered list (`RichBlockList`).
    List {
        /// `true` if ordered, `false` if unordered.
        #[serde(default)]
        is_ordered: bool,
        /// List items.
        #[serde(default)]
        items: Vec<RichBlockListItem>,
    },
    /// Block quotation (`RichBlockBlockQuotation`).
    BlockQuotation {
        /// Quoted blocks.
        #[serde(default)]
        blocks: Vec<RichBlock>,
        /// Optional plain text fallback.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    /// Collapsible/expandable block quotation (`RichBlockExpandableBlockQuotation`, Bot API 10.3).
    ExpandableBlockQuotation {
        /// Quoted blocks.
        #[serde(default)]
        blocks: Vec<RichBlock>,
        /// Optional plain text fallback.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    /// Pull quotation (`RichBlockPullQuotation`).
    PullQuotation {
        /// Quoted text.
        #[serde(default)]
        text: String,
        /// Optional author / attribution.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        author: Option<String>,
    },
    /// Styled data table (`RichBlockTable`, Bot API 10.3 adds `is_compact`).
    Table {
        /// Table caption if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Rows of table cells.
        #[serde(default)]
        rows: Vec<Vec<RichBlockTableCell>>,
        /// `true`, if the table uses compact spacing (Bot API 10.3).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        is_compact: Option<bool>,
    },
    /// Expandable `<details>` disclosure block (`RichBlockDetails`).
    Details {
        /// Summary header text.
        #[serde(default)]
        summary: String,
        /// Inner collapsible blocks.
        #[serde(default)]
        blocks: Vec<RichBlock>,
        /// `true`, if expanded by default.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        is_open: Option<bool>,
    },
    /// Native AI agent thinking/reasoning block (`RichBlockThinking`).
    Thinking {
        /// Title or status label (e.g. "Thinking...").
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Inner thought text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        /// Optional nested blocks inside the thinking section.
        #[serde(default)]
        blocks: Vec<RichBlock>,
        /// Duration of thinking in seconds, if finished.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
    },
    /// Inline buttons row/grid inside the rich message (`RichBlockButtons`, Bot API 10.3).
    Buttons {
        /// Rows of buttons.
        #[serde(default)]
        buttons: Vec<Vec<RichMessageButton>>,
    },
    /// Document attachment block (`RichBlockDocument`, Bot API 10.3).
    Document {
        /// Document object.
        document: Document,
        /// Optional caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    /// Photo block (`RichBlockPhoto`).
    Photo {
        /// Available photo sizes.
        photo: Vec<PhotoSize>,
        /// Optional caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    /// Video block (`RichBlockVideo`).
    Video {
        /// Video metadata.
        video: Video,
        /// Optional caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    /// Audio block (`RichBlockAudio`).
    Audio {
        /// Audio metadata.
        audio: Audio,
        /// Optional caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    /// Animation block (`RichBlockAnimation`).
    Animation {
        /// Animation metadata.
        animation: Animation,
        /// Optional caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    /// Voice note block (`RichBlockVoiceNote`).
    VoiceNote {
        /// Voice message metadata.
        voice: Voice,
        /// Optional caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    /// Media collage block (`RichBlockCollage`).
    Collage {
        /// Nested media blocks.
        #[serde(default)]
        blocks: Vec<RichBlock>,
        /// Optional caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    /// Media slideshow block (`RichBlockSlideshow`).
    Slideshow {
        /// Nested slide blocks.
        #[serde(default)]
        blocks: Vec<RichBlock>,
        /// Optional caption.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    /// Interactive map block (`RichBlockMap`).
    Map {
        /// Map coordinates.
        location: Location,
        /// Optional map label.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// Unrecognized or unsupported block (`RichBlockUnsupported`).
    #[serde(other)]
    Unsupported,
}

/// Explicit media mapping referenced from Markdown/HTML inside [`InputRichMessage`] (Bot API 10.2).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InputRichMessageMedia {
    /// Identifier referenced inside the rich payload (for example via `tg://document?id=<id>`).
    pub id: String,
    /// Media item bound to this identifier.
    pub media: InputMedia,
}

impl InputRichMessageMedia {
    /// Creates a new [`InputRichMessageMedia`] binding.
    pub fn new(id: impl Into<String>, media: InputMedia) -> Self {
        Self {
            id: id.into(),
            media,
        }
    }
}

/// Payload used when sending or drafting a rich message (Bot API 10.1–10.3).
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct InputRichMessage {
    /// Rich content authored in Telegram rich HTML format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Rich content authored in Telegram rich Markdown format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    /// Explicit sequence of rich blocks (`InputRichBlock*`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<RichBlock>>,
    /// Explicit media bindings referenced by tags in `html`/`markdown` (Bot API 10.2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<InputRichMessageMedia>>,
    /// Pass `true` if the rich message must be shown right-to-left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
    /// Pass `true` to disable automatic detection of URLs, usernames, and phone numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_entity_detection: Option<bool>,
}

impl InputRichMessage {
    /// Creates an [`InputRichMessage`] from a Markdown string.
    pub fn markdown(markdown: impl Into<String>) -> Self {
        Self {
            markdown: Some(markdown.into()),
            ..Default::default()
        }
    }

    /// Creates an [`InputRichMessage`] from an HTML string.
    pub fn html(html: impl Into<String>) -> Self {
        Self {
            html: Some(html.into()),
            ..Default::default()
        }
    }

    /// Creates an [`InputRichMessage`] from structured [`RichBlock`] items.
    pub fn blocks(blocks: Vec<RichBlock>) -> Self {
        Self {
            blocks: Some(blocks),
            ..Default::default()
        }
    }

    /// Attaches explicit media mappings (Bot API 10.2).
    pub fn media(mut self, media: Vec<InputRichMessageMedia>) -> Self {
        self.media = Some(media);
        self
    }

    /// Sets right-to-left layout direction.
    pub fn rtl(mut self, is_rtl: bool) -> Self {
        self.is_rtl = Some(is_rtl);
        self
    }

    /// Disables automatic entity detection.
    pub fn skip_entity_detection(mut self, skip: bool) -> Self {
        self.skip_entity_detection = Some(skip);
        self
    }
}
