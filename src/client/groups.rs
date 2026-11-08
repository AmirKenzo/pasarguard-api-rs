use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetGroupsParams {
    pub ids: Option<Vec<i64>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl PanelClient {
    /// Create a new group.
    pub async fn create_group(&self, group: &GroupCreate) -> PanelResult<GroupResponse> {
        self.call(Method::POST, "/api/group", Query::new(), json_body(group)?)
            .await
    }

    /// List groups with optional id filter and pagination.
    pub async fn get_all_groups(&self, params: &GetGroupsParams) -> PanelResult<GroupsResponse> {
        let query = Query::new()
            .list("ids", params.ids.as_deref())
            .opt("offset", params.offset)
            .opt("limit", params.limit);
        self.call(Method::GET, "/api/groups", query, Body::None)
            .await
    }

    /// List groups in the lightweight id/name form.
    pub async fn get_groups_simple(
        &self,
        params: &SimpleListParams,
    ) -> PanelResult<GroupsSimpleResponse> {
        self.call(
            Method::GET,
            "/api/groups/simple",
            simple_list_query(params),
            Body::None,
        )
        .await
    }

    /// Get a group by id.
    pub async fn get_group(&self, group_id: i64) -> PanelResult<GroupResponse> {
        self.call(
            Method::GET,
            &format!("/api/group/{group_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify a group by id.
    pub async fn modify_group(
        &self,
        group_id: i64,
        group: &GroupModify,
    ) -> PanelResult<GroupResponse> {
        self.call(
            Method::PUT,
            &format!("/api/group/{group_id}"),
            Query::new(),
            json_body(group)?,
        )
        .await
    }

    /// Remove a group by id.
    pub async fn remove_group(&self, group_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/group/{group_id}"),
            Query::new(),
        )
        .await
    }

    /// Add groups to the selected users in bulk.
    pub async fn bulk_add_groups_to_users(&self, bulk: &BulkGroup) -> PanelResult<AnyPayload> {
        self.call_any(Method::POST, "/api/groups/bulk/add", json_body(bulk)?)
            .await
    }

    /// Remove the selected users from groups in bulk.
    pub async fn bulk_remove_users_from_groups(&self, bulk: &BulkGroup) -> PanelResult<AnyPayload> {
        self.call_any(Method::POST, "/api/groups/bulk/remove", json_body(bulk)?)
            .await
    }

    /// Delete the selected groups in bulk.
    pub async fn bulk_delete_groups(
        &self,
        bulk: &BulkGroupSelection,
    ) -> PanelResult<RemoveGroupsResponse> {
        self.call(
            Method::POST,
            "/api/groups/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Disable the selected groups in bulk.
    pub async fn bulk_disable_groups(
        &self,
        bulk: &BulkGroupSelection,
    ) -> PanelResult<BulkGroupsActionResponse> {
        self.call(
            Method::POST,
            "/api/groups/bulk/disable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Enable the selected groups in bulk.
    pub async fn bulk_enable_groups(
        &self,
        bulk: &BulkGroupSelection,
    ) -> PanelResult<BulkGroupsActionResponse> {
        self.call(
            Method::POST,
            "/api/groups/bulk/enable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }
}
