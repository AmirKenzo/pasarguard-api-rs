use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct HttpStatusError {
    pub status: u16,
    pub method: String,
    pub url: String,
    pub body: String,
    pub location: Option<String>,
}

impl HttpStatusError {
    pub fn reason_phrase(&self) -> &'static str {
        reqwest::StatusCode::from_u16(self.status)
            .ok()
            .and_then(|code| code.canonical_reason())
            .unwrap_or("")
    }
}

impl fmt::Display for HttpStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.status / 100 {
            1 => "Informational response",
            3 => "Redirect response",
            4 => "Client error",
            5 => "Server error",
            _ => "Invalid status code",
        };
        write!(
            f,
            "{kind} '{} {}' for url '{}'",
            self.status,
            self.reason_phrase(),
            self.url
        )?;
        if self.status / 100 == 3
            && let Some(location) = &self.location
        {
            write!(f, "\nRedirect location: '{location}'")?;
        }
        write!(
            f,
            "\nFor more information check: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/{}",
            self.status
        )
    }
}

impl fmt::Debug for HttpStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpStatusError")
            .field("status", &self.status)
            .field("method", &self.method)
            .field("url", &self.url)
            .field("body_len", &self.body.len())
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PanelError {
    #[error("{0}")]
    Status(HttpStatusError),
    #[error("panel request failed: {0}")]
    Transport(#[source] reqwest::Error),
    #[error("invalid panel response for {url}: {source}")]
    Decode {
        url: String,
        #[source]
        source: serde_json::Error,
        body: String,
    },
    #[error("invalid panel url: {0}")]
    InvalidUrl(String),
    #[error("panel credential is not a valid header value")]
    InvalidCredential,
    #[error("value of header {0:?} is not a valid header value")]
    InvalidHeader(String),
    #[error("panel re-authentication failed: {0}")]
    Auth(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl PanelError {
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Status(err) => Some(err.status),
            _ => None,
        }
    }

    pub fn body(&self) -> Option<&str> {
        match self {
            Self::Status(err) => Some(&err.body),
            Self::Decode { body, .. } => Some(body),
            _ => None,
        }
    }

    pub fn is_status(&self, status: u16) -> bool {
        self.status() == Some(status)
    }

    pub fn is_unauthorized(&self) -> bool {
        self.is_status(401)
    }

    pub fn is_not_found(&self) -> bool {
        self.is_status(404)
    }
}

pub type PanelResult<T> = Result<T, PanelError>;
