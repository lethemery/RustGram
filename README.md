# rustgram

Async, strongly-typed Rust library and framework for building Telegram bots targeting **Telegram Bot API 10.4** (`rustgram::BOT_API_VERSION == "10.4"`).

**[Documentation Site (GitHub Pages)](docs/index.html)** · **[English Manual](docs/en/README.md)** · **[Документация на русском](docs/ru/README.md)**

---

## Highlights

- **Full Telegram Bot API 10.4 Coverage:**
  - **HTML Widgets & TON Wallet (10.4):** `Widget`, `WidgetQuery`, `SharedWidgetQuery`, `WidgetQueryType` (`Data`, `SignData`, `SentTransaction`), `WidgetQueryWallet`, `send_widget`, `send_shared_widget`, `answer_widget_query`, `edit_message_widget_data`, `edit_inline_widget_data`, `edit_shared_widget_data`, inline widget payloads (`InputWidgetMessageContent`, `InputSharedWidgetMessageContent`), plus local HTML (`32,768` bytes), data (`1,024` bytes), and `cache_time` (`5..=3600`s) validators.
  - **Rich Messages & Stoppable Streaming Drafts (10.1–10.3):** `RichMessage`, `InputRichMessage`, 25+ `RichBlock` variants (`Thinking`, compact `Table`, `ExpandableBlockQuotation`, `Details`, `Buttons` with `disabled`, etc.), `send_rich_message`, `send_rich_message_draft`, `send_message_draft` with `.can_stop(bool)` / `.keep_on_stop(bool)`, and `MessageGenerationStopped` updates.
  - **Ephemeral Messages (10.2–10.4):** `EphemeralMessageParameters::for_user` and `for_callback` (Bot API 10.4 callback targeting without `receiver_user_id`), `BotCommand::ephemeral`, `edit_ephemeral_message_*`, `delete_ephemeral_message`.
  - **Guest Mode, Communities & Subscriptions (10.0–10.2):** `Update.guest_message`, `answer_guest_query`, `BotSubscriptionUpdated`.
  - **Managed Bots & Modern Chat Tools (9.4–10.4):** `KeyboardButtonRequestManagedBot`, `ManagedBotUpdated`, `get_managed_bot_token`, `replace_managed_bot_token`, `get_managed_bot_access_settings`, `set_managed_bot_access_settings`, `get_user_personal_chat_messages`, `.icon_custom_emoji(...)` on buttons, `date_time` message entities, `set_chat_member_tag`, `answer_chat_join_request_query`, `send_checklist`, and private-chat forum topics.
- **Ergonomic Async Client:** `Bot` and `BotBuilder` supporting JSON and `multipart/form-data`, Local Bot API `base_url`, Telegram test environments, configurable HTTP 429 retries, and `IntoFuture` request builders.
- **Composable Routing & FSM:** `Router`, middleware layers (`.layer`), nested routers (`.nest`), typed DI (`Dispatcher::manage`), `MemoryStorage` + pluggable `StateStorage<S>`, and `on::*` filters.
- **Polling, Webhooks & Mini Apps:** Resilient `Polling` runner, optional `axum` webhook server (`webhook` feature), Telegram Mini App `initData` HMAC-SHA256 verification (`webapp` feature), and HTML/MarkdownV2 formatting helpers.

---

## Installation

```toml
[dependencies]
rustgram = { git = "https://github.com/<your-username>/RustGram" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Cargo features:
- `rustls-tls` *(default)* — TLS via `rustls` + `ring`.
- `webhook` *(default)* — Axum webhook router & server (`rustgram::webhook`).
- `webapp` *(default)* — Mini App `initData` HMAC-SHA256 verifier (`rustgram::webapp`).

---

## Quick Start

```rust,no_run
use rustgram::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = Bot::from_env()?;

    let router = Router::new()
        .route(on::command("start"), |ctx: Context| async move {
            ctx.reply("Hello from rustgram (Bot API 10.4)!").await?;
            Ok(())
        })
        .route(on::text(), |ctx: Context| async move {
            if let Some(msg) = ctx.message()
                && let Some(text) = &msg.text
            {
                ctx.reply(text).await?;
            }
            Ok(())
        });

    Dispatcher::new(bot, router).run_polling().await;
    Ok(())
}
```

### Interactive HTML Widget (Bot API 10.4)

```rust,no_run
use rustgram::prelude::*;

async fn send_ton_widget(bot: &Bot, chat_id: i64) -> Result<()> {
    let html = r#"<button onclick="TelegramWidget.request('pay')">Send TON</button>"#;
    bot.send_widget(chat_id, html)
        .size(320, 180)
        .shared(true)
        .cache_time(60)
        .await?;
    Ok(())
}
```

---

## Examples & Local Verification

| Example | Description |
| --- | --- |
| [`examples/echo_bot.rs`](examples/echo_bot.rs) | Minimal polling echo bot |
| [`examples/widget_bot.rs`](examples/widget_bot.rs) | Bot API 10.4 HTML widgets & TON wallet callbacks |
| [`examples/rich_streaming_bot.rs`](examples/rich_streaming_bot.rs) | Stoppable streaming drafts, `RichMessage`, and ephemeral messages |
| [`examples/fsm_webhook_bot.rs`](examples/fsm_webhook_bot.rs) | Multi-step FSM conversation over Axum webhook |

Run the full check suite locally:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo check --no-default-features --features rustls-tls --lib --examples
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo test --all-targets
```

---

## Documentation, GitHub Pages & Publishing

- **Bilingual Web Portal:** [`docs/index.html`](docs/index.html) (switchable EN / RU, light/dark themes, instant search, code copy buttons).
- **Full Guides:** [English](docs/en/README.md) · [Русский](docs/ru/README.md).
- **GitHub Pages Automation:** [`.github/workflows/docs.yml`](.github/workflows/docs.yml) builds both `docs/` and Rustdoc (`/api/rustgram/index.html`). Enable via **Settings → Pages → Source: GitHub Actions**.
- **CI Checks:** [`.github/workflows/ci.yml`](.github/workflows/ci.yml).

---

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE))
- MIT License ([`LICENSE-MIT`](LICENSE-MIT))

at your option.
