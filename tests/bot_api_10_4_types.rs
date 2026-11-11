use rustgram_api::error::Error;
use rustgram_api::formatting::{
    bold_html, custom_emoji_html, date_time_html, escape_html, escape_markdown_v2, mention_html,
    pre_html,
};
use rustgram_api::types::{
    EphemeralMessageParameters, InputRichMessage, InputWidgetMessageContent, KeyboardButton,
    KeyboardButtonRequestManagedBot, MessageEntity, RichBlock, RichMessage, RichMessageButton,
    Update, UpdateKind, WIDGET_DATA_MAX_BYTES, WIDGET_HTML_MAX_BYTES, WidgetQueryType,
    clamp_widget_cache_time, validate_widget_data, validate_widget_html,
};
use serde_json::json;

#[test]
fn crate_targets_bot_api_10_4() {
    assert_eq!(rustgram_api::BOT_API_VERSION, "10.4");
}

#[test]
fn bot_api_10_4_widget_query_and_shared_widget_deserialization() {
    let tx_update_json = json!({
        "update_id": 9001,
        "widget_query": {
            "id": "wq_tx_1",
            "from": { "id": 777, "is_bot": false, "first_name": "Alice" },
            "message": {
                "message_id": 42,
                "date": 1794240000,
                "chat": { "id": 777, "type": "private", "first_name": "Alice" },
                "widget": { "id": "shared_widget_abc" }
            },
            "chat_instance": "-424242",
            "type": {
                "type": "sent_transaction",
                "wallet": {
                    "address": "EQCD39VS5jcptHL8vMjEXrzGaRcCVYto7HUn4bpAOg8xqB2N",
                    "raw_address": "0:83df5552e63729b472fcbc8c45ebcc6691702558b68ec7527e1ba403a0f31a81",
                    "network": "-239",
                    "public_key": "abcdef",
                    "wallet_app": "tonkeeper"
                },
                "msg_hash": "9f86d081884c7d659a2feaa0c55ad015"
            },
            "data": "{\"item\":\"pro_plan\"}"
        }
    });

    let update: Update = serde_json::from_value(tx_update_json).expect("deserialize widget_query");
    assert_eq!(update.from_user().map(|u| u.id.get()), Some(777));
    assert_eq!(update.chat().map(|c| c.id), Some(777));
    assert_eq!(
        update
            .primary_message()
            .and_then(|m| m.widget.as_ref())
            .and_then(|w| w.id.as_deref()),
        Some("shared_widget_abc")
    );

    match update.kind() {
        UpdateKind::WidgetQuery(wq) => {
            assert_eq!(wq.id, "wq_tx_1");
            assert_eq!(wq.data.as_deref(), Some("{\"item\":\"pro_plan\"}"));
            match &wq.query_type {
                WidgetQueryType::SentTransaction { wallet, msg_hash } => {
                    assert_eq!(msg_hash, "9f86d081884c7d659a2feaa0c55ad015");
                    assert_eq!(wallet.wallet_app.as_deref(), Some("tonkeeper"));
                }
                other => panic!("expected SentTransaction, got {other:?}"),
            }
            assert!(wq.query_type.wallet().is_some());
        }
        other => panic!("expected UpdateKind::WidgetQuery, got {other:?}"),
    }

    let shared_update_json = json!({
        "update_id": 9002,
        "shared_widget_query": {
            "id": "swq_99",
            "from": { "id": 888, "is_bot": false, "first_name": "Bob" },
            "chat": { "id": -100123456, "type": "supergroup", "title": "Devs" },
            "message_id": 500,
            "widget_id": "shared_widget_abc",
            "widget_type": {
                "type": "sign_data",
                "wallet": { "address": "EQ...", "network": "-239" },
                "sign_data": { "signature": "sig123", "timestamp": 1794240100 }
            },
            "data": "refresh_leaderboard"
        }
    });

    let shared_update: Update =
        serde_json::from_value(shared_update_json).expect("deserialize shared_widget_query");
    match shared_update.kind() {
        UpdateKind::SharedWidgetQuery(swq) => {
            assert_eq!(swq.widget_id.as_deref(), Some("shared_widget_abc"));
            assert_eq!(swq.message_id.get(), 500);
            assert!(matches!(swq.widget_type, WidgetQueryType::SignData { .. }));
        }
        other => panic!("expected UpdateKind::SharedWidgetQuery, got {other:?}"),
    }

    let mut inline_widget = InputWidgetMessageContent::new("<div>Hello TON</div>")
        .size(320, 200)
        .shared(true)
        .public_data("{\"v\":1}");
    inline_widget.cache_time = Some(clamp_widget_cache_time(1));
    let encoded = serde_json::to_value(&inline_widget).expect("serialize widget content");
    assert_eq!(encoded["cache_time"], 5);
}

