use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetAllCoresParams {
    pub ids: Option<Vec<i64>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl PanelClient {
    /// Create a new core configuration.
    pub async fn create_core_config(&self, core: &CoreCreate) -> PanelResult<CoreResponse> {
        self.call(Method::POST, "/api/core", Query::new(), json_body(core)?)
            .await
    }

    /// Get a core configuration by its ID.
    pub async fn get_core_config(&self, core_id: i64) -> PanelResult<CoreResponse> {
        self.call(
            Method::GET,
            &format!("/api/core/{core_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Update an existing core configuration.
    pub async fn modify_core_config(
        &self,
        core_id: i64,
        core: &CoreCreate,
        restart_nodes: bool,
    ) -> PanelResult<CoreResponse> {
        self.call(
            Method::PUT,
            &format!("/api/core/{core_id}"),
            Query::new().set("restart_nodes", restart_nodes),
            json_body(core)?,
        )
        .await
    }

    /// Delete a core configuration (`restart_nodes` defaults to false in Python).
    pub async fn delete_core_config(&self, core_id: i64, restart_nodes: bool) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/core/{core_id}"),
            Query::new().set("restart_nodes", restart_nodes),
        )
        .await
    }

    /// Get a list of all core configurations.
    pub async fn get_all_cores(&self, params: &GetAllCoresParams) -> PanelResult<CoreResponseList> {
        let query = Query::new()
            .list("ids", params.ids.as_deref())
            .opt("offset", params.offset)
            .opt("limit", params.limit);
        self.call(Method::GET, "/api/cores", query, Body::None)
            .await
    }

    /// Get a lightweight core list with only id and name.
    pub async fn get_cores_simple(
        &self,
        params: &SimpleListParams,
    ) -> PanelResult<CoresSimpleResponse> {
        self.call(
            Method::GET,
            "/api/cores/simple",
            simple_list_query(params),
            Body::None,
        )
        .await
    }

    /// Restart the nodes that use a core configuration.
    pub async fn restart_core(&self, core_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::POST,
            &format!("/api/core/{core_id}/restart"),
            Query::new(),
        )
        .await
    }

    /// Delete the selected core configurations.
    pub async fn bulk_delete_cores(
        &self,
        bulk: &BulkCoreSelection,
    ) -> PanelResult<RemoveCoresResponse> {
        self.call(
            Method::POST,
            "/api/cores/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Scan a target for REALITY feasibility.
    pub async fn scan_reality_target(
        &self,
        scan: &RealityScanRequest,
    ) -> PanelResult<RealityScanResult> {
        self.call(
            Method::POST,
            "/api/core/reality-scan",
            Query::new(),
            json_body(scan)?,
        )
        .await
    }
}
