use super::*;
use crate::datetime::PanelDateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkCoreSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CoreCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub config: Map<String, Value>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub core_type: Option<CoreType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_inbound_tags: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallbacks_inbound_tags: Option<Vec<Value>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreResponse {
    pub name: String,
    pub config: Map<String, Value>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub core_type: Option<CoreType>,
    pub exclude_inbound_tags: Vec<String>,
    pub fallbacks_inbound_tags: Vec<String>,
    pub id: i64,
    pub created_at: PanelDateTime,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CoreResponseList {
    pub count: i64,
    #[serde(default = "default_core_list")]
    pub cores: Option<Vec<CoreResponse>>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_core_list() -> Option<Vec<CoreResponse>> {
    Some(Vec::new())
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CoreSimple {
    pub id: i64,
    pub name: String,
    #[serde(default, rename = "type")]
    pub core_type: Option<CoreType>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CoresSimpleResponse {
    pub cores: Vec<CoreSimple>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct RealityScanRequest {
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RealityScanResult {
    pub target: String,
    pub host: String,
    #[serde(default)]
    pub ip: Option<String>,
    pub port: i64,
    #[serde(default)]
    pub sni: Option<String>,
    #[serde(default = "default_some_false")]
    pub sni_discovered: Option<bool>,
    pub feasible: bool,
    pub tls13: bool,
    #[serde(default)]
    pub tls_version: Option<String>,
    pub h2: bool,
    #[serde(default)]
    pub alpn: Option<String>,
    #[serde(default)]
    pub x25519: Option<bool>,
    #[serde(default)]
    pub post_quantum: Option<bool>,
    #[serde(default)]
    pub curve: Option<String>,
    #[serde(default = "default_some_false")]
    pub h3: Option<bool>,
    pub cert_valid: bool,
    #[serde(default)]
    pub cert_subject: Option<String>,
    #[serde(default)]
    pub cert_issuer: Option<String>,
    #[serde(default)]
    pub not_after: Option<String>,
    #[serde(default = "default_server_names")]
    pub server_names: Option<Vec<String>>,
    #[serde(default)]
    pub latency_ms: Option<i64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_server_names() -> Option<Vec<String>> {
    Some(Vec::new())
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RemoveCoresResponse {
    pub cores: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}