#[test]
fn bot_api_10_4_widget_validators_enforce_limits() {
    assert!(validate_widget_html("<!doctype html><html></html>").is_ok());

    let nul_err = validate_widget_html("<div>\0bad</div>").unwrap_err();
    assert!(matches!(nul_err, Error::InvalidInput(_)));

    let exact_html = "x".repeat(WIDGET_HTML_MAX_BYTES);
    assert!(validate_widget_html(&exact_html).is_ok());

    let oversized_html = "x".repeat(WIDGET_HTML_MAX_BYTES + 1);
    assert!(matches!(
        validate_widget_html(&oversized_html),
        Err(Error::InvalidInput(_))
    ));

    let exact_data = "k".repeat(WIDGET_DATA_MAX_BYTES);
    assert!(validate_widget_data(&exact_data, "public_data").is_ok());

    let oversized_data = "k".repeat(WIDGET_DATA_MAX_BYTES + 1);
    assert!(matches!(
        validate_widget_data(&oversized_data, "chat_data"),
        Err(Error::InvalidInput(_))
    ));

    assert_eq!(clamp_widget_cache_time(-10), 5);
    assert_eq!(clamp_widget_cache_time(5), 5);
    assert_eq!(clamp_widget_cache_time(120), 120);
    assert_eq!(clamp_widget_cache_time(99_999), 3_600);
}

#[test]
fn bot_api_10_1_to_10_4_rich_messages_ephemeral_and_buttons() {
    let rich_json = json!({
        "blocks": [
            { "type": "thinking", "title": "Reasoning", "text": "Checking state", "duration": 2 },
            {
                "type": "table",
                "is_compact": true,
                "rows": [
                    [{ "is_header": true, "text": "Col" }],
                    [{ "text": "Val" }]
                ]
            },
            { "type": "expandable_block_quotation", "text": "Hidden details" },
            {
                "type": "buttons",
                "buttons": [[
                    { "text": "Disabled", "callback_data": "noop", "is_disabled": true }
                ]]
            },
            { "type": "some_future_bot_api_11_block", "foo": "bar" }
        ]
    });

    let rich: RichMessage = serde_json::from_value(rich_json).expect("deserialize RichMessage");
    assert_eq!(rich.blocks.len(), 5);
    assert!(matches!(
        rich.blocks[0],
        RichBlock::Thinking {
            duration: Some(2),
            ..
        }
    ));
    assert!(matches!(
        rich.blocks[1],
        RichBlock::Table {
            is_compact: Some(true),
            ..
        }
    ));
    assert!(matches!(
        rich.blocks[2],
        RichBlock::ExpandableBlockQuotation { .. }
    ));
    assert!(matches!(rich.blocks[3], RichBlock::Buttons { .. }));
    assert!(matches!(rich.blocks[4], RichBlock::Unsupported));

    let disabled_btn = RichMessageButton::callback("Click", "act").disabled(true);
    assert_eq!(disabled_btn.is_disabled, Some(true));

    let input_rich = InputRichMessage::html("<b>Hi</b>").media(vec![]);
    assert_eq!(input_rich.html.as_deref(), Some("<b>Hi</b>"));

    // Bot API 10.4: callback_query_id without receiver_user_id
    let eph = EphemeralMessageParameters::for_callback("cb_10_4").replace_callback_message(true);
    let eph_val = serde_json::to_value(&eph).expect("serialize EphemeralMessageParameters");
    assert!(eph_val.get("receiver_user_id").is_none());
    assert_eq!(eph_val["callback_query_id"], "cb_10_4");
    assert_eq!(eph_val["replace_callback_query_message"], true);

    // Bot API 9.6 / 10.4 managed bot button with custom emoji icon
    let btn = KeyboardButton::request_managed_bot(
        "Create AI Helper",
        KeyboardButtonRequestManagedBot::new(11)
            .suggested_name("Helper")
            .suggested_username("my_helper_ai_bot"),
    )
    .icon_custom_emoji("5368324170671202286");
    let btn_val = serde_json::to_value(&btn).expect("serialize KeyboardButton");
    assert_eq!(btn_val["icon_custom_emoji_id"], "5368324170671202286");
    assert_eq!(
        btn_val["request_managed_bot"]["suggested_username"],
        "my_helper_ai_bot"
    );

    // Bot API 9.5 date_time MessageEntity
    let entity: MessageEntity = serde_json::from_value(json!({
        "type": "date_time",
        "offset": 0,
        "length": 10,
        "unix_time": 1794240000,
        "date_time_format": "wDT"
    }))
    .expect("deserialize date_time entity");
    assert_eq!(entity.unix_time, Some(1_794_240_000));
    assert_eq!(entity.date_time_format.as_deref(), Some("wDT"));
}

