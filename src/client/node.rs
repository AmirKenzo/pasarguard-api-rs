use super::*;
use std::collections::HashMap;

pub use crate::models::NodeRealtimeStats;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetNodesParams {
    pub core_id: Option<i64>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub ids: Option<Vec<i64>>,
    pub status: Option<Vec<NodeStatus>>,
    pub enabled: Option<bool>,
    pub search: Option<String>,
}

impl Default for GetNodesParams {
    fn default() -> Self {
        Self {
            core_id: None,
            offset: None,
            limit: None,
            ids: None,
            status: None,
            enabled: Some(false),
            search: None,
        }
    }
}

impl PanelClient {
    async fn call_any_with_query(
        &self,
        method: Method,
        path: &str,
        query: Query,
    ) -> PanelResult<AnyPayload> {
        let raw = self.send(method, path, query, Body::None, true).await?;
        Ok(AnyPayload::from_body(&raw.body))
    }

    async fn bulk_node_action(
        &self,
        path: &str,
        bulk: &BulkNodeSelection,
    ) -> PanelResult<BulkNodesActionResponse> {
        self.call(Method::POST, path, Query::new(), json_body(bulk)?)
            .await
    }

    /// Get Node Settings.
    pub async fn get_node_settings(&self) -> PanelResult<NodeSettings> {
        self.call(Method::GET, "/api/node/settings", Query::new(), Body::None)
            .await
    }

    /// Retrieve usage statistics for nodes within a specified date range.
    pub async fn get_usage(&self, params: &UsageParams) -> PanelResult<NodeUsageStatsList> {
        self.call(
            Method::GET,
            "/api/node/usage",
            usage_query(params),
            Body::None,
        )
        .await
    }

    /// Get user count metric statistics (online, expired or limited users) over time.
    pub async fn get_user_count_metric(
        &self,
        metric: UserCountMetric,
        params: &UsageParams,
    ) -> PanelResult<UserCountMetricStatsList> {
        self.call(
            Method::GET,
            &format!("/api/node/user_counts/{metric}"),
            usage_query(params),
            Body::None,
        )
        .await
    }

    /// Retrieve a list of all nodes.
    pub async fn get_nodes(&self, params: &GetNodesParams) -> PanelResult<NodesResponse> {
        let p = params;
        let query = Query::new()
            .opt("core_id", p.core_id)
            .opt("offset", p.offset)
            .opt("limit", p.limit)
            .list("ids", p.ids.as_deref())
            .list("status", p.status.as_deref())
            .opt("enabled", p.enabled)
            .opt("search", p.search.as_deref());
        self.call(Method::GET, "/api/nodes", query, Body::None)
            .await
    }

    /// Get a lightweight node list with only id, name and status.
    pub async fn get_nodes_simple(
        &self,
        params: &SimpleListParams,
    ) -> PanelResult<NodesSimpleResponse> {
        self.call(
            Method::GET,
            "/api/nodes/simple",
            simple_list_query(params),
            Body::None,
        )
        .await
    }

    /// Trigger a reconnection for all nodes, optionally limited to one core.
    pub async fn reconnect_all_node(&self, core_id: Option<i64>) -> PanelResult<AnyPayload> {
        self.call_any_with_query(
            Method::POST,
            "/api/nodes/reconnect",
            Query::new().opt("core_id", core_id),
        )
        .await
    }

    /// Create a new node in the database.
    pub async fn create_node(&self, node: &NodeCreate) -> PanelResult<NodeResponse> {
        self.call(Method::POST, "/api/node", Query::new(), json_body(node)?)
            .await
    }

    /// Retrieve details of a specific node by its ID.
    pub async fn get_node(&self, node_id: i64) -> PanelResult<NodeResponse> {
        self.call(
            Method::GET,
            &format!("/api/node/{node_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify a node's details.
    pub async fn modify_node(&self, node_id: i64, node: &NodeModify) -> PanelResult<NodeResponse> {
        self.call(
            Method::PUT,
            &format!("/api/node/{node_id}"),
            Query::new(),
            json_body(node)?,
        )
        .await
    }

    /// Remove a node.
    pub async fn remove_node(&self, node_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/node/{node_id}"),
            Query::new(),
        )
        .await
    }

    /// Update the node binary.
    pub async fn update_node(&self, node_id: i64) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/node/{node_id}/update"),
            Body::None,
        )
        .await
    }

