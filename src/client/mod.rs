use std::sync::OnceLock;
use std::time::Duration;

use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, HeaderMap, HeaderName,
    HeaderValue, LOCATION, USER_AGENT,
};
use reqwest::{Method, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::credential::Credential;
use crate::datetime::PanelDateTime;
use crate::encoding::{py_splitlines, py_strip, quote_path, urlencode};
use crate::error::{HttpStatusError, PanelError, PanelResult};
use crate::models::*;
use crate::query::Query;

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
pub const DEFAULT_USER_AGENT: &str = concat!("pasarguard-api-rs/", env!("CARGO_PKG_VERSION"));

/// Transport settings for [`build_http_client_with`].
#[derive(Clone, Debug)]
pub struct HttpOptions {
    /// Connect and read timeout.
    pub timeout: Duration,
    /// Verify TLS certificates. Off by default, like the Python client, because many panels
    /// run on self-signed certificates.
    pub verify: bool,
    /// Proxy for every request, e.g. `http://user:pass@127.0.0.1:8080` or
    /// `socks5://127.0.0.1:1080`.
    pub proxy: Option<String>,
    pub user_agent: String,
}

impl Default for HttpOptions {
    fn default() -> Self {
        Self {
            timeout: REQUEST_TIMEOUT,
            verify: false,
            proxy: None,
            user_agent: DEFAULT_USER_AGENT.to_owned(),
        }
    }
}

/// Builds an HTTP client with the default [`HttpOptions`].
pub fn build_http_client() -> PanelResult<reqwest::Client> {
    build_http_client_with(&HttpOptions::default())
}

/// Builds an HTTP client for [`PanelClient::with_http`].
pub fn build_http_client_with(options: &HttpOptions) -> PanelResult<reqwest::Client> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&options.user_agent).map_err(|_| {
            PanelError::InvalidUrl(format!("invalid user agent: {:?}", options.user_agent))
        })?,
    );
    let mut builder = reqwest::Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(!options.verify)
        .connect_timeout(options.timeout)
        .read_timeout(options.timeout)
        .redirect(reqwest::redirect::Policy::none())
        .http1_only()
        .pool_max_idle_per_host(20)
        .pool_idle_timeout(Duration::from_secs(5));
    if let Some(proxy) = &options.proxy {
        builder = builder.proxy(reqwest::Proxy::all(proxy).map_err(PanelError::Transport)?);
    }
    builder.build().map_err(PanelError::Transport)
}

/// A process-wide client with the default [`HttpOptions`], reused by [`PanelClient::new`].
pub fn shared_http_client() -> PanelResult<reqwest::Client> {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    if let Some(client) = CLIENT.get() {
        return Ok(client.clone());
    }
    let built = build_http_client()?;
    Ok(CLIENT.get_or_init(|| built).clone())
}

impl SimpleListParams {
    pub fn all() -> Self {
        Self {
            all: Some(true),
            ..Self::default()
        }
    }
}

enum Body {
    None,
    Json(Vec<u8>),
    Form(String),
}

struct RawResponse {
    url: String,
    body: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct PanelClient {
    base_url: String,
    http: reqwest::Client,
    credential: Credential,
}

impl PanelClient {
    pub fn new(base_url: &str, credential: Credential) -> PanelResult<Self> {
        Ok(Self::with_http(shared_http_client()?, base_url, credential))
    }

