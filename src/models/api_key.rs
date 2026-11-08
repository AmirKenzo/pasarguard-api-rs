use super::*;
use crate::datetime::PanelDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct APIKeyCreate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RolePermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<PanelDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct APIKeyCreateResponse {
    pub name: String,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub permissions: Option<RolePermissions>,
    #[serde(default = "default_some_true")]
    pub inherit_permissions: Option<bool>,
    #[serde(default)]
    pub expire_date: Option<PanelDateTime>,
    pub id: i64,
    pub admin_id: i64,
    pub created_at: PanelDateTime,
    pub api_key_trimmed: String,
    #[serde(default)]
    pub revoked_at: Option<PanelDateTime>,
    #[serde(default = "default_api_key_status")]
    pub status: Option<APIKeyStatus>,
    #[serde(default = "default_some_false")]
    pub is_expired: Option<bool>,
    pub api_key: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct APIKeyResponse {
    pub name: String,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub permissions: Option<RolePermissions>,
    #[serde(default = "default_some_true")]
    pub inherit_permissions: Option<bool>,
    #[serde(default)]
    pub expire_date: Option<PanelDateTime>,
    pub id: i64,
    pub admin_id: i64,
    pub created_at: PanelDateTime,
    pub api_key_trimmed: String,
    #[serde(default)]
    pub revoked_at: Option<PanelDateTime>,
    #[serde(default = "default_api_key_status")]
    pub status: Option<APIKeyStatus>,
    #[serde(default = "default_some_false")]
    pub is_expired: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_api_key_status() -> Option<APIKeyStatus> {
    Some(APIKeyStatus::Active)
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct APIKeyUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RolePermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<PanelDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<APIKeyStatus>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct APIKeysResponse {
    pub api_keys: Vec<APIKeyResponse>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkAPIKeySelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl BulkAPIKeySelection {
    pub fn ids(ids: Vec<i64>) -> Self {
        Self {
            ids: Some(ids),
            extra: Extra::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RemoveAPIKeysResponse {
    pub api_keys: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}
