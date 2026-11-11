//! Telegram Mini App (WebApp) `initData` HMAC-SHA256 verification (enabled via `webapp` feature).

use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use url::form_urlencoded;

use crate::error::{Error, Result};

type HmacSha256 = Hmac<Sha256>;

/// Validated Mini App user descriptor decoded from the `user` field of `initData`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WebAppUser {
    /// Unique identifier for the user.
    pub id: i64,
    /// First name of the user.
    pub first_name: String,
    /// Last name of the user.
    #[serde(default)]
    pub last_name: Option<String>,
    /// Username of the user.
    #[serde(default)]
    pub username: Option<String>,
    /// IETF language tag.
    #[serde(default)]
    pub language_code: Option<String>,
    /// `true`, if the user is a Telegram Premium user.
    #[serde(default)]
    pub is_premium: Option<bool>,
    /// `true`, if the user allows the bot to write messages.
    #[serde(default)]
    pub allows_write_to_pm: Option<bool>,
}

/// Parsed and HMAC-verified Telegram Mini App `initData` payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedInitData {
    /// Unix timestamp when `initData` was generated (`auth_date`).
    pub auth_date: i64,
    /// Optional inline/WebApp query identifier (`query_id`).
    pub query_id: Option<String>,
    /// Decoded user object if present.
    pub user: Option<WebAppUser>,
    /// Raw key-value pairs (excluding `hash`).
    pub fields: BTreeMap<String, String>,
}

/// Verifies Telegram WebApp `initData` using the official `"WebAppData"` HMAC-SHA256 derivation.
///
/// If `max_age_secs` and `now_unix` are provided, also validates that `now_unix - auth_date <= max_age_secs`.
pub fn verify_init_data(
    init_data: &str,
    bot_token: &str,
    max_age_secs: Option<i64>,
    now_unix: Option<i64>,
) -> Result<VerifiedInitData> {
    let mut fields = BTreeMap::new();
    let mut received_hash: Option<String> = None;

    for (key, value) in form_urlencoded::parse(init_data.as_bytes()) {
        if key == "hash" {
            received_hash = Some(value.into_owned());
        } else {
            fields.insert(key.into_owned(), value.into_owned());
        }
    }

    let hash_hex = received_hash
        .ok_or_else(|| Error::WebAppAuth("missing `hash` parameter in initData".into()))?;
    let expected_bytes = hex::decode(&hash_hex)
        .map_err(|_| Error::WebAppAuth("invalid hex encoding in `hash`".into()))?;

    let data_check_string = fields
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut secret_mac =
        HmacSha256::new_from_slice(b"WebAppData").map_err(|e| Error::WebAppAuth(e.to_string()))?;
    secret_mac.update(bot_token.as_bytes());
    let secret_key = secret_mac.finalize().into_bytes();

    let mut check_mac =
        HmacSha256::new_from_slice(&secret_key).map_err(|e| Error::WebAppAuth(e.to_string()))?;
    check_mac.update(data_check_string.as_bytes());
    check_mac
        .verify_slice(&expected_bytes)
        .map_err(|_| Error::WebAppAuth("HMAC-SHA256 signature mismatch".into()))?;

    let auth_date = fields
        .get("auth_date")
        .ok_or_else(|| Error::WebAppAuth("missing `auth_date` in initData".into()))?
        .parse::<i64>()
        .map_err(|_| Error::WebAppAuth("invalid `auth_date` integer".into()))?;

    if let (Some(max_age), Some(now)) = (max_age_secs, now_unix)
        && now.saturating_sub(auth_date) > max_age
    {
        return Err(Error::WebAppAuth("initData has expired".into()));
    }

    let user = match fields.get("user") {
        Some(raw) => Some(
            serde_json::from_str::<WebAppUser>(raw)
                .map_err(|e| Error::WebAppAuth(format!("invalid `user` JSON: {e}")))?,
        ),
        None => None,
    };

    let query_id = fields.get("query_id").cloned();

    Ok(VerifiedInitData {
        auth_date,
        query_id,
        user,
        fields,
    })
}
