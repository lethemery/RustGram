//! Multi-step Finite State Machine (FSM) and Webhook server example.
//!
//! Run with:
//! ```bash
//! TELEGRAM_BOT_TOKEN=123456:ABC cargo run --example fsm_webhook_bot
//! ```

use std::sync::Arc;

use rustgram_api::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum RegState {
    AwaitingName,
    AwaitingAge,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let bot = Bot::from_env()?;
    let storage: Arc<dyn StateStorage<RegState>> = Arc::new(MemoryStorage::new());

    let router = Router::new()
        .layer(|ctx: Context, next: Next| async move {
            tracing::info!(update_id = ctx.update.update_id, "incoming update");
            next.run(ctx).await
        })
        .route(on::command("register"), |ctx: Context| async move {
            let fsm = ctx.fsm::<RegState>()?;
            fsm.set(RegState::AwaitingName).await?;
            ctx.reply("What is your name?").await?;
            Ok(())
        })
        .route(on::command("cancel"), |ctx: Context| async move {
            let fsm = ctx.fsm::<RegState>()?;
            fsm.clear().await?;
            ctx.reply("Registration cancelled.").await?;
            Ok(())
        })
        .route(
            on::state(RegState::AwaitingName).and(on::text()),
            |ctx: Context| async move {
                let name = ctx
                    .message()
                    .and_then(|m| m.text.as_deref())
                    .unwrap_or_default()
                    .trim()
                    .to_owned();
                let fsm = ctx.fsm::<RegState>()?;
                fsm.insert("name", name).await?;
                fsm.set(RegState::AwaitingAge).await?;
                ctx.reply("Great! How old are you?").await?;
                Ok(())
            },
        )
        .route(
            on::state(RegState::AwaitingAge).and(on::text()),
            |ctx: Context| async move {
                let text = ctx
                    .message()
                    .and_then(|m| m.text.as_deref())
                    .unwrap_or_default()
                    .trim();
                let Ok(age) = text.parse::<u16>() else {
                    ctx.reply("Please send a valid numeric age.").await?;
                    return Ok(());
                };
                let fsm = ctx.fsm::<RegState>()?;
                let name: String = fsm
                    .get_value("name")
                    .await?
                    .unwrap_or_else(|| "User".into());
                fsm.clear().await?;
                ctx.reply(format!("Registered {name} (age {age})!")).await?;
                Ok(())
            },
        );

    let dp = Dispatcher::new(bot, router).manage_fsm(storage);

    // Choose webhook mode if WEBHOOK_ADDR is set, otherwise long polling.
    if let Ok(addr_str) = std::env::var("WEBHOOK_ADDR") {
        let addr = addr_str
            .parse()
            .map_err(|e| Error::Webhook(format!("invalid WEBHOOK_ADDR: {e}")))?;
        let cancel = tokio_util::sync::CancellationToken::new();
        let config = WebhookConfig {
            path: "/webhook".into(),
            secret_token: std::env::var("WEBHOOK_SECRET").ok(),
            concurrent_handling: true,
        };
        dp.run_webhook(addr, config, cancel).await?;
    } else {
        dp.run_polling().await;
    }

    Ok(())
}
