use super::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PasskeyInfo {
    pub id: i64,
    pub name: String,
    #[serde(flatten)]
    pub extra: Extra,
}

/// Body of `POST /api/admin/passkey/login/options`.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct PasskeyOptionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

/// Body of the passkey `register/verify` endpoints.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PasskeyRegistrationRequest {
    /// The WebAuthn attestation produced by the authenticator.
    pub credential: Map<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl PasskeyRegistrationRequest {
    pub fn new(credential: Map<String, Value>) -> Self {
        Self {
            credential,
            name: None,
            extra: Extra::new(),
        }
    }
}

/// Body of `POST /api/admin/passkey/login/verify`.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PasskeyAuthenticationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The WebAuthn assertion produced by the authenticator.
    pub credential: Map<String, Value>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl PasskeyAuthenticationRequest {
    pub fn new(credential: Map<String, Value>) -> Self {
        Self {
            username: None,
            credential,
            extra: Extra::new(),
        }
    }
}
