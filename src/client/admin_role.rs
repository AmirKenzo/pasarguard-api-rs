use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetRolesParams {
    pub search: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
}

impl PanelClient {
    /// Create a new admin role.
    pub async fn create_role(&self, role: &AdminRoleCreate) -> PanelResult<AdminRoleResponse> {
        self.call(
            Method::POST,
            "/api/admin-role",
            Query::new(),
            json_body(role)?,
        )
        .await
    }

    /// Get an admin role by id.
    pub async fn get_role(&self, role_id: i64) -> PanelResult<AdminRoleResponse> {
        self.call(
            Method::GET,
            &format!("/api/admin-role/{role_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify an existing admin role.
    pub async fn modify_role(
        &self,
        role_id: i64,
        role: &AdminRoleModify,
    ) -> PanelResult<AdminRoleResponse> {
        self.call(
            Method::PUT,
            &format!("/api/admin-role/{role_id}"),
            Query::new(),
            json_body(role)?,
        )
        .await
    }

    /// Delete an admin role.
    pub async fn delete_role(&self, role_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/admin-role/{role_id}"),
            Query::new(),
        )
        .await
    }

    /// List admin roles with optional search and pagination.
    pub async fn get_roles(&self, params: &GetRolesParams) -> PanelResult<AdminRolesResponse> {
        let p = params;
        let query = Query::new()
            .opt("search", p.search.as_deref())
            .opt("offset", p.offset)
            .opt("limit", p.limit)
            .opt("sort", p.sort.as_deref());
        self.call(Method::GET, "/api/admin-roles", query, Body::None)
            .await
    }

    /// List admin roles with only their id, name and owner flag.
    pub async fn get_roles_simple(&self) -> PanelResult<AdminRolesSimpleResponse> {
        self.call(
            Method::GET,
            "/api/admin-roles/simple",
            Query::new(),
            Body::None,
        )
        .await
    }
}
