//! Bot API 10.4 Interactive HTML Widget, TON Wallet Query, and Managed Bot example.
//!
//! Run with:
//! ```bash
//! TELEGRAM_BOT_TOKEN=123456:ABC cargo run --example widget_bot
//! ```

use rustgram::prelude::*;

const COUNTER_WIDGET_HTML: &str = r#"<!doctype html>
<html>
<head>
<style>
  body { font-family: system-ui, sans-serif; padding: 12px; margin: 0; }
  button { padding: 8px 14px; border-radius: 8px; border: 0; background: #2aabee; color: #fff; font-weight: 600; }
</style>
</head>
<body>
  <h3 id="title">RustGram Live Widget</h3>
  <p id="status">Clicks: 0</p>
  <button onclick="inc()">+1 Vote</button>
  <script>
    function render() {
      const state = JSON.parse(TelegramWidget.chatData || '{"clicks":0}');
      document.getElementById('status').textContent = 'Clicks: ' + state.clicks;
    }
    async function inc() {
      const resp = await TelegramWidget.request('increment');
      if (resp) document.getElementById('status').textContent = 'Clicks: ' + JSON.parse(resp).clicks;
    }
    TelegramWidget.onChatDataChanged(render);
    render();
  </script>
</body>
</html>"#;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let bot = Bot::from_env()?;

    let router = Router::new()
        .route(on::command("widget"), |ctx: Context| async move {
            ctx.send_widget(COUNTER_WIDGET_HTML)?
                .size(0, 140)
                .chat_data(r#"{"clicks":0}"#)
                .caption("Interactive Bot API 10.4 HTML Widget")
                .await?;
            Ok(())
        })
        .route(on::command("managed"), |ctx: Context| async move {
            let kb = ReplyKeyboardMarkup::new(vec![vec![KeyboardButton::request_managed_bot(
                "Create Managed Assistant Bot",
                KeyboardButtonRequestManagedBot::new(1)
                    .suggested_name("My RustGram Agent")
                    .suggested_username("my_rustgram_agent"),
            )]])
            .one_time(true);

            ctx.send_message("Tap the button below to provision a managed bot:")?
                .reply_markup(kb)
                .await?;
            Ok(())
        })
        .route(
            on::any_widget_query().and(on::widget_data()),
            |ctx: Context| async move {
                let next_state = r#"{"clicks":1}"#;
                if let Some(wq) = ctx.widget_query() {
                    if let Some(ref msg) = wq.message {
                        ctx.bot
                            .edit_message_widget_data(msg.chat.id, msg.message_id)
                            .chat_data(next_state)
                            .await?;
                    } else if let Some(ref inline_id) = wq.inline_message_id {
                        ctx.bot
                            .edit_inline_widget_data(inline_id)
                            .chat_data(next_state)
                            .await?;
                    }
                    ctx.answer_widget()?.data(next_state).await?;
                } else if let Some(swq) = ctx.shared_widget_query() {
                    if let Some(ref wid) = swq.widget_id {
                        ctx.bot
                            .edit_shared_widget_data(wid)
                            .public_data(next_state)
                            .cache_time(15)
                            .await?;
                    }
                    ctx.answer_widget()?.data(next_state).await?;
                }
                Ok(())
            },
        )
        .route(on::widget_sent_transaction(), |ctx: Context| async move {
            if let Some(wq) = ctx.widget_query()
                && let WidgetQueryType::SentTransaction { wallet, msg_hash } = &wq.query_type
            {
                let addr = wallet.address.as_deref().unwrap_or("unknown");
                ctx.answer_widget()?
                    .data(format!(r#"{{"verified":true,"hash":"{msg_hash}"}}"#))
                    .await?;
                ctx.reply(format!("Received TON tx `{msg_hash}` from `{addr}`"))
                    .await?;
            }
            Ok(())
        });

    Dispatcher::new(bot, router).run_polling().await;
    Ok(())
}
