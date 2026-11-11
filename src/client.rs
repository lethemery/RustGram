//! HTTP client (`Bot`) and request execution layer for the Telegram Bot API 10.4.

use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::{debug, trace, warn};

use crate::error::{ApiError, Error, ResponseParameters, Result};
use crate::types::InputFile;

/// Default Telegram Bot API origin.
pub const DEFAULT_API_URL: &str = "https://api.telegram.org";

/// Telegram envelope returned by every Bot API method.
#[derive(Debug, serde::Deserialize)]
struct ApiResponse<T> {
    ok: bool,
    result: Option<T>,
    error_code: Option<i32>,
    description: Option<String>,
    parameters: Option<ResponseParameters>,
}

/// Configuration builder for [`Bot`].
#[derive(Debug, Clone)]
pub struct BotBuilder {
    token: String,
    base_url: String,
    test_environment: bool,
    timeout: Duration,
    max_rate_limit_retries: u32,
    max_retry_after: Duration,
    client: Option<reqwest::Client>,
}

impl BotBuilder {
    /// Creates a new builder for the given bot token.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: DEFAULT_API_URL.to_owned(),
            test_environment: false,
            timeout: Duration::from_secs(60),
            max_rate_limit_retries: 0,
            max_retry_after: Duration::from_secs(30),
            client: None,
        }
    }

    /// Overrides the Bot API server base URL (useful for Local Bot API servers or tests).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        let mut s = url.into();
        while s.ends_with('/') {
            s.pop();
        }
        self.base_url = s;
        self
    }

    /// Enables Telegram's test server path prefix (`/bot<token>/test/<method>`).
    pub fn test_environment(mut self, enabled: bool) -> Self {
        self.test_environment = enabled;
        self
    }

    /// Sets the HTTP request timeout (should exceed `getUpdates` long-polling `timeout`).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Automatically waits and retries when Telegram returns `429 Too Many Requests`
    /// with `retry_after <= max_retry_after`.
    pub fn retry_rate_limits(mut self, max_retries: u32, max_retry_after: Duration) -> Self {
        self.max_rate_limit_retries = max_retries;
        self.max_retry_after = max_retry_after;
        self
    }

    /// Supplies a pre-configured `reqwest::Client`.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Finalizes and constructs the [`Bot`] client.
    pub fn build(self) -> Result<Bot> {
        let trimmed = self.token.trim();
        if trimmed.is_empty() {
            return Err(Error::InvalidInput("bot token must not be empty".into()));
        }

        let http = match self.client {
            Some(c) => c,
            None => reqwest::Client::builder().timeout(self.timeout).build()?,
        };

        Ok(Bot {
            inner: Arc::new(BotInner {
                token: trimmed.to_owned(),
                base_url: self.base_url,
                test_environment: self.test_environment,
                max_rate_limit_retries: self.max_rate_limit_retries,
                max_retry_after: self.max_retry_after,
                http,
            }),
        })
    }
}

#[derive(Debug)]
struct BotInner {
    token: String,
    base_url: String,
    test_environment: bool,
    max_rate_limit_retries: u32,
    max_retry_after: Duration,
    http: reqwest::Client,
}

/// Cheaply cloneable asynchronous client for the Telegram Bot API 10.4.
#[derive(Debug, Clone)]
pub struct Bot {
    inner: Arc<BotInner>,
}

impl Bot {
    /// Creates a new [`Bot`] from a bot token (or reads `TELEGRAM_BOT_TOKEN` with [`Bot::from_env`]).
    ///
    /// # Panics
    /// Panics only if TLS backend initialization fails or the token is empty.
    /// Use [`Bot::try_new`] or [`Bot::builder`] for fallible construction.
    pub fn new(token: impl Into<String>) -> Self {
        Self::try_new(token).expect("failed to construct Bot client")
    }

    /// Fallible constructor for [`Bot`].
    pub fn try_new(token: impl Into<String>) -> Result<Self> {
        BotBuilder::new(token).build()
    }

