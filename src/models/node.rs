use super::*;
use crate::datetime::PanelDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkNodeSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BulkNodesActionResponse {
    pub nodes: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct NodeCoreUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NodeCreate {
    pub name: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_coefficient: Option<f64>,
    pub connection_type: NodeConnectionType,
    pub server_ca: String,
    pub keep_alive: i64,
    pub core_config_id: i64,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl NodeCreate {
    pub fn new(
        name: impl Into<String>,
        address: impl Into<String>,
        connection_type: NodeConnectionType,
        server_ca: impl Into<String>,
        keep_alive: i64,
        core_config_id: i64,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            address: address.into(),
            port: None,
            api_port: None,
            usage_coefficient: None,
            connection_type,
            server_ca: server_ca.into(),
            keep_alive,
            core_config_id,
            api_key: api_key.into(),
            data_limit: None,
            data_limit_reset_strategy: None,
            reset_time: None,
            default_timeout: None,
            internal_timeout: None,
            proxy_url: None,
            extra: Extra::default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct NodeGeoFilesUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<GeoFilseRegion>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct NodeModify {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<NodeConnectionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ca: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_config_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<NodeStatus>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeNotificationEnable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovered: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limited: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_usage: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeOutboundLatency {
    pub name: String,
    pub alive: bool,
    pub delay: i64,
    pub link: String,
    pub last_seen_time: i64,
    pub last_try_time: i64,
    pub source: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeOutboundsLatencyResponse {
    pub latencies: Vec<NodeOutboundLatency>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeRealtimeStats {
    pub mem_total: i64,
    pub mem_used: i64,
    pub cpu_cores: i64,
    pub cpu_usage: f64,
    pub incoming_bandwidth_speed: i64,
    pub outgoing_bandwidth_speed: i64,
    pub uptime: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeResponse {
    pub name: String,
    pub address: String,
    #[serde(default = "default_port")]
    pub port: Option<i64>,
    #[serde(default = "default_api_port")]
    pub api_port: Option<i64>,
    #[serde(default = "default_usage_coefficient")]
    pub usage_coefficient: Option<f64>,
    pub connection_type: NodeConnectionType,
    pub server_ca: String,
    pub keep_alive: i64,
    #[serde(default)]
    pub core_config_id: Option<i64>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_some_zero")]
    pub data_limit: Option<i64>,
    #[serde(default = "default_node_reset_strategy")]
    pub data_limit_reset_strategy: Option<DataLimitResetStrategy>,
    #[serde(default = "default_reset_time")]
    pub reset_time: Option<i64>,
    #[serde(default = "default_timeout")]
    pub default_timeout: Option<i64>,
    #[serde(default = "default_internal_timeout")]
    pub internal_timeout: Option<i64>,
    #[serde(default)]
    pub proxy_url: Option<String>,
    pub id: i64,
    #[serde(default)]
    pub xray_version: Option<String>,
    #[serde(default)]
    pub node_version: Option<String>,
    pub status: NodeStatus,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default = "default_some_zero")]
    pub uplink: Option<i64>,
    #[serde(default = "default_some_zero")]
    pub downlink: Option<i64>,
    #[serde(default)]
    pub lifetime_uplink: Option<i64>,
    #[serde(default)]
    pub lifetime_downlink: Option<i64>,
    #[serde(default)]
    pub core_version: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_port() -> Option<i64> {
    Some(62050)
}

fn default_api_port() -> Option<i64> {
    Some(62051)
}

fn default_usage_coefficient() -> Option<f64> {
    Some(1.0)
}

fn default_node_reset_strategy() -> Option<DataLimitResetStrategy> {
    Some(DataLimitResetStrategy::NoReset)
}

fn default_reset_time() -> Option<i64> {
    Some(-1)
}

fn default_timeout() -> Option<i64> {
    Some(10)
}

fn default_internal_timeout() -> Option<i64> {
    Some(15)
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodesResponse {
    pub nodes: Vec<NodeResponse>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeSimple {
    pub id: i64,
    pub name: String,
    pub status: NodeStatus,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodesSimpleResponse {
    pub nodes: Vec<NodeSimple>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

pub type NodeResponseList = NodesResponse;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeSettings {
    #[serde(default = "default_min_node_version")]
    pub min_node_version: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_min_node_version() -> Option<String> {
    Some("v1.0.0".to_owned())
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeStats {
    pub period_start: PanelDateTime,
    pub mem_usage_percentage: f64,
    pub cpu_usage_percentage: f64,
    pub incoming_bandwidth_speed: f64,
    pub outgoing_bandwidth_speed: f64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeStatsList {
    #[serde(default)]
    pub period: Option<Period>,
    pub start: PanelDateTime,
    pub end: PanelDateTime,
    pub stats: Vec<NodeStats>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeUsageStat {
    pub uplink: i64,
    pub downlink: i64,
    pub period_start: PanelDateTime,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NodeUsageStatsList {
    #[serde(default)]
    pub period: Option<Period>,
    pub start: PanelDateTime,
    pub end: PanelDateTime,
    pub stats: HashMap<String, Vec<NodeUsageStat>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RemoveNodesResponse {
    pub nodes: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserIPList {
    pub ips: HashMap<String, i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserIPListAll {
    pub nodes: HashMap<String, Option<UserIPList>>,
    #[serde(flatten)]
    pub extra: Extra,
}
