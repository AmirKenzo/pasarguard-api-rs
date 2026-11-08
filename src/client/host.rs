use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetHostsParams {
    pub ids: Option<Vec<i64>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Default for GetHostsParams {
    fn default() -> Self {
        Self {
            ids: None,
            offset: Some(0),
            limit: Some(0),
        }
    }
}

impl PanelClient {
    async fn bulk_host_action(
        &self,
        path: &str,
        bulk: &BulkHostSelection,
    ) -> PanelResult<BulkHostsActionResponse> {
        self.call(Method::POST, path, Query::new(), json_body(bulk)?)
            .await
    }

    /// Get a host by its ID.
    pub async fn get_host(&self, host_id: i64) -> PanelResult<BaseHost> {
        self.call(
            Method::GET,
            &format!("/api/host/{host_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify a host by its ID.
    pub async fn modify_host(&self, host_id: i64, host: &CreateHost) -> PanelResult<BaseHost> {
        self.call(
            Method::PUT,
            &format!("/api/host/{host_id}"),
            Query::new(),
            json_body(host)?,
        )
        .await
    }

    /// Remove a host by its ID.
    pub async fn remove_host(&self, host_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/host/{host_id}"),
            Query::new(),
        )
        .await
    }

    /// Get a list of hosts.
    pub async fn get_hosts(&self, params: &GetHostsParams) -> PanelResult<Vec<BaseHost>> {
        let query = Query::new()
            .list("ids", params.ids.as_deref())
            .opt("offset", params.offset)
            .opt("limit", params.limit);
        self.call(Method::GET, "/api/hosts", query, Body::None)
            .await
    }

    /// Replace the hosts with the given list.
    pub async fn modify_hosts(&self, hosts: &[CreateHost]) -> PanelResult<Vec<BaseHost>> {
        self.call(Method::PUT, "/api/hosts", Query::new(), json_body(&hosts)?)
            .await
    }

    /// Create a new host.
    pub async fn create_host(&self, host: &CreateHost) -> PanelResult<BaseHost> {
        self.call(Method::POST, "/api/host/", Query::new(), json_body(host)?)
            .await
    }

    /// Delete the selected hosts.
    pub async fn bulk_delete_hosts(
        &self,
        bulk: &BulkHostSelection,
    ) -> PanelResult<RemoveHostsResponse> {
        self.call(
            Method::POST,
            "/api/hosts/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Disable the selected hosts.
    pub async fn bulk_disable_hosts(
        &self,
        bulk: &BulkHostSelection,
    ) -> PanelResult<BulkHostsActionResponse> {
        self.bulk_host_action("/api/hosts/bulk/disable", bulk).await
    }

    /// Enable the selected hosts.
    pub async fn bulk_enable_hosts(
        &self,
        bulk: &BulkHostSelection,
    ) -> PanelResult<BulkHostsActionResponse> {
        self.bulk_host_action("/api/hosts/bulk/enable", bulk).await
    }
}
