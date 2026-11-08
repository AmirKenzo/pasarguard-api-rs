use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Token {
    pub access_token: String,
    #[serde(default = "default_token_type")]
    pub token_type: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_token_type() -> Option<String> {
    Some("bearer".into())
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminNotificationEnable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_usage: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_limit_warning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_limit_warning_percentages: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct AdminCreate {
    pub password: String,
    pub role_id: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord_webhook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AdminAccountStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_enable: Option<AdminNotificationEnable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overrides: Option<RoleLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_variables: Option<Vec<CustomVariable>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct AdminModify {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord_webhook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AdminAccountStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_enable: Option<AdminNotificationEnable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overrides: Option<RoleLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_variables: Option<Vec<CustomVariable>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub username: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminDetails {
    pub username: String,
    pub is_disabled: bool,
    pub is_limited: bool,
    #[serde(default)]
    pub telegram_id: Option<i64>,
    #[serde(default)]
    pub discord_webhook: Option<String>,
    #[serde(default)]
    pub sub_domain: Option<String>,
    #[serde(default)]
    pub profile_title: Option<String>,
    #[serde(default)]
    pub support_url: Option<String>,
    #[serde(default)]
    pub notification_enable: Option<AdminNotificationEnable>,
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default = "default_some_zero")]
    pub total_users: Option<i64>,
    #[serde(default = "default_some_zero")]
    pub used_traffic: Option<i64>,
    #[serde(default)]
    pub data_limit: Option<i64>,
    #[serde(default = "default_admin_status")]
    pub status: Option<AdminStatus>,
    #[serde(default)]
    pub sub_template: Option<String>,
    #[serde(default)]
    pub lifetime_used_traffic: Option<i64>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub role: Option<AdminRoleData>,
    #[serde(default)]
    pub permission_overrides: Option<RoleLimits>,
    #[serde(default)]
    pub custom_variables: Option<Vec<CustomVariable>>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_admin_status() -> Option<AdminStatus> {
    Some(AdminStatus::Active)
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminsResponse {
    pub admins: Vec<AdminDetails>,
    pub total: i64,
    pub active: i64,
    pub disabled: i64,
    pub limited: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminSimple {
    pub id: i64,
    pub username: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminsSimpleResponse {
    pub admins: Vec<AdminSimple>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

pub type Admin = AdminDetails;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminContactInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_webhook: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_enable: Option<AdminNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_variables: Option<Vec<CustomVariable>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BodyAdminToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

pub type BodyAdminTokenApiAdminTokenPost = BodyAdminToken;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkAdminSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl BulkAdminSelection {
    pub fn ids(ids: Vec<i64>) -> Self {
        Self {
            ids: Some(ids),
            extra: Extra::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BulkAdminsActionResponse {
    pub admins: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

pub type RemoveAdminsResponse = BulkAdminsActionResponse;
