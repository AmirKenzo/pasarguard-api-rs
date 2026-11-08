use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetUsersParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub ids: Option<Vec<i64>>,
    pub username: Option<Vec<String>>,
    pub usernames: Option<Vec<String>>,
    pub admin: Option<Vec<String>>,
    pub admin_ids: Option<Vec<i64>>,
    pub group: Option<Vec<i64>>,
    pub search: Option<String>,
    pub status: Option<Vec<UserStatus>>,
    pub sort: Option<String>,
    pub proxy_id: Option<String>,
    pub data_limit_reset_strategy: Option<Vec<DataLimitResetStrategy>>,
    pub data_limit_min: Option<i64>,
    pub data_limit_max: Option<i64>,
    pub expire_after: Option<PanelDateTime>,
    pub expire_before: Option<PanelDateTime>,
    pub online_after: Option<PanelDateTime>,
    pub online_before: Option<PanelDateTime>,
    pub online: Option<bool>,
    pub no_data_limit: Option<bool>,
    pub no_expire: Option<bool>,
    pub load_sub: Option<bool>,
}

impl Default for GetUsersParams {
    fn default() -> Self {
        Self {
            offset: None,
            limit: None,
            ids: None,
            username: None,
            usernames: None,
            admin: None,
            admin_ids: None,
            group: None,
            search: None,
            status: None,
            sort: None,
            proxy_id: None,
            data_limit_reset_strategy: None,
            data_limit_min: None,
            data_limit_max: None,
            expire_after: None,
            expire_before: None,
            online_after: None,
            online_before: None,
            online: Some(false),
            no_data_limit: Some(false),
            no_expire: Some(false),
            load_sub: Some(false),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetUsersSimpleParams {
    pub ids: Option<Vec<i64>>,
    pub usernames: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub all: Option<bool>,
}

impl Default for GetUsersSimpleParams {
    fn default() -> Self {
        Self {
            ids: None,
            usernames: None,
            offset: None,
            limit: None,
            search: None,
            sort: None,
            all: Some(false),
        }
    }
}

/// Which users `get_expired_users` / `delete_expired_users` target.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExpiredUsersTarget {
    Expired,
    Limited,
}

impl ExpiredUsersTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Expired => "expired",
            Self::Limited => "limited",
        }
    }
}