    /// Creates a [`Bot`] from the `BOT_TOKEN` or `TELEGRAM_BOT_TOKEN` environment variable.
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("BOT_TOKEN")
            .or_else(|_| std::env::var("TELEGRAM_BOT_TOKEN"))
            .map_err(|_| {
                Error::InvalidInput(
                    "neither BOT_TOKEN nor TELEGRAM_BOT_TOKEN environment variable is set".into(),
                )
            })?;
        Self::try_new(token)
    }

    /// Starts building a customized [`Bot`].
    pub fn builder(token: impl Into<String>) -> BotBuilder {
        BotBuilder::new(token)
    }

    /// Returns the bot token string.
    #[inline]
    pub fn token(&self) -> &str {
        &self.inner.token
    }

    /// Returns the configured API base URL.
    #[inline]
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// Returns `true` if configured for Telegram's test environment.
    #[inline]
    pub fn is_test_environment(&self) -> bool {
        self.inner.test_environment
    }

    /// Returns a reference to the underlying `reqwest::Client`.
    #[inline]
    pub fn http_client(&self) -> &reqwest::Client {
        &self.inner.http
    }

    /// Builds the full HTTP endpoint URL for `method`.
    pub fn method_url(&self, method: &str) -> String {
        if self.inner.test_environment {
            format!(
                "{}/bot{}/test/{method}",
                self.inner.base_url, self.inner.token
            )
        } else {
            format!("{}/bot{}/{method}", self.inner.base_url, self.inner.token)
        }
    }

    /// Builds the file download URL for a `file_path` obtained from `getFile`.
    pub fn file_url(&self, file_path: &str) -> String {
        let clean = file_path.trim_start_matches('/');
        if self.inner.test_environment {
            format!(
                "{}/file/bot{}/test/{clean}",
                self.inner.base_url, self.inner.token
            )
        } else {
            format!(
                "{}/file/bot{}/{clean}",
                self.inner.base_url, self.inner.token
            )
        }
    }

    /// Calls a Bot API method using a JSON request body.
    pub async fn call_json<P, R>(&self, method: &str, payload: &P) -> Result<R>
    where
        P: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let body = serde_json::to_vec(payload)?;
        let url = self.method_url(method);
        let mut attempt = 0u32;

        loop {
            trace!(method, "sending JSON request");
            let response = self
                .inner
                .http
                .post(&url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(body.clone())
                .send()
                .await?;

            match self.decode_response::<R>(method, response).await {
                Ok(value) => return Ok(value),
                Err(Error::Api(api_err))
                    if api_err.is_rate_limited() && attempt < self.inner.max_rate_limit_retries =>
                {
                    if let Some(delay) = api_err.retry_after()
                        && delay <= self.inner.max_retry_after
                    {
                        attempt += 1;
                        warn!(
                            method,
                            attempt,
                            retry_after_secs = delay.as_secs(),
                            "rate limited by Telegram, retrying after delay"
                        );
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err(Error::Api(api_err));
                }
                Err(err) => return Err(err),
            }
        }
    }

    /// Calls a Bot API method using `multipart/form-data`.
    pub async fn call_multipart<R>(&self, method: &str, form: Form) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let url = self.method_url(method);
        debug!(method, "sending multipart request");
        let response = self.inner.http.post(&url).multipart(form).send().await?;
        self.decode_response::<R>(method, response).await
    }

    /// Downloads a file by its `file_path` (from [`getFile`](Self::get_file)) into memory.
    pub async fn download_file(&self, file_path: &str) -> Result<Bytes> {
        let url = self.file_url(file_path);
        let response = self.inner.http.get(&url).send().await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::UnexpectedResponse {
                status: status.as_u16(),
                body,
            });
        }
        Ok(response.bytes().await?)
    }

    /// Downloads a file by `file_id` (performs `getFile` then fetches the bytes).
    pub async fn download_file_by_id(&self, file_id: &str) -> Result<Bytes> {
        let file = self.get_file(file_id).send().await?;
        let path = file.file_path.ok_or_else(|| {
            Error::InvalidInput(format!("file `{file_id}` returned no file_path"))
        })?;
        self.download_file(&path).await
    }

    async fn decode_response<R: DeserializeOwned>(
        &self,
        method: &str,
        response: reqwest::Response,
    ) -> Result<R> {
        let status = response.status();
        let bytes = response.bytes().await?;

        let parsed: ApiResponse<R> = match serde_json::from_slice(&bytes) {
            Ok(p) => p,
            Err(err) => {
                let snippet = String::from_utf8_lossy(&bytes);
                let body = snippet.chars().take(512).collect::<String>();
                if !status.is_success() {
                    return Err(Error::UnexpectedResponse {
                        status: status.as_u16(),
                        body,
                    });
                }
                return Err(Error::Json(err));
            }
        };

        if parsed.ok {
            parsed.result.ok_or_else(|| Error::UnexpectedResponse {
                status: status.as_u16(),
                body: format!("Bot API `{method}` returned ok=true with missing result"),
            })
        } else {
            Err(Error::Api(ApiError::new(
                parsed.error_code.unwrap_or(status.as_u16() as i32),
                parsed
                    .description
                    .unwrap_or_else(|| "Unknown Telegram Bot API error".to_owned()),
                parsed.parameters,
            )))
        }
    }
}

