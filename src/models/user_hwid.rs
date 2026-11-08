use super::*;
use crate::datetime::PanelDateTime;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserHWIDResponse {
    pub id: i64,
    pub hwid: String,
    #[serde(default)]
    pub device_os: Option<String>,
    #[serde(default)]
    pub os_version: Option<String>,
    #[serde(default)]
    pub device_model: Option<String>,
    pub created_at: PanelDateTime,
    pub last_used_at: PanelDateTime,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserHWIDListResponse {
    pub hwids: Vec<UserHWIDResponse>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}
