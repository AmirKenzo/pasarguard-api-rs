use super::*;
use crate::datetime::{DateTimeOrInt, PanelDateTime};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

fn default_update_interval() -> Option<i64> {
    Some(12)
}

fn default_support_url() -> Option<String> {
    Some("https://t.me/".into())
}

fn default_profile_title() -> Option<String> {
    Some("Subscription".into())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DownloadLink {
    pub name: String,
    pub url: String,
    pub language: Language,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    #[serde(
        default = "default_some_empty_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_url: Option<String>,
    #[serde(
        default = "default_some_empty_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub import_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Map<String, Value>>,
    #[serde(
        default = "default_some_false",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommended: Option<bool>,
    #[serde(
        default = "default_some_false",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_when_hwid_enabled: Option<bool>,
    pub platform: Platform,
    pub download_links: Vec<DownloadLink>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SubFormatEnable {
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub links: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub links_base64: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub xray: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub wireguard: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub sing_box: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub clash: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub clash_meta: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub outline: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubRule {
    pub pattern: String,
    pub target: ConfigFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Map<String, Value>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(
        default = "default_some_empty_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_prefix: Option<String>,
    #[serde(
        default = "default_update_interval",
        skip_serializing_if = "Option::is_none"
    )]
    pub update_interval: Option<i64>,
    #[serde(
        default = "default_support_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub support_url: Option<String>,
    #[serde(
        default = "default_profile_title",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_title: Option<String>,
    #[serde(
        default = "default_some_empty_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub announce: Option<String>,
    #[serde(
        default = "default_some_empty_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub announce_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Map<String, Value>>,
    pub rules: Vec<SubRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_sub_request: Option<SubFormatEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub allow_browser_config: Option<bool>,
    #[serde(
        default = "default_some_false",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_sub_template: Option<bool>,
    #[serde(
        default = "default_some_false",
        skip_serializing_if = "Option::is_none"
    )]
    pub randomize_order: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_variables: Option<Vec<CustomVariable>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionTemplates {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xray: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SubscriptionUserResponse {
    #[serde(default)]
    pub proxy_settings: Option<ProxyTable>,
    #[serde(default)]
    pub expire: Option<DateTimeOrInt>,
    #[serde(default)]
    pub data_limit: Option<i64>,
    #[serde(default)]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(default)]
    pub on_hold_expire_duration: Option<i64>,
    #[serde(default)]
    pub on_hold_timeout: Option<DateTimeOrInt>,
    #[serde(default)]
    pub group_ids: Option<Vec<i64>>,
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
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}
