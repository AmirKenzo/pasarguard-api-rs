use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct General {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_method: Option<ShadowsocksMethods>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_hold_timeout_action: Option<OnHoldTimeoutAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_variables: Option<Vec<CustomVariable>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_health: Option<NodeHealthSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_nodes_on_shutdown: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HWIDSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_hwid_for_manual_sub: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RoleHWIDSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_hwid_for_manual_sub: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<HWIDMode>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NotificationChannel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_chat_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_topic_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_webhook_url: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NotificationChannels {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_role: Option<NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core: Option<NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_template: Option<NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<NotificationChannel>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NotificationEnable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<AdminNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_role: Option<BaseNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core: Option<BaseNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<BaseNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<HostNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<NodeNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_template: Option<BaseNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<BaseNotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_left: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage_reached: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_telegram: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_discord: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_api_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_chat_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_topic_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord_webhook_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<NotificationChannels>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    pub max_retries: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SettingsSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram: Option<Telegram>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_enable: Option<NotificationEnable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hwid: Option<HWIDSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub general: Option<General>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleanup: Option<CleanupSettings>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Telegram {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<RunMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mini_app_login: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mini_app_web_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub for_admins_only: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<WebhookInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_left: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_percent: Option<Vec<i64>>,
    pub timeout: i64,
    pub recurrent: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub secret: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_node_usage: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_node_stats: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_usage_clean_chart_data: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_outbound_usage: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeHealthSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_threshold: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recover_threshold: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe_timeout: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_backoff: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

/// Automatic retention periods. `None` keeps the corresponding data indefinitely.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CleanupSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_users_retention_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_limited_users: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_history_retention_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_stats_retention_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_rollup_hourly_after_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_rollup_half_day_after_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_rollup_daily_after_days: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}
