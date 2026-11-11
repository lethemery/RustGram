//! Basic echo and command bot using `rustgram` long polling.
//!
//! Run with:
//! ```bash
//! TELEGRAM_BOT_TOKEN=123456:ABC cargo run --example echo_bot
//! ```

use rustgram::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let bot = Bot::from_env()?;
    let me = bot.get_me().await?;
    println!(
        "Starting @{} (Bot API {})",
        me.username.as_deref().unwrap_or("bot"),
        BOT_API_VERSION
    );

    let router = Router::new()
        .route(on::command("start"), |ctx: Context| async move {
            let name = ctx
                .user()
                .map(User::full_name)
                .unwrap_or_else(|| "friend".to_owned());
            ctx.send_message(format!(
                "Hello, {}! Send me anything and I will echo it back.",
                bold_html(&name)
            ))?
            .parse_mode(ParseMode::Html)
            .await?;
            Ok(())
        })
        .route(on::text(), |ctx: Context| async move {
            if let Some(text) = ctx.message().and_then(|m| m.text.as_deref()) {
                ctx.reply(format!("Echo: {text}")).await?;
            }
            Ok(())
        });

    Dispatcher::new(bot, router).run_polling().await;
    Ok(())
}
