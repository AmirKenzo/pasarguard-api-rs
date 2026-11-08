use super::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct InboundSummary {
    pub tag: String,
    pub protocol: String,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SystemStats {
    pub version: String,
    pub uptime_seconds: i64,
    #[serde(default)]
    pub mem_total: Option<i64>,
    #[serde(default)]
    pub mem_used: Option<i64>,
    #[serde(default)]
    pub disk_total: Option<i64>,
    #[serde(default)]
    pub disk_used: Option<i64>,
    #[serde(default)]
    pub cpu_cores: Option<i64>,
    #[serde(default)]
    pub cpu_usage: Option<f64>,
    pub total_user: i64,
    pub online_users: i64,
    pub active_users: i64,
    pub on_hold_users: i64,
    pub disabled_users: i64,
    pub expired_users: i64,
    pub limited_users: i64,
    pub incoming_bandwidth: i64,
    pub outgoing_bandwidth: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SystemUsersStats {
    pub total_user: i64,
    pub online_users: i64,
    pub active_users: i64,
    pub on_hold_users: i64,
    pub disabled_users: i64,
    pub expired_users: i64,
    pub limited_users: i64,
    pub incoming_bandwidth: i64,
    pub outgoing_bandwidth: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SystemResourceStats {
    pub version: String,
    pub uptime_seconds: i64,
    #[serde(default)]
    pub mem_total: Option<i64>,
    #[serde(default)]
    pub mem_used: Option<i64>,
    #[serde(default)]
    pub disk_total: Option<i64>,
    #[serde(default)]
    pub disk_used: Option<i64>,
    #[serde(default)]
    pub cpu_cores: Option<i64>,
    #[serde(default)]
    pub cpu_usage: Option<f64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct WorkerHealth {
    pub status: String,
    #[serde(default)]
    pub response_time_ms: Option<i64>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct WireGuardSubnetUsage {
    pub subnet: String,
    pub interface_tags: Vec<String>,
    pub capacity: i64,
    pub used: i64,
    pub free: i64,
    pub free_ips: Vec<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct WorkersHealth {
    pub scheduler: WorkerHealth,
    pub node: WorkerHealth,
    #[serde(flatten)]
    pub extra: Extra,
}
