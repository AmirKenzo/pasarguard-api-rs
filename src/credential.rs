use crate::Secret;

pub const API_KEY_PREFIX: &str = "pg_key_";

/// How requests authenticate: a JWT access token, a `pg_key_` API key, or nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Credential {
    Bearer(Secret),
    ApiKey(Secret),
    Anonymous,
}

impl Credential {
    /// Picks [`Credential::ApiKey`] for `pg_key_` values and [`Credential::Bearer`] otherwise.
    pub fn from_token(value: impl Into<String>) -> Self {
        let value = value.into();
        if value.starts_with(API_KEY_PREFIX) {
            Self::ApiKey(Secret::new(value))
        } else {
            Self::Bearer(Secret::new(value))
        }
    }

    pub fn header(&self) -> Option<(&'static str, String)> {
        match self {
            Self::Bearer(token) if !token.is_empty() => {
                Some(("authorization", format!("Bearer {}", token.expose())))
            }
            Self::ApiKey(key) if !key.is_empty() => Some(("x-api-key", key.expose().to_owned())),
            _ => None,
        }
    }

    pub fn is_api_key(&self) -> bool {
        matches!(self, Self::ApiKey(_))
    }
}