/// Helper for constructing `multipart/form-data` payloads cleanly without code duplication.
pub(crate) struct MultipartBuilder {
    form: Form,
    attach_counter: usize,
}

impl MultipartBuilder {
    pub(crate) fn new() -> Self {
        Self {
            form: Form::new(),
            attach_counter: 0,
        }
    }

    pub(crate) fn push_owned_text(&mut self, name: String, value: String) {
        self.form = self.take_form().text(name, value);
    }

    pub(crate) fn take_form(&mut self) -> Form {
        std::mem::replace(&mut self.form, Form::new())
    }

    async fn build_part(filename: String, bytes: Vec<u8>) -> Result<Part> {
        let mime = mime_guess::from_path(&filename)
            .first_or_octet_stream()
            .essence_str()
            .to_owned();
        Part::bytes(bytes)
            .file_name(filename)
            .mime_str(&mime)
            .map_err(|e| Error::InvalidInput(e.to_string()))
    }

    /// Adds an [`InputFile`] field (`file_id`, URL, or uploaded `Part` under `field_name`).
    pub(crate) async fn push_file(&mut self, field_name: &str, file: &InputFile) -> Result<()> {
        match file {
            InputFile::FileId(id) => {
                self.form = self.take_form().text(field_name.to_owned(), id.clone());
            }
            InputFile::Url(url) => {
                self.form = self.take_form().text(field_name.to_owned(), url.clone());
            }
            InputFile::Memory { filename, data } => {
                let part = Self::build_part(filename.clone(), data.to_vec()).await?;
                self.form = self.take_form().part(field_name.to_owned(), part);
            }
            InputFile::Path(path) => {
                let bytes = tokio::fs::read(path).await?;
                let filename = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "file.bin".to_owned());
                let part = Self::build_part(filename, bytes).await?;
                self.form = self.take_form().part(field_name.to_owned(), part);
            }
        }
        Ok(())
    }

    /// Resolves an [`InputFile`] into either its string handle or `attach://file_N`,
    /// appending the file part if upload is needed.
    #[allow(dead_code)]
    pub(crate) async fn attach_or_ref(&mut self, file: &InputFile) -> Result<String> {
        match file {
            InputFile::FileId(id) => Ok(id.clone()),
            InputFile::Url(url) => Ok(url.clone()),
            InputFile::Memory { .. } | InputFile::Path(_) => {
                let key = format!("file_{}", self.attach_counter);
                self.attach_counter += 1;
                self.push_file(&key, file).await?;
                Ok(format!("attach://{key}"))
            }
        }
    }

    pub(crate) fn finish(self) -> Form {
        self.form
    }
}