impl std::fmt::Display for ExpiredUsersTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl crate::query::QueryValue for ExpiredUsersTarget {
    fn to_query(&self) -> String {
        self.as_str().to_owned()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpiredUsersParams {
    pub admin_username: Option<String>,
    pub target: Option<ExpiredUsersTarget>,
    pub expired_after: Option<PanelDateTime>,
    pub expired_before: Option<PanelDateTime>,
}

impl Default for ExpiredUsersParams {
    fn default() -> Self {
        Self {
            admin_username: None,
            target: Some(ExpiredUsersTarget::Expired),
            expired_after: None,
            expired_before: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetUsersSubUpdateChartParams {
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub admin_id: Option<i64>,
}

fn expired_users_query(params: &ExpiredUsersParams) -> Query {
    Query::new()
        .opt("admin_username", params.admin_username.as_deref())
        .opt("target", params.target)
        .opt("expired_after", params.expired_after.as_ref())
        .opt("expired_before", params.expired_before.as_ref())
}

fn page_query(params: &PageParams) -> Query {
    Query::new()
        .opt("offset", params.offset)
        .opt("limit", params.limit)
}

impl PanelClient {
    /// Create a user.
    /// Create a user.
    pub async fn create_user(&self, user: &UserCreate) -> PanelResult<UserResponse> {
        self.call(Method::POST, "/api/user", Query::new(), json_body(user)?)
            .await
    }

    /// Get a user by username.
    /// Get a user by username.
    pub async fn get_user(&self, username: &str) -> PanelResult<UserResponse> {
        self.call(
            Method::GET,
            &format!("/api/user/{username}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Get a user through the by-username route.
    /// Get a user through the by-username route.
    pub async fn get_user_by_username(&self, username: &str) -> PanelResult<UserResponse> {
        self.call(
            Method::GET,
            &format!("/api/user/by-username/{username}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Get a user by id.
    /// Get a user by id.
    pub async fn get_user_by_id(&self, user_id: i64) -> PanelResult<UserResponse> {
        self.call(
            Method::GET,
            &format!("/api/user/by-id/{user_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify a user by id.
    /// Modify a user by id.
    pub async fn modify_user_by_id(
        &self,
        user_id: i64,
        user: &UserModify,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/by-id/{user_id}"),
            Query::new(),
            json_body(user)?,
        )
        .await
    }

    /// Remove a user by id.
    /// Remove a user by id.
    pub async fn remove_user_by_id(&self, user_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/user/by-id/{user_id}"),
            Query::new(),
        )
        .await
    }

    /// Reset the data usage of a user by id.
    /// Reset the data usage of a user by id.
    pub async fn reset_user_data_usage_by_id(&self, user_id: i64) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/by-id/{user_id}/reset"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Revoke the subscription of a user by username.
    /// Revoke the subscription of a user by username.
    pub async fn revoke_user_subscription(&self, username: &str) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/{username}/revoke_sub"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Revoke the subscription of a user by id.
    /// Revoke the subscription of a user by id.
    pub async fn revoke_user_subscription_by_id(&self, user_id: i64) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/by-id/{user_id}/revoke_sub"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Set the owner admin of a user by username.
    /// Set the owner admin of a user by username.
    pub async fn set_owner(
        &self,
        username: &str,
        admin_username: &str,
    ) -> PanelResult<UserResponse> {
        let query = Query::new().set("admin_username", admin_username);
        self.call(
            Method::PUT,
            &format!("/api/user/{username}/set_owner"),
            query,
            Body::None,
        )
        .await
    }

    /// Set the owner admin of a user by id.
    /// Set the owner admin of a user by id.
    pub async fn set_owner_by_id(
        &self,
        user_id: i64,
        admin_username: &str,
    ) -> PanelResult<UserResponse> {
        let query = Query::new().set("admin_username", admin_username);
        self.call(
            Method::PUT,
            &format!("/api/user/by-id/{user_id}/set_owner"),
            query,
            Body::None,
        )
        .await
    }

    /// Get the subscription of a user by id in the given client format.
    /// Get the subscription of a user by id in the given client format.
    pub async fn get_user_subscription_by_id(
        &self,
        user_id: i64,
        client_type: ConfigFormat,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::GET,
            &format!("/api/user/{user_id}/subscription/{client_type}"),
            Body::None,
        )
        .await
    }

    /// List subscription updates of a user through the by-username route.
    /// List subscription updates of a user through the by-username route.
    pub async fn get_user_sub_update_list_by_username(
        &self,
        username: &str,
        params: &PageParams,
    ) -> PanelResult<UserSubscriptionUpdateList> {
        let query = Query::new()
            .opt("offset", params.offset)
            .opt("limit", params.limit);
        self.call(
            Method::GET,
            &format!("/api/user/by-username/{username}/sub_update"),
            query,
            Body::None,
        )
        .await
    }

    /// List users with filters.
    /// List users with filters.
    pub async fn get_users(&self, params: &GetUsersParams) -> PanelResult<UsersResponse> {
        let p = params;
        let query = Query::new()
            .opt("offset", p.offset)
            .opt("limit", p.limit)
            .list("ids", p.ids.as_deref())
            .list("username", p.username.as_deref())
            .list("usernames", p.usernames.as_deref())
            .list("admin", p.admin.as_deref())
            .list("admin_ids", p.admin_ids.as_deref())
            .list("group", p.group.as_deref())
            .opt("search", p.search.as_deref())
            .list("status", p.status.as_deref())
            .opt("sort", p.sort.as_deref())
            .opt("proxy_id", p.proxy_id.as_deref())
            .list(
                "data_limit_reset_strategy",
                p.data_limit_reset_strategy.as_deref(),
            )
            .opt("data_limit_min", p.data_limit_min)
            .opt("data_limit_max", p.data_limit_max)
            .opt("expire_after", p.expire_after.as_ref())
            .opt("expire_before", p.expire_before.as_ref())
            .opt("online_after", p.online_after.as_ref())
            .opt("online_before", p.online_before.as_ref())
            .opt("online", p.online)
            .opt("no_data_limit", p.no_data_limit)
            .opt("no_expire", p.no_expire)
            .opt("load_sub", p.load_sub);
        self.call(Method::GET, "/api/users", query, Body::None)
            .await
    }

    /// List users with only their id and username.
    /// List users with only their id and username.
    pub async fn get_users_simple(
        &self,
        params: &GetUsersSimpleParams,
    ) -> PanelResult<UsersSimpleResponse> {
        let p = params;
        let query = Query::new()
            .list("ids", p.ids.as_deref())
            .list("usernames", p.usernames.as_deref())
            .opt("offset", p.offset)
            .opt("limit", p.limit)
            .opt("search", p.search.as_deref())
            .opt("sort", p.sort.as_deref())
            .opt("all", p.all);
        self.call(Method::GET, "/api/users/simple", query, Body::None)
            .await
    }

    /// Get the traffic usage of a user by id.
    /// Get the traffic usage of a user by id.
    pub async fn get_user_usage_by_id(
        &self,
        user_id: i64,
        params: &UsageParams,
    ) -> PanelResult<UserUsageStatsList> {
        self.call(
            Method::GET,
            &format!("/api/user/by-id/{user_id}/usage"),
            usage_query(params),
            Body::None,
        )
        .await
    }

    /// Delete several users.
    /// Delete several users.
    pub async fn bulk_delete_users(
        &self,
        bulk: &BulkUsersSelection,
    ) -> PanelResult<RemoveUsersResponse> {
        self.call(
            Method::POST,
            "/api/users/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Disable several users.
    /// Disable several users.
    pub async fn bulk_disable_users(
        &self,
        bulk: &BulkUsersSelection,
    ) -> PanelResult<BulkUsersActionResponse> {
        self.call(
            Method::POST,
            "/api/users/bulk/disable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Enable several users.
    /// Enable several users.
    pub async fn bulk_enable_users(
        &self,
        bulk: &BulkUsersSelection,
    ) -> PanelResult<BulkUsersActionResponse> {
        self.call(
            Method::POST,
            "/api/users/bulk/enable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Set the owner admin of several users.
    /// Set the owner admin of several users.
    pub async fn bulk_set_owner(
        &self,
        bulk: &BulkUsersSetOwner,
    ) -> PanelResult<BulkUsersActionResponse> {
        self.call(
            Method::PUT,
            "/api/users/bulk/set_owner",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Shift the expiry of matching users.
    /// Shift the expiry of matching users.
    pub async fn bulk_modify_users_expire(&self, bulk: &BulkUser) -> PanelResult<AnyPayload> {
        self.call_any(Method::POST, "/api/users/bulk/expire", json_body(bulk)?)
            .await
    }

    /// Shift the data limit of matching users.
    /// Shift the data limit of matching users.
    pub async fn bulk_modify_users_datalimit(&self, bulk: &BulkUser) -> PanelResult<AnyPayload> {
        self.call_any(Method::POST, "/api/users/bulk/data_limit", json_body(bulk)?)
            .await
    }

    /// Create a user and add it to every group on the panel.
    pub async fn create_user_in_all_groups(&self, user: &UserCreate) -> PanelResult<UserResponse> {
        let groups = self.get_groups_simple(&SimpleListParams::all()).await?;
        let mut user = user.clone();
        user.group_ids = Some(groups.groups.iter().map(|group| group.id).collect());
        self.create_user(&user).await
    }

    /// Modify a user by username.
    pub async fn modify_user(
        &self,
        username: &str,
        user: &UserModify,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/{username}"),
            Query::new(),
            json_body(user)?,
        )
        .await
    }

    /// Remove a user by username.
    pub async fn remove_user(&self, username: &str) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/user/{username}"),
            Query::new(),
        )
        .await
    }

    /// Modify a user through the by-username route.
    pub async fn modify_user_by_username(
        &self,
        username: &str,
        user: &UserModify,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/by-username/{username}"),
            Query::new(),
            json_body(user)?,
        )
        .await
    }

    /// Remove a user through the by-username route.
    pub async fn remove_user_by_username(&self, username: &str) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/user/by-username/{username}"),
            Query::new(),
        )
        .await
    }

    /// Reset the data usage of a user by username.
    pub async fn reset_user_data_usage(&self, username: &str) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/{username}/reset"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Reset the data usage of a user through the by-username route.
    pub async fn reset_user_data_usage_by_username(
        &self,
        username: &str,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/by-username/{username}/reset"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Revoke the subscription of a user through the by-username route.
    pub async fn revoke_user_subscription_by_username(
        &self,
        username: &str,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/by-username/{username}/revoke_sub"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Reset the data usage of every user.
    pub async fn reset_users_data_usage(&self) -> PanelResult<AnyPayload> {
        self.call_any(Method::POST, "/api/users/reset", Body::None)
            .await
    }

    /// Get the subscription update chart, optionally filtered by user or admin.
    pub async fn get_users_sub_update_chart(
        &self,
        params: &GetUsersSubUpdateChartParams,
    ) -> PanelResult<UserSubscriptionUpdateChart> {
        let query = Query::new()
            .opt("user_id", params.user_id)
            .opt("username", params.username.as_deref())
            .opt("admin_id", params.admin_id);
        self.call(
            Method::GET,
            "/api/users/sub_update/chart",
            query,
            Body::None,
        )
        .await
    }

    /// Set the owner admin of a user through the by-username route.
    pub async fn set_owner_by_username(
        &self,
        username: &str,
        admin_username: &str,
    ) -> PanelResult<UserResponse> {
        let query = Query::new().set("admin_username", admin_username);
        self.call(
            Method::PUT,
            &format!("/api/user/by-username/{username}/set_owner"),
            query,
            Body::None,
        )
        .await
    }

    /// Activate the next plan of a user by username.
    pub async fn active_next_plan(&self, username: &str) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/{username}/active_next"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Activate the next plan of a user through the by-username route.
    pub async fn active_next_plan_by_username(&self, username: &str) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/by-username/{username}/active_next"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Activate the next plan of a user by id.
    pub async fn active_next_plan_by_id(&self, user_id: i64) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            &format!("/api/user/by-id/{user_id}/active_next"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// List subscription updates of a user by username.
    pub async fn get_user_sub_update_list(
        &self,
        username: &str,
        params: &PageParams,
    ) -> PanelResult<UserSubscriptionUpdateList> {
        self.call(
            Method::GET,
            &format!("/api/user/{username}/sub_update"),
            page_query(params),
            Body::None,
        )
        .await
    }

    /// List subscription updates of a user by id.
    pub async fn get_user_sub_update_list_by_id(
        &self,
        user_id: i64,
        params: &PageParams,
    ) -> PanelResult<UserSubscriptionUpdateList> {
        self.call(
            Method::GET,
            &format!("/api/user/by-id/{user_id}/sub_update"),
            page_query(params),
            Body::None,
        )
        .await
    }

    /// Get the traffic usage of a user by username.
    pub async fn get_user_usage(
        &self,
        username: &str,
        params: &UsageParams,
    ) -> PanelResult<UserUsageStatsList> {
        self.call(
            Method::GET,
            &format!("/api/user/{username}/usage"),
            usage_query(params),
            Body::None,
        )
        .await
    }

    /// Get the traffic usage of a user through the by-username route.
    pub async fn get_user_usage_by_username(
        &self,
        username: &str,
        params: &UsageParams,
    ) -> PanelResult<UserUsageStatsList> {
        self.call(
            Method::GET,
            &format!("/api/user/by-username/{username}/usage"),
            usage_query(params),
            Body::None,
        )
        .await
    }

    /// Get the aggregated traffic usage of all users, optionally limited to some admins.
    pub async fn get_users_usage(
        &self,
        params: &UsageParams,
        admin: Option<&[String]>,
    ) -> PanelResult<UserUsageStatsList> {
        let query = usage_query(params).list("admin", admin);
        self.call(Method::GET, "/api/users/usage", query, Body::None)
            .await
    }

    /// Get a user count metric over time, optionally limited to some admins.
    pub async fn get_users_count_metric(
        &self,
        metric: UserCountMetric,
        params: &UsageParams,
        admin: Option<&[String]>,
    ) -> PanelResult<UserCountMetricStatsList> {
        let query = usage_query(params).list("admin", admin);
        self.call(
            Method::GET,
            &format!("/api/users/counts/{metric}"),
            query,
            Body::None,
        )
        .await
    }

    /// List the usernames of expired or limited users.
    pub async fn get_expired_users(&self, params: &ExpiredUsersParams) -> PanelResult<Vec<String>> {
        self.call(
            Method::GET,
            "/api/users/expired",
            expired_users_query(params),
            Body::None,
        )
        .await
    }

    /// Delete expired or limited users.
    pub async fn delete_expired_users(
        &self,
        params: &ExpiredUsersParams,
    ) -> PanelResult<RemoveUsersResponse> {
        self.call(
            Method::DELETE,
            "/api/users/expired",
            expired_users_query(params),
            Body::None,
        )
        .await
    }

    /// Reset the data usage of several users.
    pub async fn bulk_reset_users_data_usage(
        &self,
        bulk: &BulkUsersSelection,
    ) -> PanelResult<BulkUsersActionResponse> {
        self.call(
            Method::POST,
            "/api/users/bulk/reset",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Revoke the subscription of several users.
    pub async fn bulk_revoke_users_subscription(
        &self,
        bulk: &BulkUsersSelection,
    ) -> PanelResult<BulkUsersActionResponse> {
        self.call(
            Method::POST,
            "/api/users/bulk/revoke_sub",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Create a user from a user template.
    pub async fn create_user_from_template(
        &self,
        template_user: &CreateUserFromTemplate,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::POST,
            "/api/user/from_template",
            Query::new(),
            json_body(template_user)?,
        )
        .await
    }

    /// Create several users from a user template.
    pub async fn bulk_create_users_from_template(
        &self,
        bulk: &BulkUsersFromTemplate,
    ) -> PanelResult<BulkUsersCreateResponse> {
        self.call(
            Method::POST,
            "/api/users/bulk/from_template",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Apply a user template to several users.
    pub async fn bulk_apply_template_to_users(
        &self,
        bulk: &BulkUsersApplyTemplate,
    ) -> PanelResult<BulkUsersActionResponse> {
        self.call(
            Method::POST,
            "/api/users/bulk/apply_template",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Apply a user template to a user by username.
    pub async fn modify_user_with_template(
        &self,
        username: &str,
        template_user: &ModifyUserByTemplate,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/from_template/{username}"),
            Query::new(),
            json_body(template_user)?,
        )
        .await
    }

    /// Apply a user template to a user through the by-username route.
    pub async fn modify_user_with_template_by_username(
        &self,
        username: &str,
        template_user: &ModifyUserByTemplate,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/from_template/by-username/{username}"),
            Query::new(),
            json_body(template_user)?,
        )
        .await
    }

    /// Apply a user template to a user by id.
    pub async fn modify_user_with_template_by_id(
        &self,
        user_id: i64,
        template_user: &ModifyUserByTemplate,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/from_template/by-id/{user_id}"),
            Query::new(),
            json_body(template_user)?,
        )
        .await
    }

    /// Change the proxy settings of matching users.
    pub async fn bulk_modify_users_proxy_settings(
        &self,
        bulk: &BulkUsersProxy,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            "/api/users/bulk/proxy_settings",
            json_body(bulk)?,
        )
        .await
    }

    /// Disable or enable a user by username.
    pub async fn set_user_disabled(
        &self,
        username: &str,
        status: &UserStatusToggle,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/{username}/disabled"),
            Query::new(),
            json_body(status)?,
        )
        .await
    }

    /// Disable or enable a user through the by-username route.
    pub async fn set_user_disabled_by_username(
        &self,
        username: &str,
        status: &UserStatusToggle,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/by-username/{username}/disabled"),
            Query::new(),
            json_body(status)?,
        )
        .await
    }

    /// Disable or enable a user by id.
    pub async fn set_user_disabled_by_id(
        &self,
        user_id: i64,
        status: &UserStatusToggle,
    ) -> PanelResult<UserResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user/by-id/{user_id}/disabled"),
            Query::new(),
            json_body(status)?,
        )
        .await
    }
}
