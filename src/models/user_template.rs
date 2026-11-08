use super::*;
use serde::{Deserialize, Serialize};

fn default_no_reset() -> Option<DataLimitResetStrategy> {
    Some(DataLimitResetStrategy::NoReset)
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkUserTemplateSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl BulkUserTemplateSelection {
    pub fn ids(ids: Vec<i64>) -> Self {
        Self {
            ids: Some(ids),
            extra: Extra::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BulkUserTemplatesActionResponse {
    pub templates: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RemoveUserTemplatesResponse {
    pub templates: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct UserTemplateCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_suffix: Option<String>,
    pub group_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_settings: Option<ExtraSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatusCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_usages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct UserTemplateModify {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_settings: Option<ExtraSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatusCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_usages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserTemplateResponse {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub data_limit: Option<i64>,
    #[serde(default)]
    pub hwid_limit: Option<i64>,
    #[serde(default)]
    pub expire_duration: Option<i64>,
    #[serde(default)]
    pub username_prefix: Option<String>,
    #[serde(default)]
    pub username_suffix: Option<String>,
    pub group_ids: Vec<i64>,
    #[serde(default)]
    pub extra_settings: Option<ExtraSettings>,
    #[serde(default)]
    pub status: Option<UserStatusCreate>,
    #[serde(default)]
    pub reset_usages: Option<bool>,
    #[serde(default)]
    pub on_hold_timeout: Option<i64>,
    #[serde(default = "default_no_reset")]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(default)]
    pub is_disabled: Option<bool>,
    pub id: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserTemplateSimple {
    pub id: i64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserTemplatesSimpleResponse {
    pub templates: Vec<UserTemplateSimple>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}
