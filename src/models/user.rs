use super::*;
use crate::datetime::{DateTimeOrInt, PanelDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NextPlanModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_template_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_remaining_traffic: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct UserCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_settings: Option<ProxyTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<DateTimeOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_expire_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_timeout: Option<DateTimeOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete_in_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_plan: Option<NextPlanModel>,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl UserCreate {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct UserModify {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_settings: Option<ProxyTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<DateTimeOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_expire_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_timeout: Option<DateTimeOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete_in_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_plan: Option<NextPlanModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserResponse {
    #[serde(default)]
    pub proxy_settings: Option<ProxyTable>,
    #[serde(default)]
    pub expire: Option<DateTimeOrInt>,
    #[serde(default)]
    pub data_limit: Option<i64>,
    #[serde(default)]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub on_hold_expire_duration: Option<i64>,
    #[serde(default)]
    pub on_hold_timeout: Option<DateTimeOrInt>,
    #[serde(default)]
    pub group_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub auto_delete_in_days: Option<i64>,
    #[serde(default)]
    pub hwid_limit: Option<i64>,
    #[serde(default)]
    pub next_plan: Option<NextPlanModel>,
    pub id: i64,
    pub username: String,
    pub status: UserStatus,
    pub used_traffic: i64,
    #[serde(default = "default_some_zero")]
    pub lifetime_used_traffic: Option<i64>,
    pub created_at: PanelDateTime,
    #[serde(default)]
    pub edit_at: Option<PanelDateTime>,
    #[serde(default)]
    pub online_at: Option<PanelDateTime>,
    #[serde(default = "default_some_empty_string")]
    pub subscription_url: Option<String>,
    #[serde(default)]
    pub admin: Option<AdminBase>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserSimple {
    pub id: i64,
    pub username: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UsersSimpleResponse {
    pub users: Vec<UserSimple>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserSubscriptionUpdateSchema {
    pub created_at: PanelDateTime,
    pub user_agent: String,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub hwid: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserSubscriptionUpdateList {
    #[serde(default)]
    pub updates: Option<Vec<UserSubscriptionUpdateSchema>>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserUsageStat {
    pub total_traffic: i64,
    pub period_start: PanelDateTime,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserUsageStatsList {
    #[serde(default)]
    pub period: Option<Period>,
    pub start: PanelDateTime,
    pub end: PanelDateTime,
    #[serde(deserialize_with = "ordered_entries")]
    pub stats: Vec<(String, Vec<UserUsageStat>)>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkUsersSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl BulkUsersSelection {
    pub fn ids(ids: Vec<i64>) -> Self {
        Self {
            ids: Some(ids),
            extra: Extra::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkUsersSetOwner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    pub admin_username: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admins: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<UserStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<PanelDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_before: Option<PanelDateTime>,
    pub amount: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BulkUsersActionResponse {
    pub users: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

pub type RemoveUsersResponse = BulkUsersActionResponse;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkUsersApplyTemplate {
    pub user_template_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BulkUsersCreateResponse {
    #[serde(default)]
    pub subscription_urls: Option<Vec<String>>,
    #[serde(default = "default_some_zero")]
    pub created: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkUsersFromTemplate {
    pub user_template_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<UsernameGenerationStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_number: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkUsersProxy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admins: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<UserStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<PanelDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_before: Option<PanelDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<ShadowsocksMethods>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct CreateUserFromTemplate {
    pub user_template_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub username: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ModifyUserByTemplate {
    pub user_template_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserCountMetricStat {
    pub count: i64,
    pub period_start: PanelDateTime,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserCountMetricStatsList {
    #[serde(default)]
    pub period: Option<Period>,
    pub start: PanelDateTime,
    pub end: PanelDateTime,
    pub metric: UserCountMetric,
    #[serde(deserialize_with = "ordered_entries")]
    pub stats: Vec<(String, Vec<UserCountMetricStat>)>,
    #[serde(default)]
    pub count_during_period: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserNotificationEnable {
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub modify: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub status_change: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub reset_data_usage: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub data_reset_by_next: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub subscription_revoked: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct UserStatusToggle {
    pub disabled: bool,
    #[serde(flatten)]
    pub extra: Extra,
}

impl UserStatusToggle {
    pub fn new(disabled: bool) -> Self {
        Self {
            disabled,
            extra: Extra::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserSubscriptionUpdateChartSegment {
    pub name: String,
    pub count: i64,
    pub percentage: f64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserSubscriptionUpdateChartStat {
    pub period_start: PanelDateTime,
    pub agent: String,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserSubscriptionUpdateChart {
    #[serde(default)]
    pub period: Option<Period>,
    pub start: PanelDateTime,
    pub end: PanelDateTime,
    pub total: i64,
    #[serde(default)]
    pub segments: Option<Vec<UserSubscriptionUpdateChartSegment>>,
    #[serde(default)]
    pub stats: Option<Vec<UserSubscriptionUpdateChartStat>>,
    #[serde(flatten)]
    pub extra: Extra,
}
