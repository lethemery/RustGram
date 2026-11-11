use std::sync::Arc;
use std::time::Duration;

use axum::http::{HeaderMap, HeaderValue, StatusCode};
use hmac::{Hmac, Mac};
use rustgram_api::prelude::*;
use rustgram_api::router::StateKey;
use rustgram_api::types::{WIDGET_DATA_MAX_BYTES, WIDGET_HTML_MAX_BYTES};
use rustgram_api::webhook::{
    SECRET_TOKEN_HEADER, WebhookConfig, parse_webhook_request, webhook_router,
};
use serde_json::json;
use sha2::Sha256;
use tokio::sync::Mutex;

type HmacSha256 = Hmac<Sha256>;

fn text_message_update(update_id: i64, chat_id: i64, user_id: i64, text: &str) -> Update {
    serde_json::from_value(json!({
        "update_id": update_id,
        "message": {
            "message_id": update_id,
            "date": 1794240000,
            "chat": { "id": chat_id, "type": "private", "first_name": "Tester" },
            "from": { "id": user_id, "is_bot": false, "first_name": "Tester" },
            "text": text
        }
    }))
    .expect("construct text update")
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WizardStep {
    WaitEmail,
    WaitConfirm,
}

#[test]
fn update_and_message_stay_compact_on_stack() {
    assert!(
        std::mem::size_of::<Update>() < 4096,
        "Update size {} is unexpectedly large",
        std::mem::size_of::<Update>()
    );
    assert!(
        std::mem::size_of::<Message>() < 4096,
        "Message size {} is unexpectedly large",
        std::mem::size_of::<Message>()
    );
}

#[tokio::test]
async fn router_filters_middleware_and_fsm_work_end_to_end() {
    let bot = Bot::new("123456:TEST_TOKEN");
    let storage: Arc<dyn StateStorage<WizardStep>> = Arc::new(MemoryStorage::new());
    let trace_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let log_for_state = Arc::clone(&trace_log);

    let widget_router = Router::new()
        .route(on::widget_sent_transaction(), |ctx: Context| async move {
            ctx.state::<Arc<Mutex<Vec<String>>>>()
                .unwrap()
                .lock()
                .await
                .push("widget_tx".into());
            Ok(())
        })
        .route(on::widget_data(), |ctx: Context| async move {
            ctx.state::<Arc<Mutex<Vec<String>>>>()
                .unwrap()
                .lock()
                .await
                .push("widget_data".into());
            Ok(())
        });

    let root = Router::new()
        .layer(|ctx: Context, next: Next| async move {
            ctx.state::<Arc<Mutex<Vec<String>>>>()
                .unwrap()
                .lock()
                .await
                .push(format!("mw:{}", ctx.update.update_id));
            next.run(ctx).await
        })
        .route(on::command("start"), |ctx: Context| async move {
            let fsm = ctx.fsm::<WizardStep>()?;
            fsm.set(WizardStep::WaitEmail).await?;
            ctx.state::<Arc<Mutex<Vec<String>>>>()
                .unwrap()
                .lock()
                .await
                .push("cmd:start".into());
            Ok(())
        })
        .route(
            on::state(WizardStep::WaitEmail).and(on::text()),
            |ctx: Context| async move {
                let email = ctx.message().and_then(|m| m.text.as_deref()).unwrap_or("");
                let fsm = ctx.fsm::<WizardStep>()?;
                fsm.insert("email", email).await?;
                fsm.set(WizardStep::WaitConfirm).await?;
                Ok(())
            },
        )
        .route(
            on::state(WizardStep::WaitConfirm).and(on::text()),
            |ctx: Context| async move {
                let fsm = ctx.fsm::<WizardStep>()?;
                let saved: Option<String> = fsm.get_value("email").await?;
                fsm.clear().await?;
                ctx.state::<Arc<Mutex<Vec<String>>>>()
                    .unwrap()
                    .lock()
                    .await
                    .push(format!("confirmed:{}", saved.unwrap_or_default()));
                Ok(())
            },
        )
        .nest(on::any_widget_query(), widget_router)
        .route(on::guest_message(), |ctx: Context| async move {
            ctx.state::<Arc<Mutex<Vec<String>>>>()
                .unwrap()
                .lock()
                .await
                .push("guest".into());
            Ok(())
        });

    let dp = Dispatcher::new(bot, root)
        .manage(log_for_state)
        .manage_fsm(Arc::clone(&storage));

    assert_eq!(
        dp.dispatch(text_message_update(1, 50, 50, "/start")).await,
        DispatchOutcome::Handled
    );
    assert_eq!(
        dp.dispatch(text_message_update(2, 50, 50, "ada@example.org"))
            .await,
        DispatchOutcome::Handled
    );
    assert_eq!(
        dp.dispatch(text_message_update(3, 50, 50, "yes")).await,
        DispatchOutcome::Handled
    );

    let widget_tx: Update = serde_json::from_value(json!({
        "update_id": 4,
        "widget_query": {
            "id": "w1",
            "from": { "id": 50, "is_bot": false, "first_name": "Tester" },
            "chat_instance": "50",
            "type": {
                "type": "sent_transaction",
                "wallet": { "address": "EQ..." },
                "msg_hash": "hash123"
            }
        }
    }))
    .expect("widget tx update");
    assert_eq!(dp.dispatch(widget_tx).await, DispatchOutcome::Handled);

    let guest: Update = serde_json::from_value(json!({
        "update_id": 5,
        "guest_message": {
            "message_id": 9,
            "date": 1794240000,
            "chat": { "id": -1001, "type": "supergroup", "title": "Pub" },
            "text": "hi"
        }
    }))
    .expect("guest update");
    assert_eq!(dp.dispatch(guest).await, DispatchOutcome::Handled);

    let unrouted: Update = serde_json::from_value(json!({
        "update_id": 6,
        "subscription": {
            "user": { "id": 1, "is_bot": false, "first_name": "Sub" },
            "invoice_payload": "p",
            "state": "active"
        }
    }))
    .expect("subscription update");
    assert_eq!(dp.dispatch(unrouted).await, DispatchOutcome::Unhandled);

    let events = trace_log.lock().await.clone();
    assert_eq!(
        events,
        vec![
            "mw:1",
            "cmd:start",
            "mw:2",
            "mw:3",
            "confirmed:ada@example.org",
            "mw:4",
            "widget_tx",
            "mw:5",
            "guest",
            "mw:6",
        ]
    );

    // State was cleared on confirmation.
    let final_state = storage.get_state(StateKey::new(50, 50)).await.unwrap();
    assert!(final_state.is_none());
}

#[tokio::test]
async fn bot_builder_urls_validation_and_precancelled_polling() {
    assert!(matches!(Bot::try_new("   "), Err(Error::InvalidInput(_))));

    let prod_bot = Bot::builder("123:ABC")
        .base_url("https://local-bot-api.example/")
        .retry_rate_limits(2, Duration::from_secs(10))
        .build()
        .unwrap();
    assert!(!prod_bot.is_test_environment());
    assert_eq!(
        prod_bot.method_url("sendWidget"),
        "https://local-bot-api.example/bot123:ABC/sendWidget"
    );
    assert_eq!(
        prod_bot.file_url("/photos/p1.jpg"),
        "https://local-bot-api.example/file/bot123:ABC/photos/p1.jpg"
    );

    let test_bot = Bot::builder("123:ABC")
        .test_environment(true)
        .build()
        .unwrap();
    assert!(test_bot.is_test_environment());
    assert_eq!(
        test_bot.method_url("sendRichMessage"),
        "https://api.telegram.org/bot123:ABC/test/sendRichMessage"
    );
    assert_eq!(
        test_bot.file_url("docs/a.pdf"),
        "https://api.telegram.org/file/bot123:ABC/test/docs/a.pdf"
    );

    // Local widget invariant validation rejects invalid payloads before making HTTP calls.
    let nul_res = prod_bot.send_widget(123, "<div>\0</div>").await;
    assert!(matches!(nul_res, Err(Error::InvalidInput(_))));

    let oversized_html = "x".repeat(WIDGET_HTML_MAX_BYTES + 1);
    let len_res = prod_bot.send_widget(123, oversized_html).await;
    assert!(matches!(len_res, Err(Error::InvalidInput(_))));

    let oversized_data = "d".repeat(WIDGET_DATA_MAX_BYTES + 1);
    let query_res = prod_bot
        .answer_widget_query("wq_1")
        .data(oversized_data)
        .await;
    assert!(matches!(query_res, Err(Error::InvalidInput(_))));

    // Verify clamped cache_time and serialization on widget builder payload.
    let builder = prod_bot
        .send_widget(123, "<p>Widget</p>")
        .size(300, 120)
        .shared(true)
        .cache_time(99_999);
    assert_eq!(builder.payload.width, 300);
    assert_eq!(builder.payload.height, 120);
    assert_eq!(builder.payload.is_shared, Some(true));
    assert_eq!(builder.payload.cache_time, Some(3600));

    // ResponseParameters deserialization & ApiError helpers.
    let params: rustgram_api::ResponseParameters =
        serde_json::from_value(json!({ "retry_after": 3, "migrate_to_chat_id": -100999 })).unwrap();
    let api_err = ApiError::new(429, "Too Many Requests: retry after 3", Some(params));
    assert!(api_err.is_rate_limited());
    assert_eq!(api_err.retry_after(), Some(Duration::from_secs(3)));
    assert_eq!(api_err.migrate_to_chat_id(), Some(-100999));

    // Pre-cancelled Polling runner exits immediately without touching the network.
    let polling = Polling::new(prod_bot).limit(50);
    polling.token().cancel();
    polling.run(|_u| async move {}).await;
}

#[tokio::test]
async fn webhook_parsing_and_secret_verification_enforces_security() {
    let secret = Some("super_secret_10_4");

    // 1. Missing or wrong secret header => 401 UNAUTHORIZED
    let mut bad_headers = HeaderMap::new();
    bad_headers.insert(SECRET_TOKEN_HEADER, HeaderValue::from_static("wrong_token"));
    assert_eq!(
        parse_webhook_request(secret, &bad_headers, b"{}").unwrap_err(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        parse_webhook_request(secret, &HeaderMap::new(), b"{}").unwrap_err(),
        StatusCode::UNAUTHORIZED
    );

    // 2. Valid secret header but invalid Update JSON => 400 BAD_REQUEST
    let mut ok_headers = HeaderMap::new();
    ok_headers.insert(
        SECRET_TOKEN_HEADER,
        HeaderValue::from_static("super_secret_10_4"),
    );
    assert_eq!(
        parse_webhook_request(secret, &ok_headers, br#"{"not_an_update":true}"#).unwrap_err(),
        StatusCode::BAD_REQUEST
    );

    // 3. Valid secret header + valid Update => parsed Update
    let body = serde_json::to_vec(&json!({ "update_id": 555 })).unwrap();
    let parsed = parse_webhook_request(secret, &ok_headers, &body).expect("valid webhook update");
    assert_eq!(parsed.update_id, 555);

    // Ensure axum Router constructs cleanly for the configured path.
    let _router = webhook_router(
        WebhookConfig {
            path: "/tg/hook".into(),
            secret_token: Some("super_secret_10_4".into()),
            concurrent_handling: false,
        },
        |_u| async move {},
    );
}

#[test]
fn webapp_verify_init_data_validates_hmac_and_freshness() {
    let token = "123456:ABC-DEF";
    let user_json = r#"{"id":42,"first_name":"Ada","username":"ada_lovelace"}"#;
    let auth_date = "1794240000";

    // Build sorted data_check_string manually per Telegram WebApp specification.
    let data_check = format!("auth_date={auth_date}\nquery_id=Q123\nuser={user_json}");
    let mut secret = HmacSha256::new_from_slice(b"WebAppData").unwrap();
    secret.update(token.as_bytes());
    let key = secret.finalize().into_bytes();

    let mut mac = HmacSha256::new_from_slice(&key).unwrap();
    mac.update(data_check.as_bytes());
    let hash_hex = hex::encode(mac.finalize().into_bytes());

    let encoded = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("query_id", "Q123")
        .append_pair("user", user_json)
        .append_pair("auth_date", auth_date)
        .append_pair("hash", &hash_hex)
        .finish();

    let verified = verify_init_data(&encoded, token, Some(300), Some(1_794_240_100))
        .expect("valid initData must verify");
    assert_eq!(verified.auth_date, 1_794_240_000);
    assert_eq!(verified.query_id.as_deref(), Some("Q123"));
    assert_eq!(verified.user.as_ref().map(|u| u.id), Some(42));

    // Stale auth_date beyond max_age_secs must fail.
    assert!(verify_init_data(&encoded, token, Some(60), Some(1_794_240_100)).is_err());
    // Wrong token must fail.
    assert!(verify_init_data(&encoded, "wrong:token", None, None).is_err());
}
