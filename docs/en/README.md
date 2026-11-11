# rustgram — Complete Documentation (English)

> Async, strongly-typed Telegram Bot library and framework for Rust targeting **Telegram Bot API 10.4** (`rustgram::BOT_API_VERSION == "10.4"`).

- [Interactive Bilingual Docs Site](../index.html)
- [Документация на русском языке](../ru/README.md)
- [Repository Root README](../../README.md)

---

## Table of Contents

1. [Overview & Architecture](#1-overview--architecture)
2. [Installation & Cargo Features](#2-installation--cargo-features)
3. [Bot Client, Files & Error Handling](#3-bot-client-files--error-handling)
4. [Bot API 10.4: HTML Widgets & TON Wallet](#4-bot-api-104-html-widgets--ton-wallet)
5. [Bot API 10.1–10.3: Rich Messages & Streaming Drafts](#5-bot-api-101103-rich-messages--streaming-drafts)
6. [Bot API 9.4–10.4: Ephemeral, Guest Mode, Managed Bots & Communities](#6-bot-api-94104-ephemeral-guest-mode-managed-bots--communities)
7. [Router, Filters, Middleware & Dependency Injection](#7-router-filters-middleware--dependency-injection)
8. [Finite State Machine (FSM)](#8-finite-state-machine-fsm)
9. [Polling, Axum Webhooks, Mini Apps & Formatting](#9-polling-axum-webhooks-mini-apps--formatting)
10. [Examples & Verification Commands](#10-examples--verification-commands)

---

## 1. Overview & Architecture

`rustgram` separates transport, Telegram Bot API protocol models, request builders, and update dispatching into focused modules:

| Module | Responsibility |
| --- | --- |
| `rustgram::client` | `Bot`, `BotBuilder`, URL routing (`method_url` / `file_url`), JSON & `multipart/form-data` execution, automatic 429 retry logic, file downloads. |
| `rustgram::types` | Serde models covering Telegram Bot API 9.4 through 10.4 (`Update`, `Message`, `Widget`, `WidgetQuery`, `RichMessage`, `RichBlock`, `EphemeralMessageParameters`, etc.), compactly boxed to stay under `< 4096` bytes on the stack. |
| `rustgram::requests` | Ergonomic builders implementing `IntoFuture<Output = Result<T>>` across messaging, files, chats, managed bots, updates/queries, widgets, and rich/ephemeral endpoints. |
| `rustgram::router` | Composable `Router`, `Dispatcher`, `Context`, middleware (`Next`), `on::*` filters, and FSM (`StateStorage<S>`, `MemoryStorage<S>`, `StateContext<S>`). |
| `rustgram::polling` | Resilient `getUpdates` stream and runner with exponential backoff and `CancellationToken` shutdown. |
| `rustgram::webhook` *(feature `webhook`)* | Axum `Router` builder, `serve_webhook`, `SECRET_TOKEN_HEADER`, and `parse_webhook_request`. |
| `rustgram::webapp` *(feature `webapp`)* | Constant-time-safe HMAC-SHA256 `verify_init_data` verifier for Telegram Mini Apps. |
| `rustgram::formatting` | HTML and MarkdownV2 escaping plus tags including `custom_emoji_html` and `date_time_html`. |
| `rustgram::error` | `Error`, `ApiError`, and `ResponseParameters` with helper predicates (`is_rate_limited`, `retry_after`, `migrate_to_chat_id`, etc.). |

Import everything commonly used via:

```rust,no_run
use rustgram::prelude::*;
```

---

## 2. Installation & Cargo Features

Add `rustgram` and `tokio` to your `Cargo.toml`:

```toml
[dependencies]
rustgram = { git = "https://github.com/lethemery/RustGram" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

To build a minimal polling-only bot without `axum` webhooks or `webapp` verification:

```toml
[dependencies]
rustgram = { git = "https://github.com/lethemery/RustGram", default-features = false, features = ["rustls-tls"] }
```

### Feature Matrix

| Feature | Default | Description |
| --- | --- | --- |
| `rustls-tls` | Yes | Uses pure-Rust `rustls` + `ring` TLS inside `reqwest`. |
| `webhook` | Yes | Enables `rustgram::webhook` powered by `axum`. |
| `webapp` | Yes | Enables `rustgram::webapp` HMAC-SHA256 verification powered by `hmac`, `sha2`, `hex`, and `url`. |

---

## 3. Bot Client, Files & Error Handling

### Creating a Client

```rust,no_run
use std::time::Duration;
use rustgram::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. From environment variable TELEGRAM_BOT_TOKEN
    let bot_from_env = Bot::from_env()?;

    // 2. Programmatically via validated constructor
    let bot_validated = Bot::try_new("123456:ABC-DEF")?;

    // 3. Full builder configuration (Local Bot API, test env, rate-limit retries)
    let bot_custom = Bot::builder("123456:ABC-DEF")
        .base_url("https://api.telegram.org")
        .test_environment(false)
        .timeout(Duration::from_secs(60))
        .retry_rate_limits(3, Duration::from_secs(15))
        .build()?;

    let me = bot_from_env.get_me().await?;
    println!("Bot @{} is online (test_env={})", me.username.unwrap_or_default(), bot_custom.is_test_environment());
    let _ = bot_validated;
    Ok(())
}
```

### Sending & Downloading Files

`InputFile` transparently chooses JSON or `multipart/form-data` based on the variant:

- `InputFile::file_id("...")` — existing Telegram file ID (JSON).
- `InputFile::url("https://...")` — remote HTTP URL (JSON).
- `InputFile::memory("report.txt", bytes)` — in-memory payload (multipart).
- `InputFile::path("./chart.png")` — local filesystem upload (multipart).

Download helpers on `Bot`:

```rust,no_run
use rustgram::prelude::*;

async fn download_asset(bot: &Bot, file_id: &str) -> Result<bytes::Bytes> {
    bot.download_file_by_id(file_id).await
}
```

### Structured Error Inspection

When Telegram returns an error JSON object (`ok: false`), `rustgram` surfaces `Error::Api(ApiError)`:

- `api_err.error_code` — HTTP-style status code returned by Telegram (e.g. `429`, `403`, `400`).
- `api_err.description` — human-readable description.
- `api_err.parameters` — optional `ResponseParameters` (`retry_after`, `migrate_to_chat_id`).
- `api_err.is_rate_limited()` / `api_err.retry_after()` — inspect HTTP 429 retry cooldowns.
- `api_err.migrate_to_chat_id()` — detect group-to-supergroup migration targets.
- `api_err.is_forbidden()` — user blocked the bot or bot lacks permissions.
- `api_err.is_not_modified()` — message edit payload matched existing message content.

---

## 4. Bot API 10.4: HTML Widgets & TON Wallet

Bot API 10.4 adds interactive HTML widgets rendered inside Telegram messages plus native TON wallet hooks (`SignData` and `SentTransaction`).

### Built-in Invariant Validation

Before executing HTTP requests, `rustgram` locally checks:

- `WIDGET_HTML_MAX_BYTES` (`32_768` bytes) and absence of `\0` NUL bytes via `validate_widget_html`.
- `WIDGET_DATA_MAX_BYTES` (`1_024` bytes) for `public_data` and `chat_data` via `validate_widget_data`.
- `clamp_widget_cache_time` clamps cache lifetime into `[WIDGET_CACHE_TIME_MIN (5) ..= WIDGET_CACHE_TIME_MAX (3_600)]` (default `WIDGET_CACHE_TIME_DEFAULT = 30`).

### Sending & Updating Widgets

```rust,no_run
use rustgram::prelude::*;

async fn widget_handlers(ctx: Context) -> Result<()> {
    // Send a shared interactive HTML widget from handler context
    let html = r#"
      <button onclick="TelegramWidget.request('checkout')">Pay 1 TON</button>
    "#;
    ctx.send_widget(html)?
        .size(320, 180)
        .shared(true)
        .public_data(r#"{"price":"1 TON"}"#)
        .chat_data(r#"{"clicks":0}"#)
        .cache_time(60)
        .caption("TON Checkout")
        .await?;

    // Later, reuse an existing shared widget ID
    if let Some(chat_id) = ctx.chat_id() {
        ctx.bot.send_shared_widget(chat_id, "shared_widget_id").await?;
    }
    Ok(())
}
```

### Handling Widget & TON Wallet Queries

Route `on::widget_query()`, `on::shared_widget_query()`, or `on::any_widget_query()`, as well as type-specific filters (`on::widget_data()`, `on::widget_sign_data()`, `on::widget_sent_transaction()`):

```rust,no_run
use rustgram::prelude::*;

fn build_widget_router() -> Router {
    Router::new().route(on::any_widget_query(), |ctx: Context| async move {
        if let Some(wq) = ctx.widget_query() {
            match &wq.query_type {
                WidgetQueryType::Data { with_wallet, wallet } => {
                    let _ = (with_wallet, wallet);
                    ctx.answer_widget()?.data(r#"{"clicks":1}"#).await?;
                }
                WidgetQueryType::SignData { wallet, sign_data } => {
                    let _ = (wallet, sign_data);
                    ctx.answer_widget()?.data(r#"{"verified":true}"#).await?;
                }
                WidgetQueryType::SentTransaction { wallet, msg_hash } => {
                    if let Some(ref msg) = wq.message {
                        ctx.bot
                            .edit_message_widget_data(msg.chat.id, msg.message_id)
                            .chat_data(format!(r#"{{"tx":"{msg_hash}"}}"#))
                            .await?;
                    }
                    let _addr = &wallet.address;
                    ctx.answer_widget()?.data(r#"{"paid":true}"#).await?;
                }
                WidgetQueryType::Unknown => {}
            }
        }
        Ok(())
    })
}
```

Other widget endpoints:
- `bot.edit_inline_widget_data(inline_message_id)`
- `bot.edit_shared_widget_data(widget_id)`
- Inline results: `InputWidgetMessageContent`, `InputSharedWidgetMessageContent`, `InputMessageContent::Widget`, `InputMessageContent::SharedWidget`.

---

## 5. Bot API 10.1–10.3: Rich Messages & Streaming Drafts

Compose rich structured layouts using `InputRichMessage::html(...)`, `InputRichMessage::markdown(...)`, or explicit semantic blocks (`InputRichMessage::blocks(...)`):

Supported `RichBlock` variants include `Paragraph`, `SectionHeading`, `Preformatted`, `Footer`, `Divider`, `MathematicalExpression`, `Anchor`, `List`, `BlockQuotation`, `ExpandableBlockQuotation`, `PullQuotation`, `Table` (`is_compact`), `Details`, `Thinking`, `Buttons` (with `RichMessageButton::disabled`), `Document`, `Photo`, `Video`, `Audio`, `Animation`, `VoiceNote`, `Collage`, `Slideshow`, `Map`, and forward-compatible `Unsupported`.

```rust,no_run
use rustgram::prelude::*;
use rustgram::types::{InputRichMessage, RichBlock, RichBlockTableCell, RichMessageButton};

async fn stream_ai_answer(bot: &Bot, chat_id: i64, draft_id: i64) -> Result<()> {
    // Stream stoppable draft (Bot API 10.3)
    bot.send_message_draft(chat_id, draft_id, "Thinking...")
        .can_stop(true)
        .keep_on_stop(true)
        .await?;

    // Send final RichMessage
    let rich = InputRichMessage::blocks(vec![
        RichBlock::SectionHeading {
            text: "Summary".into(),
            level: Some(2),
            entities: None,
        },
        RichBlock::Table {
            caption: Some("Metrics".into()),
            is_compact: Some(true),
            rows: vec![
                vec![RichBlockTableCell::header("Item"), RichBlockTableCell::header("State")],
                vec![RichBlockTableCell::text("API"), RichBlockTableCell::text("10.4")],
            ],
        },
        RichBlock::Buttons {
            buttons: vec![vec![
                RichMessageButton::callback("Approve", "ok"),
                RichMessageButton::callback("Disabled", "noop").disabled(true),
            ]],
        },
    ]);

    bot.send_rich_message(chat_id, rich).await?;
    Ok(())
}
```

When a user stops generation in the client, route `on::stopped_generation()` and inspect `ctx.stopped_generation()` (`MessageGenerationStopped { chat, draft_id }`).

---

## 6. Bot API 9.4–10.4: Ephemeral, Guest Mode, Managed Bots & Communities

- **Ephemeral Messages (10.2–10.4):**
  - Target by user ID: `EphemeralMessageParameters::for_user(user_id)`
  - Target by callback query **without `receiver_user_id`** (Bot API 10.4): `EphemeralMessageParameters::for_callback(callback_query_id).replace_callback_message(true)`
  - Route helper: `ctx.reply_ephemeral("Only visible to you").await?`
  - Mark commands in menu: `BotCommand::new("secret", "Private help").ephemeral(true)`
  - Edit/delete ephemeral content: `edit_ephemeral_message_text`, `edit_ephemeral_message_caption`, `edit_ephemeral_message_media`, `edit_ephemeral_message_reply_markup`, `delete_ephemeral_message`.
- **Guest Mode (10.2):** Route `on::guest_message()`, inspect `ctx.guest_message()`, and answer via `ctx.answer_guest()` or `bot.answer_guest_query(...)`.
- **Managed Bots (9.6–10.4):** Request creation via `KeyboardButton::request_managed_bot(text, KeyboardButtonRequestManagedBot::new(id).suggested_name(...).suggested_username(...))` with optional `.icon_custom_emoji(...)`; react to `on::managed_bot()`; manage credentials and permissions via `get_managed_bot_token`, `replace_managed_bot_token`, `get_managed_bot_access_settings`, and `set_managed_bot_access_settings`.
- **Subscriptions, Communities & Chats:** Subscribe to `on::subscription()` (`BotSubscriptionUpdated`), call `get_user_personal_chat_messages`, `set_chat_member_tag`, `answer_chat_join_request_query`, `create_forum_topic`, and `send_checklist`, or embed client-localized timestamps using `MessageEntity` (`type = "date_time"`) and `date_time_html`.

---

## 7. Router, Filters, Middleware & Dependency Injection

```rust,no_run
use std::sync::Arc;
use rustgram::prelude::*;
use tokio::sync::Mutex;

#[derive(Default)]
struct Counter {
    hits: Mutex<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let bot = Bot::from_env()?;
    let counter = Arc::new(Counter::default());

    let sub = Router::new().route(on::command("count"), |ctx: Context| async move {
        let c = ctx.state::<Arc<Counter>>().unwrap();
        let n = *c.hits.lock().await;
        ctx.reply(format!("Total handled updates: {n}")).await?;
        Ok(())
    });

    let root = Router::new()
        .layer(|ctx: Context, next: Next| async move {
            if let Some(c) = ctx.state::<Arc<Counter>>() {
                *c.hits.lock().await += 1;
            }
            next.run(ctx).await
        })
        .nest(on::private_chat(), sub)
        .fallback(|ctx: Context| async move {
            let _ = ctx.reply("Unhandled update").await;
            Ok(())
        });

    Dispatcher::new(bot, root)
        .manage(counter)
        .on_error(|_ctx, err| async move {
            eprintln!("Handler error: {err}");
        })
        .run_polling()
        .await;

    Ok(())
}
```

Combinators & custom filters:
- Combine filters with `.and(...)`, `.or(...)`, `!filter`, or `.negated()`.
- Define inline sync/async predicates via `filter_fn(|ctx| ...)` and `async_filter(|ctx| async move { ... })`.

---

## 8. Finite State Machine (FSM)

```rust,no_run
use std::sync::Arc;
use rustgram::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Wizard {
    WaitEmail,
    WaitConfirm,
}

fn configure_fsm(bot: Bot) -> Dispatcher {
    let storage: Arc<dyn StateStorage<Wizard>> = Arc::new(MemoryStorage::new());

    let router = Router::new()
        .route(on::command("register"), |ctx: Context| async move {
            ctx.fsm::<Wizard>()?.set(Wizard::WaitEmail).await?;
            ctx.reply("Enter your email:").await?;
            Ok(())
        })
        .route(on::state(Wizard::WaitEmail).and(on::text()), |ctx: Context| async move {
            let email = ctx.message().and_then(|m| m.text.as_deref()).unwrap_or("");
            let fsm = ctx.fsm::<Wizard>()?;
            fsm.insert("email", email).await?;
            fsm.set(Wizard::WaitConfirm).await?;
            ctx.reply("Confirm registration (yes/no)?").await?;
            Ok(())
        })
        .route(on::state(Wizard::WaitConfirm).and(on::text()), |ctx: Context| async move {
            let fsm = ctx.fsm::<Wizard>()?;
            let email: String = fsm.get_value("email").await?.unwrap_or_default();
            fsm.clear().await?;
            ctx.reply(format!("Registered: {email}")).await?;
            Ok(())
        });

    Dispatcher::new(bot, router).manage_fsm(storage)
}
```

Implement `StateStorage<S>` for persistent external storage (Redis/PostgreSQL) if running multiple replicas.

---

## 9. Polling, Axum Webhooks, Mini Apps & Formatting

### Polling Options

```rust,no_run
use rustgram::prelude::*;

async fn run_custom_polling(bot: Bot, dp: Dispatcher) {
    let polling = Polling::new(bot)
        .timeout_secs(30)
        .limit(100)
        .drop_pending_updates(true)
        .delete_webhook(true);

    // Optional graceful shutdown handle: polling.token().cancel();
    dp.run_polling_with(polling).await;
}
```

### Axum Webhook Server (`webhook` feature)

```rust,no_run
use std::net::SocketAddr;
use rustgram::prelude::*;
use rustgram::webhook::{WebhookConfig, serve_webhook};

async fn start_webhook(bot: Bot, router: Router) -> Result<()> {
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let cfg = WebhookConfig {
        path: "/tg/webhook".into(),
        secret_token: Some("change-me-secret".into()),
        concurrent_handling: true,
    };

    let dp = Dispatcher::new(bot, router);
    serve_webhook(addr, cfg, move |update| {
        let dp = dp.clone();
        async move {
            let _ = dp.dispatch(update).await;
        }
    })
    .await
}
```

You can also integrate `webhook_router` or `parse_webhook_request(secret, &headers, body)` directly into an existing `axum::Router`.

### Telegram Mini App `initData` Verification (`webapp` feature)

```rust,no_run
use rustgram::prelude::*;
use rustgram::webapp::verify_init_data;

fn verify_webapp(raw_init_data: &str, bot_token: &str) -> Result<Option<i64>> {
    let verified = verify_init_data(raw_init_data, bot_token, Some(300), None)?;
    Ok(verified.user.map(|u| u.id))
}
```

### Safe Formatting Helpers (`rustgram::formatting`)

`escape_html`, `escape_markdown_v2`, `bold_html`, `italic_html`, `code_html`, `pre_html`, `link_html`, `mention_html`, `custom_emoji_html`, `date_time_html`.

---

## 10. Examples & Verification Commands

Included runnable examples in `examples/`:
- [`examples/echo_bot.rs`](../../examples/echo_bot.rs) — Minimal polling echo bot.
- [`examples/widget_bot.rs`](../../examples/widget_bot.rs) — Bot API 10.4 interactive HTML widget and TON `WidgetQueryType` showcase.
- [`examples/rich_streaming_bot.rs`](../../examples/rich_streaming_bot.rs) — Stoppable draft streaming, `RichMessage`, and ephemeral responses.
- [`examples/fsm_webhook_bot.rs`](../../examples/fsm_webhook_bot.rs) — Multi-step FSM wizard served via Axum webhook.

Local quality gate:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo check --no-default-features --features rustls-tls --lib --examples
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo test --all-targets
```
