//! Bot API 10.1–10.4 Rich Messages, Stoppable Streaming Drafts, and Ephemeral Callback Replies.
//!
//! Run with:
//! ```bash
//! TELEGRAM_BOT_TOKEN=123456:ABC cargo run --example rich_streaming_bot
//! ```

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use rustgram::prelude::*;
use rustgram::types::{RichBlockTableCell, RichMessageButton};
use tokio::sync::RwLock;

type CancelledChats = Arc<RwLock<HashSet<i64>>>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let bot = Bot::from_env()?;
    let cancelled: CancelledChats = Arc::new(RwLock::new(HashSet::new()));

    let router = Router::new()
        .route(on::command("ask"), |ctx: Context| async move {
            let Some(chat_id) = ctx.chat_id() else {
                return Ok(());
            };
            let numeric_chat = chat_id.as_i64().unwrap_or_default();
            let cancelled = ctx.state::<CancelledChats>().cloned();
            if let Some(ref set) = cancelled {
                set.write().await.remove(&numeric_chat);
            }

            let draft_id = 42_i64;
            let thinking = InputRichMessage::blocks(vec![RichBlock::Thinking {
                title: Some("Analyzing request...".into()),
                text: Some("Inspecting Bot API 10.4 schema...".into()),
                blocks: vec![],
                duration: None,
            }]);

            ctx.bot
                .send_rich_message_draft(chat_id.clone(), draft_id, thinking)
                .can_stop(true)
                .keep_on_stop(true)
                .await?;

            tokio::time::sleep(Duration::from_millis(600)).await;

            if let Some(ref set) = cancelled
                && set.read().await.contains(&numeric_chat)
            {
                return Ok(());
            }

            let final_msg = InputRichMessage::blocks(vec![
                RichBlock::SectionHeading {
                    text: "Bot API 10.4 Summary".into(),
                    level: Some(2),
                    entities: None,
                },
                RichBlock::Table {
                    caption: Some("Recent Milestones".into()),
                    is_compact: Some(true),
                    rows: vec![
                        vec![
                            RichBlockTableCell::header("Version"),
                            RichBlockTableCell::header("Highlight"),
                        ],
                        vec![
                            RichBlockTableCell::text("10.1–10.3"),
                            RichBlockTableCell::text("Rich Messages, Ephemeral Replies"),
                        ],
                        vec![
                            RichBlockTableCell::text("10.4"),
                            RichBlockTableCell::text("HTML Widgets & TON Wallet Queries"),
                        ],
                    ],
                },
                RichBlock::ExpandableBlockQuotation {
                    blocks: vec![],
                    text: Some("Tap to expand detailed architectural notes.".into()),
                },
                RichBlock::Buttons {
                    buttons: vec![vec![RichMessageButton::callback(
                        "Private Details (Ephemeral)",
                        "ephemeral_details",
                    )]],
                },
            ]);

            ctx.reply_rich(final_msg).await?;
            Ok(())
        })
        .route(on::stopped_generation(), |ctx: Context| async move {
            if let Some(stopped) = ctx.stopped_generation()
                && let Some(set) = ctx.state::<CancelledChats>()
            {
                set.write().await.insert(stopped.chat.id);
            }
            Ok(())
        })
        .route(
            on::callback_data("ephemeral_details"),
            |ctx: Context| async move {
                ctx.answer_callback()?.await?;
                // Bot API 10.4: callback_query_id can be passed without receiver_user_id!
                ctx.reply_ephemeral("Only you can see this ephemeral message in the chat.")
                    .await?;
                Ok(())
            },
        );

    Dispatcher::new(bot, router)
        .manage(cancelled)
        .run_polling()
        .await;
    Ok(())
}