    pub fn with_http(http: reqwest::Client, base_url: &str, credential: Credential) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_owned(),
            http,
            credential,
        }
    }

    pub fn with_credential(&self, credential: Credential) -> Self {
        Self {
            base_url: self.base_url.clone(),
            http: self.http.clone(),
            credential,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn credential(&self) -> &Credential {
        &self.credential
    }

    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }

    fn resolve_url(&self, target: &str, query: &Query) -> PanelResult<Url> {
        let lower = target.get(..8).unwrap_or(target).to_ascii_lowercase();
        let mut url = if lower.starts_with("http://") || lower.starts_with("https://") {
            target.to_owned()
        } else {
            format!("{}{}", self.base_url, quote_path(target))
        };
        if !query.is_empty() {
            url.push(if url.contains('?') { '&' } else { '?' });
            url.push_str(&query.encode());
        }
        Url::parse(&url).map_err(|err| PanelError::InvalidUrl(format!("{err}")))
    }

    async fn send(
        &self,
        method: Method,
        target: &str,
        query: Query,
        body: Body,
        authenticated: bool,
    ) -> PanelResult<RawResponse> {
        self.send_with_headers(method, target, query, body, authenticated, &[])
            .await
            .map(|(_, raw)| raw)
    }

    /// Like [`Self::send`], with extra request headers (lowercase names) and the response
    /// headers returned alongside the body.
    async fn send_with_headers(
        &self,
        method: Method,
        target: &str,
        query: Query,
        body: Body,
        authenticated: bool,
        headers: &[(&'static str, &str)],
    ) -> PanelResult<(HeaderMap, RawResponse)> {
        let url = self.resolve_url(target, &query)?;
        let url_text = url.to_string();
        let sends_length = matches!(method, Method::POST | Method::PUT | Method::PATCH);
        let mut request = self.http.request(method.clone(), url);
        if authenticated && let Some((name, value)) = self.credential.header() {
            let mut value =
                HeaderValue::from_str(&value).map_err(|_| PanelError::InvalidCredential)?;
            value.set_sensitive(true);
            request = request.header(HeaderName::from_static(name), value);
        }
        for (name, value) in headers {
            let mut value = HeaderValue::from_str(value)
                .map_err(|_| PanelError::InvalidHeader((*name).to_owned()))?;
            value.set_sensitive(true);
            request = request.header(HeaderName::from_static(name), value);
        }
        request = match body {
            Body::Json(bytes) => request.header(CONTENT_TYPE, "application/json").body(bytes),
            Body::Form(text) => request
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(text),
            // Body-less writes still carry `Content-Length: 0`.
            Body::None if sends_length => request.header(CONTENT_LENGTH, "0"),
            Body::None => request,
        };

        let response = request.send().await.map_err(PanelError::Transport)?;
        let status = response.status();
        let response_headers = response.headers().clone();
        let bytes = response
            .bytes()
            .await
            .map_err(PanelError::Transport)?
            .to_vec();
        if !status.is_success() {
            return Err(PanelError::Status(HttpStatusError {
                status: status.as_u16(),
                method: method.to_string(),
                url: url_text,
                body: String::from_utf8_lossy(&bytes).into_owned(),
                location: response_headers
                    .get(LOCATION)
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_owned),
            }));
        }
        Ok((
            response_headers,
            RawResponse {
                url: url_text,
                body: bytes,
            },
        ))
    }

    /// Sends an authenticated request to any panel endpoint and decodes the JSON answer.
    ///
    /// An escape hatch for endpoints newer than this crate, or for sending a body as raw
    /// JSON instead of a typed model.
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: Query,
        body: Option<&serde_json::Value>,
    ) -> PanelResult<T> {
        let body = match body {
            Some(value) => json_body(value)?,
            None => Body::None,
        };
        self.call(method, path, query, body).await
    }

    async fn call<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: Query,
        body: Body,
    ) -> PanelResult<T> {
        let raw = self.send(method, path, query, body, true).await?;
        decode(raw)
    }

    async fn call_unit(&self, method: Method, path: &str, query: Query) -> PanelResult<()> {
        self.send(method, path, query, Body::None, true)
            .await
            .map(|_| ())
    }

    async fn call_any(&self, method: Method, path: &str, body: Body) -> PanelResult<AnyPayload> {
        let raw = self.send(method, path, Query::new(), body, true).await?;
        Ok(AnyPayload::from_body(&raw.body))
    }

    /// Logs in with an admin username and password and returns an access token.
    pub async fn get_token(&self, username: &str, password: &str) -> PanelResult<Token> {
        let form = urlencode([
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
            ("scope", ""),
            ("client_id", ""),
            ("client_secret", ""),
        ]);
        let raw = self
            .send(
                Method::POST,
                "/api/admin/token",
                Query::new(),
                Body::Form(form),
                false,
            )
            .await?;
        decode(raw)
    }

    /// Alias of [`PanelClient::get_token`].
    pub async fn admin_token(&self, username: &str, password: &str) -> PanelResult<Token> {
        self.get_token(username, password).await
    }
}

fn json_body<T: Serialize>(value: &T) -> PanelResult<Body> {
    serde_json::to_vec(value)
        .map(Body::Json)
        .map_err(|source| PanelError::Decode {
            url: String::new(),
            source,
            body: String::new(),
        })
}

fn decode<T: DeserializeOwned>(raw: RawResponse) -> PanelResult<T> {
    serde_json::from_slice(&raw.body).map_err(|source| PanelError::Decode {
        url: raw.url,
        source,
        body: String::from_utf8_lossy(&raw.body).into_owned(),
    })
}

mod admin;
mod admin_role;
mod api_key;
mod client_template;
mod compat;
mod core;
mod default;
mod groups;
mod host;
mod node;
mod params;
mod settings;
mod setup;
mod subscription;
mod system;
mod user;
mod user_hwid;
mod user_template;

pub use admin::*;
pub use admin_role::*;
pub use api_key::*;
pub use client_template::*;
pub use core::*;
pub use groups::*;
pub use host::*;
pub use node::*;
pub use params::*;
pub use subscription::*;
pub use user::*;
pub use user_template::*;
