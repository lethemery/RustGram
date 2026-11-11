//! Text escaping and formatting helpers for Telegram HTML, MarkdownV2, custom emojis, and Bot API 9.5 date-time tags.

use std::fmt::Write;

use crate::types::UserId;

/// Escapes `<`, `>`, `&`, and `"` for safe inclusion inside Telegram `HTML` messages.
pub fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            other => out.push(other),
        }
    }
    out
}

/// Escapes all reserved characters for Telegram `MarkdownV2` text.
pub fn escape_markdown_v2(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 8);
    for ch in input.chars() {
        if matches!(
            ch,
            '_' | '*'
                | '['
                | ']'
                | '('
                | ')'
                | '~'
                | '`'
                | '>'
                | '#'
                | '+'
                | '-'
                | '='
                | '|'
                | '{'
                | '}'
                | '.'
                | '!'
                | '\\'
        ) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

/// Formats bold HTML text (`<b>...</b>`), escaping `text`.
pub fn bold_html(text: &str) -> String {
    format!("<b>{}</b>", escape_html(text))
}

/// Formats italic HTML text (`<i>...</i>`), escaping `text`.
pub fn italic_html(text: &str) -> String {
    format!("<i>{}</i>", escape_html(text))
}

/// Formats inline code HTML (`<code>...</code>`), escaping `text`.
pub fn code_html(text: &str) -> String {
    format!("<code>{}</code>", escape_html(text))
}

/// Formats a syntax-highlighted `<pre><code class="language-...">...</code></pre>` HTML block.
pub fn pre_html(code: &str, language: Option<&str>) -> String {
    match language {
        Some(lang) if !lang.is_empty() => format!(
            "<pre><code class=\"language-{}\">{}</code></pre>",
            escape_html(lang),
            escape_html(code)
        ),
        _ => format!("<pre>{}</pre>", escape_html(code)),
    }
}

/// Formats a hyperlink (`<a href="...">label</a>`), escaping both URL and label.
pub fn link_html(url: &str, label: &str) -> String {
    format!(
        "<a href=\"{}\">{}</a>",
        escape_html(url),
        escape_html(label)
    )
}

/// Formats an inline user mention (`<a href="tg://user?id=...">label</a>`).
pub fn mention_html(user_id: impl Into<UserId>, label: &str) -> String {
    let uid = user_id.into();
    format!(
        "<a href=\"tg://user?id={}\">{}</a>",
        uid.get(),
        escape_html(label)
    )
}

/// Formats a custom emoji tag (`<tg-emoji emoji-id="...">fallback</tg-emoji>`, Bot API 9.4/10.4).
pub fn custom_emoji_html(custom_emoji_id: &str, fallback_emoji: &str) -> String {
    format!(
        "<tg-emoji emoji-id=\"{}\">{}</tg-emoji>",
        escape_html(custom_emoji_id),
        escape_html(fallback_emoji)
    )
}

/// Formats a localized date-time tag (`<tg-time unix="..." format="...">fallback</tg-time>`, Bot API 9.5).
pub fn date_time_html(unix_time: i64, format_spec: Option<&str>, fallback_text: &str) -> String {
    let mut out = String::new();
    let _ = write!(out, "<tg-time unix=\"{unix_time}\"");
    if let Some(fmt) = format_spec {
        let _ = write!(out, " format=\"{}\"", escape_html(fmt));
    }
    let _ = write!(out, ">{}</tg-time>", escape_html(fallback_text));
    out
}
