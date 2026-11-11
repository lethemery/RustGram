# rustgram — Полная документация (Русский)

> Асинхронная, строго типизированная библиотека и фреймворк на Rust для создания Telegram-ботов с поддержкой **Telegram Bot API 10.4** (`rustgram::BOT_API_VERSION == "10.4"`).

- [Интерактивный двуязычный сайт документации](../index.html)
- [Documentation in English](../en/README.md)
- [Корневой README репозитория](../../README.md)

---

## Содержание

1. [Обзор и архитектура](#1-обзор-и-архитектура)
2. [Установка и Cargo-фичи](#2-установка-и-cargo-фичи)
3. [Клиент Bot, файлы и обработка ошибок](#3-клиент-bot-файлы-и-обработка-ошибок)
4. [Bot API 10.4: HTML-виджеты и TON-кошелёк](#4-bot-api-104-html-виджеты-и-ton-кошелёк)
5. [Bot API 10.1–10.3: Rich-сообщения и стриминг черновиков](#5-bot-api-101103-rich-сообщения-и-стриминг-черновиков)
6. [Bot API 9.4–10.4: Эфемерные сообщения, Guest Mode, Managed Bots и сообщества](#6-bot-api-94104-эфемерные-сообщения-guest-mode-managed-bots-и-сообщества)
7. [Роутер, фильтры, Middleware и DI](#7-роутер-фильтры-middleware-и-di)
8. [Конечный автомат (FSM)](#8-конечный-автомат-fsm)
9. [Polling, Axum Webhook, Mini Apps и форматирование](#9-polling-axum-webhook-mini-apps-и-форматирование)
10. [Примеры и команды проверки](#10-примеры-и-команды-проверки)
11. [Публикация на GitHub, GitHub Pages и crates.io](#11-публикация-на-github-github-pages-и-cratesio)

---

## 1. Обзор и архитектура

Библиотека `rustgram` разделена на понятные независимые модули:

| Модуль | Назначение |
| --- | --- |
| `rustgram::client` | `Bot`, `BotBuilder`, маршрутизация URL (`method_url` / `file_url`), JSON и `multipart/form-data` запросы, автоматические повторы при 429 rate limit, скачивание файлов. |
| `rustgram::types` | Модели Serde для Telegram Bot API 9.4–10.4 (`Update`, `Message`, `Widget`, `WidgetQuery`, `RichMessage`, `RichBlock`, `EphemeralMessageParameters` и др.). Крупные варианты упакованы в `Box`, чтобы `Update` и `Message` занимали `< 4096` байт на стеке. |
| `rustgram::requests` | Типизированные билдеры запросов с реализацией `IntoFuture<Output = Result<T>>` для сообщений, файлов, чатов, управляемых ботов, апдейтов, виджетов, Rich- и эфемерных сообщений. |
| `rustgram::router` | `Router`, `Dispatcher`, `Context`, middleware (`Next`), встроенные фильтры `on::*` и FSM (`StateStorage<S>`, `MemoryStorage<S>`, `StateContext<S>`). |
| `rustgram::polling` | Асинхронный `getUpdates`-поллер с экспоненциальным бэкоффом и корректным завершением через `CancellationToken`. |
| `rustgram::webhook` *(фича `webhook`)* | HTTP-роутер и сервер для вебхуков на базе `axum`, проверка заголовка `SECRET_TOKEN_HEADER`, `parse_webhook_request`. |
| `rustgram::webapp` *(фича `webapp`)* | Криптографическая проверка `initData` из Telegram Mini Apps по HMAC-SHA256 (`verify_init_data`). |
| `rustgram::formatting` | Безопасное экранирование HTML и MarkdownV2, теги оформления включая `custom_emoji_html` и `date_time_html`. |
| `rustgram::error` | `Error`, `ApiError` и `ResponseParameters` с хелперами (`is_rate_limited`, `retry_after`, `migrate_to_chat_id` и др.). |

Импорт базовых типов и фильтров:

```rust,no_run
use rustgram::prelude::*;
```

---

## 2. Установка и Cargo-фичи

Добавьте `rustgram` и `tokio` в `Cargo.toml` вашего проекта:

```toml
[dependencies]
rustgram = { git = "https://github.com/<your-username>/RustGram" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Если нужен минимальный бот только на long polling (без `axum` и проверки `webapp`):

```toml
[dependencies]
rustgram = { git = "https://github.com/<your-username>/RustGram", default-features = false, features = ["rustls-tls"] }
```

### Таблица Cargo-фич

| Фича | По умолчанию | Описание |
| --- | --- | --- |
| `rustls-tls` | Да | TLS через `rustls` + `ring` внутри `reqwest` без необходимости системного OpenSSL. |
| `webhook` | Да | Подключает `rustgram::webhook` на базе `axum`. |
| `webapp` | Да | Подключает `rustgram::webapp` для проверки `initData` (`hmac`, `sha2`, `hex`, `url`). |

---

## 3. Клиент Bot, файлы и обработка ошибок

### Инициализация клиента

```rust,no_run
use std::time::Duration;
use rustgram::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Загрузка токена из переменной окружения TELEGRAM_BOT_TOKEN
    let bot_from_env = Bot::from_env()?;

    // 2. Явная передача токена с проверкой
    let bot_validated = Bot::try_new("123456:ABC-DEF")?;

    // 3. Гибкая настройка (Local Bot API, тестовый контур, retry при 429)
    let bot_custom = Bot::builder("123456:ABC-DEF")
        .base_url("https://api.telegram.org")
        .test_environment(false)
        .timeout(Duration::from_secs(60))
        .retry_rate_limits(3, Duration::from_secs(15))
        .build()?;

    let me = bot_from_env.get_me().await?;
    println!(
        "Бот @{} запущен (test_env={})",
        me.username.unwrap_or_default(),
        bot_custom.is_test_environment()
    );
    let _ = bot_validated;
    Ok(())
}
```

> **Важно про безопасность:** никогда не храните реальный токен в коде и не добавляйте `.env` в Git. Для передачи ключа используйте переменную окружения `TELEGRAM_BOT_TOKEN`.

### Отправка и скачивание файлов

`InputFile` автоматически выбирает JSON-отправку или `multipart/form-data`:

- `InputFile::file_id("...")` — готовый `file_id` в Telegram (JSON).
- `InputFile::url("https://...")` — внешняя ссылка (JSON).
- `InputFile::memory("report.txt", bytes)` — загрузка из памяти (multipart).
- `InputFile::path("./chart.png")` — чтение и загрузка локального файла (multipart).

Скачивание файла по `file_id`:

```rust,no_run
use rustgram::prelude::*;

async fn download_asset(bot: &Bot, file_id: &str) -> Result<bytes::Bytes> {
    bot.download_file_by_id(file_id).await
}
```

### Обработка ошибок Telegram API

Ошибки от Bot API (`ok: false`) возвращаются как `Error::Api(ApiError)`:

- `api_err.error_code` — код ответа (например, `429`, `403`, `400`).
- `api_err.description` — текст ошибки от Telegram.
- `api_err.parameters` — `ResponseParameters` (`retry_after`, `migrate_to_chat_id`).
- `api_err.is_rate_limited()` и `api_err.retry_after()` — проверка превышения лимитов (HTTP 429).
- `api_err.migrate_to_chat_id()` — новый ID при миграции группы в супергруппу.
- `api_err.is_forbidden()` — пользователь заблокировал бота или не хватает прав.
- `api_err.is_not_modified()` — редактирование не изменило содержимое сообщения.

---

## 4. Bot API 10.4: HTML-виджеты и TON-кошелёк

Версия Bot API 10.4 добавляет интерактивные HTML-виджеты прямо внутри сообщений и встроенные события TON-кошелька (`SignData`, `SentTransaction`).

### Локальная валидация ограничений

Перед отправкой HTTP-запроса `rustgram` проверяет инварианты на стороне клиента:

- `WIDGET_HTML_MAX_BYTES` (`32_768` байт) и запрет нулевых байтов `\0` (`validate_widget_html`).
- `WIDGET_DATA_MAX_BYTES` (`1_024` байт) для `public_data` и `chat_data` (`validate_widget_data`).
- `clamp_widget_cache_time` ограничивает время кэша диапазоном `[WIDGET_CACHE_TIME_MIN (5) ..= WIDGET_CACHE_TIME_MAX (3_600)]` (по умолчанию `WIDGET_CACHE_TIME_DEFAULT = 30`).

### Отправка и обновление виджетов

```rust,no_run
use rustgram::prelude::*;

async fn widget_handlers(ctx: Context) -> Result<()> {
    let html = r#"
      <button onclick="TelegramWidget.request('checkout')">Оплатить 1 TON</button>
    "#;
    ctx.send_widget(html)?
        .size(320, 180)
        .shared(true)
        .public_data(r#"{"price":"1 TON"}"#)
        .chat_data(r#"{"clicks":0}"#)
        .cache_time(60)
        .caption("Оплата в TON")
        .await?;

    // Повторное использование общего виджета по widget_id
    if let Some(chat_id) = ctx.chat_id() {
        ctx.bot.send_shared_widget(chat_id, "shared_widget_id").await?;
    }
    Ok(())
}
```

### Обработка запросов виджета и TON-транзакций

Фильтры: `on::widget_query()`, `on::shared_widget_query()`, `on::any_widget_query()`, `on::widget_data()`, `on::widget_sign_data()`, `on::widget_sent_transaction()`:

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

Дополнительные методы:
- `bot.edit_inline_widget_data(inline_message_id)` и `bot.edit_shared_widget_data(widget_id)`
- Инлайн-контент: `InputWidgetMessageContent`, `InputSharedWidgetMessageContent`, `InputMessageContent::Widget`, `InputMessageContent::SharedWidget`.

---

## 5. Bot API 10.1–10.3: Rich-сообщения и стриминг черновиков

Создавайте насыщенные блочные сообщения через `InputRichMessage::html(...)`, `InputRichMessage::markdown(...)` или `InputRichMessage::blocks(...)`.

Поддерживаемые варианты `RichBlock`: `Paragraph`, `SectionHeading`, `Preformatted`, `Footer`, `Divider`, `MathematicalExpression`, `Anchor`, `List`, `BlockQuotation`, `ExpandableBlockQuotation`, `PullQuotation`, `Table` (`is_compact`), `Details`, `Thinking`, `Buttons` (с поддержкой `RichMessageButton::disabled`), `Document`, `Photo`, `Video`, `Audio`, `Animation`, `VoiceNote`, `Collage`, `Slideshow`, `Map`, а также `Unsupported` для будущих версий API.

```rust,no_run
use rustgram::prelude::*;
use rustgram::types::{InputRichMessage, RichBlock, RichBlockTableCell, RichMessageButton};

async fn stream_ai_answer(bot: &Bot, chat_id: i64, draft_id: i64) -> Result<()> {
    // Стриминговый черновик с кнопкой остановки (Bot API 10.3)
    bot.send_message_draft(chat_id, draft_id, "Формирую ответ...")
        .can_stop(true)
        .keep_on_stop(true)
        .await?;

    // Итоговое Rich-сообщение
    let rich = InputRichMessage::blocks(vec![
        RichBlock::SectionHeading {
            text: "Сводка".into(),
            level: Some(2),
            entities: None,
        },
        RichBlock::Table {
            caption: Some("Метрики".into()),
            is_compact: Some(true),
            rows: vec![
                vec![RichBlockTableCell::header("Параметр"), RichBlockTableCell::header("Значение")],
                vec![RichBlockTableCell::text("API"), RichBlockTableCell::text("10.4")],
            ],
        },
        RichBlock::Buttons {
            buttons: vec![vec![
                RichMessageButton::callback("Подтвердить", "ok"),
                RichMessageButton::callback("Заблокировано", "noop").disabled(true),
            ]],
        },
    ]);

    bot.send_rich_message(chat_id, rich).await?;
    Ok(())
}
```

Чтобы реагировать на остановку генерации пользователем, используйте фильтр `on::stopped_generation()` и метод `ctx.stopped_generation()` (`MessageGenerationStopped`).

---

## 6. Bot API 9.4–10.4: Эфемерные сообщения, Guest Mode, Managed Bots и сообщества

- **Эфемерные сообщения (10.2–10.4):**
  - Адресация пользователю: `EphemeralMessageParameters::for_user(user_id)`.
  - Адресация по callback-запросу **без `receiver_user_id`** (Bot API 10.4): `EphemeralMessageParameters::for_callback(callback_query_id).replace_callback_message(true)`.
  - Быстрый ответ в хендлере: `ctx.reply_ephemeral("Это сообщение видите только вы").await?`.
  - Пометка команд меню как эфемерных: `BotCommand::new("secret", "Приватная справка").ephemeral(true)`.
  - Редактирование и удаление: `edit_ephemeral_message_text`, `edit_ephemeral_message_caption`, `edit_ephemeral_message_media`, `edit_ephemeral_message_reply_markup`, `delete_ephemeral_message`.
- **Гостевой режим (10.2):** Фильтр `on::guest_message()`, данные `ctx.guest_message()`, ответы через `ctx.answer_guest()` или `bot.answer_guest_query(...)`.
- **Управляемые боты (Managed Bots, 9.6–10.4):** Запрос создания бота через `KeyboardButton::request_managed_bot(text, KeyboardButtonRequestManagedBot::new(id).suggested_name(...).suggested_username(...))` с иконкой `.icon_custom_emoji(...)`; фильтр `on::managed_bot()`; методы `get_managed_bot_token`, `replace_managed_bot_token`, `get_managed_bot_access_settings`, `set_managed_bot_access_settings`.
- **Подписки, сообщества и чаты:** Фильтр `on::subscription()` (`BotSubscriptionUpdated`), методы `get_user_personal_chat_messages`, `set_chat_member_tag`, `answer_chat_join_request_query`, `create_forum_topic`, `send_checklist`, а также сущности `date_time` и хелпер `date_time_html`.

---

## 7. Роутер, фильтры, Middleware и DI

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
        ctx.reply(format!("Всего обработано апдейтов: {n}")).await?;
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
            let _ = ctx.reply("Команда не распознана").await;
            Ok(())
        });

    Dispatcher::new(bot, root)
        .manage(counter)
        .on_error(|_ctx, err| async move {
            eprintln!("Ошибка обработчика: {err}");
        })
        .run_polling()
        .await;

    Ok(())
}
```

Комбинаторы и кастомные фильтры:
- `.and(...)`, `.or(...)`, `!filter`, `.negated()`.
- Синхронные и асинхронные предикаты через `filter_fn(|ctx| ...)` и `async_filter(|ctx| async move { ... })`.

---

## 8. Конечный автомат (FSM)

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
            ctx.reply("Введите email:").await?;
            Ok(())
        })
        .route(on::state(Wizard::WaitEmail).and(on::text()), |ctx: Context| async move {
            let email = ctx.message().and_then(|m| m.text.as_deref()).unwrap_or("");
            let fsm = ctx.fsm::<Wizard>()?;
            fsm.insert("email", email).await?;
            fsm.set(Wizard::WaitConfirm).await?;
            ctx.reply("Подтвердить регистрацию (да/нет)?").await?;
            Ok(())
        })
        .route(on::state(Wizard::WaitConfirm).and(on::text()), |ctx: Context| async move {
            let fsm = ctx.fsm::<Wizard>()?;
            let email: String = fsm.get_value("email").await?.unwrap_or_default();
            fsm.clear().await?;
            ctx.reply(format!("Регистрация завершена: {email}")).await?;
            Ok(())
        });

    Dispatcher::new(bot, router).manage_fsm(storage)
}
```

Для распределённых ботов реализуйте собственный бэкенд `StateStorage<S>` (например, поверх Redis или PostgreSQL).

---

## 9. Polling, Axum Webhook, Mini Apps и форматирование

### Настройка Polling

```rust,no_run
use rustgram::prelude::*;

async fn run_custom_polling(bot: Bot, dp: Dispatcher) {
    let polling = Polling::new(bot)
        .timeout_secs(30)
        .limit(100)
        .drop_pending_updates(true)
        .delete_webhook(true);

    // Плавная остановка по сигналу: polling.token().cancel();
    dp.run_polling_with(polling).await;
}
```

### HTTP-вебхук на Axum (фича `webhook`)

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

Также можно встраивать `webhook_router` или вызывать `parse_webhook_request(secret, &headers, body)` в собственном приложении на `axum`.

### Проверка `initData` для Telegram Mini Apps (фича `webapp`)

```rust,no_run
use rustgram::prelude::*;
use rustgram::webapp::verify_init_data;

fn verify_webapp(raw_init_data: &str, bot_token: &str) -> Result<Option<i64>> {
    let verified = verify_init_data(raw_init_data, bot_token, Some(300), None)?;
    Ok(verified.user.map(|u| u.id))
}
```

### Хелперы форматирования (`rustgram::formatting`)

`escape_html`, `escape_markdown_v2`, `bold_html`, `italic_html`, `code_html`, `pre_html`, `link_html`, `mention_html`, `custom_emoji_html`, `date_time_html`.

---

## 10. Примеры и команды проверки

Готовые примеры в папке `examples/`:
- [`examples/echo_bot.rs`](../../examples/echo_bot.rs) — простой эхо-бот на polling.
- [`examples/widget_bot.rs`](../../examples/widget_bot.rs) — интерактивные HTML-виджеты 10.4 и TON-кошелёк.
- [`examples/rich_streaming_bot.rs`](../../examples/rich_streaming_bot.rs) — стриминг черновиков с отменой, `RichMessage` и эфемерные ответы.
- [`examples/fsm_webhook_bot.rs`](../../examples/fsm_webhook_bot.rs) — пошаговый FSM-бот на `axum` webhook.

Локальная проверка перед коммитом:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo check --no-default-features --features rustls-tls --lib --examples
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo test --all-targets
```

---

## 11. Публикация на GitHub, GitHub Pages и crates.io

### Шаг 1: Загрузка кода на GitHub

```bash
cd c:\Projects\RustGram
git init -b main
git add .
git commit -m "Initial release: rustgram Bot API 10.4 + bilingual docs"
git remote add origin https://github.com/<your-username>/RustGram.git
git push -u origin main
```

### Шаг 2: Включение GitHub Pages (двуязычный сайт + Rustdoc API)

1. Откройте репозиторий на GitHub и перейдите в **Settings → Pages**.
2. В пункте **Build and deployment → Source** выберите **GitHub Actions**.
3. Сделайте `git push` в ветку `main` (или запустите workflow **Deploy Documentation to GitHub Pages** вручную во вкладке **Actions**).
4. Workflow [`.github/workflows/docs.yml`](../../.github/workflows/docs.yml) соберёт `cargo doc --no-deps --all-features`, разместит Rustdoc по адресу `/api/rustgram/index.html` и опубликует двуязычный сайт из папки `docs/` на `https://<your-username>.github.io/RustGram/`.

*(Альтернативный способ без Actions: **Deploy from a branch** → `main` → `/docs`. Статический сайт `docs/index.html` работает сразу без этапа сборки, но без автосгенерированного Rustdoc в `/api/`.)*

### Шаг 3: Публикация библиотеки на `crates.io`

1. Перед публикацией обновите поля `repository`, `homepage` и `documentation` в файле `Cargo.toml`, указав ваш реальный GitHub-репозиторий и адрес GitHub Pages.
2. Запустите пробную упаковку и публикацию:

```bash
cargo publish --dry-run
cargo login
cargo publish
```
