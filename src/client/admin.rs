use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetAdminsParams {
    pub ids: Option<Vec<i64>>,
    pub usernames: Option<Vec<String>>,
    pub username: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetAdminsSimpleParams {
    pub ids: Option<Vec<i64>>,
    pub usernames: Option<Vec<String>>,
    pub search: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
    pub all: Option<bool>,
}

impl Default for GetAdminsSimpleParams {
    fn default() -> Self {
        Self {
            ids: None,
            usernames: None,
            search: None,
            offset: None,
            limit: None,
            sort: None,
            all: Some(false),
        }
    }
}

impl PanelClient {
    /// Exchange a Telegram mini app init data for an admin token.
    pub async fn admin_mini_app_token(
        &self,
        x_telegram_authorization: &str,
    ) -> PanelResult<AnyPayload> {
        let (_, raw) = self
            .send_with_headers(
                Method::POST,
                "/api/admin/miniapp/token",
                Query::new(),
                Body::None,
                false,
                &[("x-telegram-authorization", x_telegram_authorization)],
            )
            .await?;
        Ok(AnyPayload::from_body(&raw.body))
    }

    /// Get the current authenticated admin.
    pub async fn get_current_admin(&self) -> PanelResult<AdminDetails> {
        self.call(Method::GET, "/api/admin", Query::new(), Body::None)
            .await
    }

    /// Create a new admin.
    pub async fn create_admin(&self, admin: &AdminCreate) -> PanelResult<AdminDetails> {
        self.call(Method::POST, "/api/admin", Query::new(), json_body(admin)?)
            .await
    }

    /// Modify an existing admin by username.
    pub async fn modify_admin(
        &self,
        username: &str,
        admin: &AdminModify,
    ) -> PanelResult<AdminDetails> {
        self.call(
            Method::PUT,
            &format!("/api/admin/{username}"),
            Query::new(),
            json_body(admin)?,
        )
        .await
    }

    /// Remove an admin by username.
    pub async fn remove_admin(&self, username: &str) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/admin/{username}"),
            Query::new(),
        )
        .await
    }

    /// Get usage statistics of an admin's users by username.
    pub async fn get_admin_usage(
        &self,
        username: &str,
        params: &UsageParams,
    ) -> PanelResult<UserUsageStatsList> {
        self.call(
            Method::GET,
            &format!("/api/admin/{username}/usage"),
            usage_query(params),
            Body::None,
        )
        .await
    }

    /// Disable all active users of an admin by username.
    pub async fn disable_all_active_users(&self, username: &str) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/admin/{username}/users/disable"),
            Body::None,
        )
        .await
    }

    /// Activate all disabled users of an admin by username.
    pub async fn activate_all_disabled_users(&self, username: &str) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/admin/{username}/users/activate"),
            Body::None,
        )
        .await
    }

    /// Remove all users of an admin by username.
    pub async fn remove_all_users(&self, username: &str) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::DELETE,
            &format!("/api/admin/{username}/users"),
            Body::None,
        )
        .await
    }

    /// Reset the usage of an admin by username.
    pub async fn reset_admin_usage(&self, username: &str) -> PanelResult<AdminDetails> {
        self.call(
            Method::POST,
            &format!("/api/admin/{username}/reset"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify an existing admin by username.
    pub async fn modify_admin_by_username(
        &self,
        username: &str,
        admin: &AdminModify,
    ) -> PanelResult<AdminDetails> {
        self.call(
            Method::PUT,
            &format!("/api/admin/by-username/{username}"),
            Query::new(),
            json_body(admin)?,
        )
        .await
    }

    /// Remove an admin by username.
    pub async fn remove_admin_by_username(&self, username: &str) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/admin/by-username/{username}"),
            Query::new(),
        )
        .await
    }

    /// Get usage statistics of an admin's users by username.
    pub async fn get_admin_usage_by_username(
        &self,
        username: &str,
        params: &UsageParams,
    ) -> PanelResult<UserUsageStatsList> {
        self.call(
            Method::GET,
            &format!("/api/admin/by-username/{username}/usage"),
            usage_query(params),
            Body::None,
        )
        .await
    }

    /// Disable all active users of an admin by username.
    pub async fn disable_all_active_users_by_username(
        &self,
        username: &str,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/admin/by-username/{username}/users/disable"),
            Body::None,
        )
        .await
    }

    /// Activate all disabled users of an admin by username.
    pub async fn activate_all_disabled_users_by_username(
        &self,
        username: &str,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/admin/by-username/{username}/users/activate"),
            Body::None,
        )
        .await
    }

    /// Remove all users of an admin by username.
    pub async fn remove_all_users_by_username(&self, username: &str) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::DELETE,
            &format!("/api/admin/by-username/{username}/users"),
            Body::None,
        )
        .await
    }

    /// Reset the usage of an admin by username.
    pub async fn reset_admin_usage_by_username(&self, username: &str) -> PanelResult<AdminDetails> {
        self.call(
            Method::POST,
            &format!("/api/admin/by-username/{username}/reset"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify an existing admin by id.
    pub async fn modify_admin_by_id(
        &self,
        admin_id: i64,
        admin: &AdminModify,
    ) -> PanelResult<AdminDetails> {
        self.call(
            Method::PUT,
            &format!("/api/admin/by-id/{admin_id}"),
            Query::new(),
            json_body(admin)?,
        )
        .await
    }

    /// Remove an admin by id.
    pub async fn remove_admin_by_id(&self, admin_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/admin/by-id/{admin_id}"),
            Query::new(),
        )
        .await
    }

    /// Get usage statistics of an admin's users by id.
    pub async fn get_admin_usage_by_id(
        &self,
        admin_id: i64,
        params: &UsageParams,
    ) -> PanelResult<UserUsageStatsList> {
        self.call(
            Method::GET,
            &format!("/api/admin/by-id/{admin_id}/usage"),
            usage_query(params),
            Body::None,
        )
        .await
    }

    /// Disable all active users of an admin by id.
    pub async fn disable_all_active_users_by_id(&self, admin_id: i64) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/admin/by-id/{admin_id}/users/disable"),
            Body::None,
        )
        .await
    }

    /// Activate all disabled users of an admin by id.
    pub async fn activate_all_disabled_users_by_id(
        &self,
        admin_id: i64,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/admin/by-id/{admin_id}/users/activate"),
            Body::None,
        )
        .await
    }

    /// Remove all users of an admin by id.
    pub async fn remove_all_users_by_id(&self, admin_id: i64) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::DELETE,
            &format!("/api/admin/by-id/{admin_id}/users"),
            Body::None,
        )
        .await
    }

    /// Reset the usage of an admin by id.
    pub async fn reset_admin_usage_by_id(&self, admin_id: i64) -> PanelResult<AdminDetails> {
        self.call(
            Method::POST,
            &format!("/api/admin/by-id/{admin_id}/reset"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// List admins with optional filters and pagination.
    pub async fn get_admins(&self, params: &GetAdminsParams) -> PanelResult<AdminsResponse> {
        let p = params;
        let query = Query::new()
            .list("ids", p.ids.as_deref())
            .list("usernames", p.usernames.as_deref())
            .opt("username", p.username.as_deref())
            .opt("offset", p.offset)
            .opt("limit", p.limit)
            .opt("sort", p.sort.as_deref());
        self.call(Method::GET, "/api/admins", query, Body::None)
            .await
    }

    /// List admins with only their id and username.
    pub async fn get_admins_simple(
        &self,
        params: &GetAdminsSimpleParams,
    ) -> PanelResult<AdminsSimpleResponse> {
        let p = params;
        let query = Query::new()
            .list("ids", p.ids.as_deref())
            .list("usernames", p.usernames.as_deref())
            .opt("search", p.search.as_deref())
            .opt("offset", p.offset)
            .opt("limit", p.limit)
            .opt("sort", p.sort.as_deref())
            .opt("all", p.all);
        self.call(Method::GET, "/api/admins/simple", query, Body::None)
            .await
    }

    /// Delete the selected admins.
    pub async fn bulk_delete_admins(
        &self,
        bulk: &BulkAdminSelection,
    ) -> PanelResult<RemoveAdminsResponse> {
        self.call(
            Method::POST,
            "/api/admins/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Reset the usage of the selected admins.
    pub async fn bulk_reset_admins_usage(
        &self,
        bulk: &BulkAdminSelection,
    ) -> PanelResult<BulkAdminsActionResponse> {
        self.call(
            Method::POST,
            "/api/admins/bulk/reset",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Disable the selected admins.
    pub async fn bulk_disable_admins(
        &self,
        bulk: &BulkAdminSelection,
    ) -> PanelResult<BulkAdminsActionResponse> {
        self.call(
            Method::POST,
            "/api/admins/bulk/disable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Enable the selected admins.
    pub async fn bulk_enable_admins(
        &self,
        bulk: &BulkAdminSelection,
    ) -> PanelResult<BulkAdminsActionResponse> {
        self.call(
            Method::POST,
            "/api/admins/bulk/enable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Disable all active users of the selected admins.
    pub async fn bulk_disable_all_active_users(
        &self,
        bulk: &BulkAdminSelection,
    ) -> PanelResult<BulkAdminsActionResponse> {
        self.call(
            Method::POST,
            "/api/admins/bulk/users/disable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Activate all disabled users of the selected admins.
    pub async fn bulk_activate_all_disabled_users(
        &self,
        bulk: &BulkAdminSelection,
    ) -> PanelResult<BulkAdminsActionResponse> {
        self.call(
            Method::POST,
            "/api/admins/bulk/users/activate",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Remove all users of the selected admins.
    pub async fn bulk_remove_all_users(
        &self,
        bulk: &BulkAdminSelection,
    ) -> PanelResult<BulkAdminsActionResponse> {
        self.call(
            Method::DELETE,
            "/api/admins/bulk/users",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }
}
