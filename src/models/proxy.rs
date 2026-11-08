use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StrOrInt {
    Int(i64),
    Str(String),
}

impl From<i64> for StrOrInt {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<String> for StrOrInt {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<&str> for StrOrInt {
    fn from(value: &str) -> Self {
        Self::Str(value.to_owned())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StrOrIntList {
    Str(String),
    List(Vec<i64>),
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VMessSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VlessSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TrojanSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ShadowsocksSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WireGuardSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_ips: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HysteriaSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProxyTable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vmess: Option<VMessSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vless: Option<VlessSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trojan: Option<TrojanSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadowsocks: Option<ShadowsocksSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wireguard: Option<WireGuardSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hysteria: Option<HysteriaSettings>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Brutal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    pub up_mbps: i64,
    pub down_mbps: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClashMuxSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<MultiplexProtocol>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_streams: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_streams: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brutal: Option<Brutal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_tcp: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ExtraSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<ShadowsocksMethods>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FragmentSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xray: Option<XrayFragmentSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sing_box: Option<SingBoxFragmentSettings>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GRPCSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permit_without_stream: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_windows_size: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct KCPSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tti: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downlink_capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub congestion: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_buffer_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_buffer_size: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuxSettingsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sing_box: Option<SingBoxMuxSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clash: Option<ClashMuxSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xray: Option<XrayMuxSettingsInput>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuxSettingsOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sing_box: Option<SingBoxMuxSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clash: Option<ClashMuxSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xray: Option<XrayMuxSettingsOutput>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NoiseSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xray: Option<Vec<XrayNoiseSettings>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SingBoxFragmentSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment_fallback_delay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_fragment: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SingBoxMuxSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<MultiplexProtocol>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_streams: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_streams: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brutal: Option<Brutal>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TcpSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<HTTPRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<HTTPResponse>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TransportSettingsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xhttp_settings: Option<XHttpSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc_settings: Option<GRPCSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcp_settings: Option<KCPSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_settings: Option<TcpSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub websocket_settings: Option<WebSocketSettings>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TransportSettingsOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xhttp_settings: Option<XHttpSettingsOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc_settings: Option<GRPCSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcp_settings: Option<KCPSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_settings: Option<TcpSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub websocket_settings: Option<WebSocketSettings>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WebSocketSettings {
    #[serde(
        default,
        rename = "heartbeatPeriod",
        alias = "heartbeat_period",
        skip_serializing_if = "Option::is_none"
    )]
    pub heartbeat_period: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XHttpSettingsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<XHttpModes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_grpc_header: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_bytes: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_obfs_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_http_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id_table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id_length: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_data_placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_data_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_chunk_size: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sc_max_each_post_bytes: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sc_min_posts_interval_ms: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xmux: Option<XMuxSettingsInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_settings: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XHttpSettingsOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<XHttpModes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_grpc_header: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_bytes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_obfs_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_padding_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_http_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id_table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id_length: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_data_placement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_data_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplink_chunk_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sc_max_each_post_bytes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sc_min_posts_interval_ms: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xmux: Option<XMuxSettingsOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_settings: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XMuxSettingsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c_max_reuse_times: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h_max_reusable_secs: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h_max_request_times: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h_keep_alive_period: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XMuxSettingsOutput {
    #[serde(
        default,
        rename = "maxConcurrency",
        alias = "max_concurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrency: Option<String>,
    #[serde(
        default,
        rename = "maxConnections",
        alias = "max_connections",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_connections: Option<String>,
    #[serde(
        default,
        rename = "cMaxReuseTimes",
        alias = "c_max_reuse_times",
        skip_serializing_if = "Option::is_none"
    )]
    pub c_max_reuse_times: Option<String>,
    #[serde(
        default,
        rename = "hMaxReusableSecs",
        alias = "h_max_reusable_secs",
        skip_serializing_if = "Option::is_none"
    )]
    pub h_max_reusable_secs: Option<String>,
    #[serde(
        default,
        rename = "hMaxRequestTimes",
        alias = "h_max_request_times",
        skip_serializing_if = "Option::is_none"
    )]
    pub h_max_request_times: Option<String>,
    #[serde(
        default,
        rename = "hKeepAlivePeriod",
        alias = "h_keep_alive_period",
        skip_serializing_if = "Option::is_none"
    )]
    pub h_keep_alive_period: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XrayFragmentSettings {
    pub packets: String,
    pub length: String,
    pub interval: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XrayMuxSettingsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xudp_concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xudp_proxy_udp_443: Option<XUDP>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XrayMuxSettingsOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    #[serde(
        default,
        rename = "xudpConcurrency",
        alias = "xudp_concurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub xudp_concurrency: Option<i64>,
    #[serde(
        default,
        rename = "xudpProxyUDP443",
        alias = "xudp_proxy_u_d_p443",
        skip_serializing_if = "Option::is_none"
    )]
    pub xudp_proxy_u_d_p443: Option<XUDP>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XrayNoiseSettings {
    #[serde(rename = "type")]
    pub noise_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packet: Option<StrOrIntList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<StrOrInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rand: Option<StrOrInt>,
    #[serde(
        default,
        rename = "randRange",
        alias = "rand_range",
        skip_serializing_if = "Option::is_none"
    )]
    pub rand_range: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}
pub type ProxySettings = ProxyTable;

pub type VmessSettings = VMessSettings;