    /// Update the core version running on a node.
    pub async fn update_core(
        &self,
        node_id: i64,
        node_core_update: &NodeCoreUpdate,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/node/{node_id}/core_update"),
            json_body(node_core_update)?,
        )
        .await
    }

    /// Update the geo files of a node.
    pub async fn update_geofiles(
        &self,
        node_id: i64,
        node_geo_files_update: &NodeGeoFilesUpdate,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/node/{node_id}/geofiles"),
            json_body(node_geo_files_update)?,
        )
        .await
    }

    /// Reset a node's traffic usage.
    pub async fn reset_node_usage(&self, node_id: i64) -> PanelResult<NodeResponse> {
        self.call(
            Method::POST,
            &format!("/api/node/{node_id}/reset"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Trigger a reconnection for the specified node.
    pub async fn reconnect_node(&self, node_id: i64) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/node/{node_id}/reconnect"),
            Body::None,
        )
        .await
    }

    /// Sync a node's users and configuration (`flush_users` defaults to true in Python).
    pub async fn sync_node(&self, node_id: i64, flush_users: bool) -> PanelResult<AnyPayload> {
        self.call_any_with_query(
            Method::PUT,
            &format!("/api/node/{node_id}/sync"),
            Query::new().set("flush_users", flush_users),
        )
        .await
    }

    /// Retrieve the logs of a node.
    pub async fn node_logs(&self, node_id: i64) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::GET,
            &format!("/api/node/{node_id}/logs"),
            Body::None,
        )
        .await
    }

    /// Get periodic statistics of a node.
    pub async fn get_node_stats_periodic(
        &self,
        node_id: i64,
        params: &SubUsageParams,
    ) -> PanelResult<NodeStatsList> {
        let query = Query::new()
            .opt("period", params.period)
            .opt("start", params.start.as_ref())
            .opt("end", params.end.as_ref());
        self.call(
            Method::GET,
            &format!("/api/node/{node_id}/stats"),
            query,
            Body::None,
        )
        .await
    }

    /// Retrieve real-time statistics of a node.
    pub async fn realtime_node_stats(&self, node_id: i64) -> PanelResult<NodeRealtimeStats> {
        self.call(
            Method::GET,
            &format!("/api/node/{node_id}/realtime_stats"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Measure the outbound latencies of a node (`name` defaults to "" in Python).
    pub async fn node_outbounds_latency(
        &self,
        node_id: i64,
        name: &str,
        timeout: Option<i64>,
    ) -> PanelResult<NodeOutboundsLatencyResponse> {
        self.call(
            Method::GET,
            &format!("/api/node/{node_id}/outbounds_latency"),
            Query::new().set("name", name).opt("timeout", timeout),
            Body::None,
        )
        .await
    }

    /// Retrieve real-time statistics of all nodes.
    pub async fn realtime_nodes_stats(
        &self,
    ) -> PanelResult<HashMap<String, Option<NodeRealtimeStats>>> {
        self.call(
            Method::GET,
            "/api/nodes/realtime_stats",
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Retrieve a user's online IPs on every node.
    pub async fn user_online_ip_list_all_nodes(&self, user_id: i64) -> PanelResult<UserIPListAll> {
        self.call(
            Method::GET,
            &format!("/api/node/online_stats/{user_id}/ip"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Retrieve a user's online statistics on a node.
    pub async fn user_online_stats(
        &self,
        node_id: i64,
        user_id: i64,
    ) -> PanelResult<HashMap<String, i64>> {
        self.call(
            Method::GET,
            &format!("/api/node/{node_id}/online_stats/{user_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Retrieve a user's online IPs on a node.
    pub async fn user_online_ip_list(&self, node_id: i64, user_id: i64) -> PanelResult<UserIPList> {
        self.call(
            Method::GET,
            &format!("/api/node/{node_id}/online_stats/{user_id}/ip"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Clear rows of a usage table within an optional date range.
    pub async fn clear_usage_data(
        &self,
        table: UsageTable,
        start: Option<PanelDateTime>,
        end: Option<PanelDateTime>,
    ) -> PanelResult<AnyPayload> {
        self.call_any_with_query(
            Method::DELETE,
            &format!("/api/nodes/clear_usage_data/{table}"),
            Query::new().opt("start", start).opt("end", end),
        )
        .await
    }

    /// Delete the selected nodes.
    pub async fn bulk_delete_nodes(
        &self,
        bulk: &BulkNodeSelection,
    ) -> PanelResult<RemoveNodesResponse> {
        self.call(
            Method::POST,
            "/api/nodes/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Disable the selected nodes.
    pub async fn bulk_disable_nodes(
        &self,
        bulk: &BulkNodeSelection,
    ) -> PanelResult<BulkNodesActionResponse> {
        self.bulk_node_action("/api/nodes/bulk/disable", bulk).await
    }

    /// Enable the selected nodes.
    pub async fn bulk_enable_nodes(
        &self,
        bulk: &BulkNodeSelection,
    ) -> PanelResult<BulkNodesActionResponse> {
        self.bulk_node_action("/api/nodes/bulk/enable", bulk).await
    }

    /// Reset the traffic usage of the selected nodes.
    pub async fn bulk_reset_nodes_usage(
        &self,
        bulk: &BulkNodeSelection,
    ) -> PanelResult<BulkNodesActionResponse> {
        self.bulk_node_action("/api/nodes/bulk/reset", bulk).await
    }

    /// Reconnect the selected nodes.
    pub async fn bulk_reconnect_nodes(
        &self,
        bulk: &BulkNodeSelection,
    ) -> PanelResult<BulkNodesActionResponse> {
        self.bulk_node_action("/api/nodes/bulk/reconnect", bulk)
            .await
    }

    /// Update the node binary of the selected nodes.
    pub async fn bulk_update_nodes(
        &self,
        bulk: &BulkNodeSelection,
    ) -> PanelResult<BulkNodesActionResponse> {
        self.bulk_node_action("/api/nodes/bulk/update", bulk).await
    }
}