#[test]
fn bot_api_10_0_to_10_3_lifecycle_updates_deserialize() {
    let guest_update: Update = serde_json::from_value(json!({
        "update_id": 10,
        "guest_message": {
            "message_id": 1,
            "date": 1794240000,
            "chat": { "id": -10099, "type": "supergroup", "title": "Lounge" },
            "guest_bot_caller_user": { "id": 55, "is_bot": false, "first_name": "GuestUser" },
            "text": "Hello in Guest Mode"
        }
    }))
    .expect("deserialize guest_message update");
    assert!(matches!(guest_update.kind(), UpdateKind::GuestMessage(_)));
    assert_eq!(guest_update.from_user().map(|u| u.id.get()), Some(55));

    let stopped_update: Update = serde_json::from_value(json!({
        "update_id": 11,
        "stopped_message_generation": {
            "chat": { "id": 55, "type": "private", "first_name": "GuestUser" },
            "draft_id": 77
        }
    }))
    .expect("deserialize stopped_message_generation update");
    assert!(matches!(
        stopped_update.kind(),
        UpdateKind::StoppedMessageGeneration(_)
    ));

    let managed_update: Update = serde_json::from_value(json!({
        "update_id": 12,
        "managed_bot": {
            "user": { "id": 10, "is_bot": false, "first_name": "Owner" },
            "bot": { "id": 20, "is_bot": true, "first_name": "ChildBot", "username": "child_bot" }
        }
    }))
    .expect("deserialize managed_bot update");
    assert!(matches!(managed_update.kind(), UpdateKind::ManagedBot(_)));

    let sub_update: Update = serde_json::from_value(json!({
        "update_id": 13,
        "subscription": {
            "user": { "id": 30, "is_bot": false, "first_name": "Subscriber" },
            "invoice_payload": "monthly_pro",
            "state": "active"
        }
    }))
    .expect("deserialize subscription update");
    assert!(matches!(sub_update.kind(), UpdateKind::Subscription(_)));
}

#[test]
fn formatting_helpers_produce_valid_telegram_markup() {
    assert_eq!(escape_html("<a&\"b\">"), "&lt;a&amp;&quot;b&quot;&gt;");
    assert_eq!(bold_html("1 < 2"), "<b>1 &lt; 2</b>");
    assert_eq!(
        pre_html("fn main() {}", Some("rust")),
        "<pre><code class=\"language-rust\">fn main() {}</code></pre>"
    );
    assert_eq!(
        mention_html(12345_i64, "A&B"),
        "<a href=\"tg://user?id=12345\">A&amp;B</a>"
    );
    assert_eq!(
        custom_emoji_html("999", "🔥"),
        "<tg-emoji emoji-id=\"999\">🔥</tg-emoji>"
    );
    assert_eq!(
        date_time_html(1794240000, Some("wDT"), "Mon, Nov 9"),
        "<tg-time unix=\"1794240000\" format=\"wDT\">Mon, Nov 9</tg-time>"
    );
    assert_eq!(escape_markdown_v2("v10.4! (ok)"), r"v10\.4\! \(ok\)");
}
