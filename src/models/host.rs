use super::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseHost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub remark: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbound_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        default = "default_host_security",
        skip_serializing_if = "Option::is_none"
    )]
    pub security: Option<ProxyHostSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<ProxyHostALPN>>,
    #[serde(
        default = "default_host_fingerprint",
        skip_serializing_if = "Option::is_none"
    )]
    pub fingerprint: Option<ProxyHostFingerprint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowinsecure: Option<bool>,
    #[serde(
        default = "default_some_false",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_settings: Option<TransportSettingsOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mux_settings: Option<MuxSettingsOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment_settings: Option<FragmentSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_settings: Option<NoiseSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_mask_settings: Option<Map<String, Value>>,
    #[serde(
        default = "default_some_false",
        skip_serializing_if = "Option::is_none"
    )]
    pub random_user_agent: Option<bool>,
    #[serde(
        default = "default_some_false",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_sni_as_host: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vless_route: Option<String>,
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<UserStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ech_config_list: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ech_query_strategy: Option<ECHQueryStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_peer_cert_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify_peer_cert_by_name: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wireguard_overrides: Option<WireGuardHostOverrides>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_templates: Option<SubscriptionTemplates>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_host_security() -> Option<ProxyHostSecurity> {
    Some(ProxyHostSecurity::InboundDefault)
}

fn default_host_fingerprint() -> Option<ProxyHostFingerprint> {
    Some(ProxyHostFingerprint::Empty)
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkHostSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BulkHostsActionResponse {
    pub hosts: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateHost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub remark: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbound_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<ProxyHostSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<ProxyHostALPN>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<ProxyHostFingerprint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowinsecure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_settings: Option<TransportSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mux_settings: Option<MuxSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment_settings: Option<FragmentSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_settings: Option<NoiseSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_mask_settings: Option<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_user_agent: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_sni_as_host: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vless_route: Option<String>,
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<UserStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ech_config_list: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ech_query_strategy: Option<ECHQueryStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_peer_cert_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify_peer_cert_by_name: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wireguard_overrides: Option<WireGuardHostOverrides>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_templates: Option<SubscriptionTemplates>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HostNotificationEnable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_hosts: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RemoveHostsResponse {
    pub hosts: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WireGuardHostOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keepalive_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Extra,
}

pub type HostsModel = Vec<CreateHost>;

pub type HostBase = CreateHost;

pub type HostResponse = BaseHost;
