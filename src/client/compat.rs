use super::subscription::sub_usage_query;
use super::*;

impl PanelClient {
    /// Legacy alias of [`PanelClient::create_user`].
    pub async fn add_user(&self, user: &UserCreate) -> PanelResult<UserResponse> {
        self.create_user(user).await
    }

    /// Legacy alias of [`PanelClient::active_next_plan`].
    pub async fn activate_next_plan(&self, username: &str) -> PanelResult<UserResponse> {
        self.active_next_plan(username).await
    }

    /// Legacy alias of [`PanelClient::get_user_usage`] with Python's argument names.
    pub async fn get_user_data_usage(
        &self,
        username: &str,
        start_date: Option<PanelDateTime>,
        end_date: Option<PanelDateTime>,
        period: Period,
    ) -> PanelResult<UserUsageStatsList> {
        let params = UsageParams {
            period: Some(period),
            start: start_date,
            end: end_date,
            ..UsageParams::default()
        };
        self.get_user_usage(username, &params).await
    }

    /// Legacy alias of [`PanelClient::update_core`] taking the core version directly.
    pub async fn update_node_core(
        &self,
        node_id: i64,
        core_version: &str,
    ) -> PanelResult<AnyPayload> {
        let update = NodeCoreUpdate {
            core_version: Some(core_version.to_owned()),
            ..NodeCoreUpdate::default()
        };
        self.update_core(node_id, &update).await
    }

    /// Legacy alias of [`PanelClient::update_geofiles`] taking the region directly.
    pub async fn update_node_geofiles(
        &self,
        node_id: i64,
        region: GeoFilseRegion,
    ) -> PanelResult<AnyPayload> {
        let update = NodeGeoFilesUpdate {
            region: Some(region),
            ..NodeGeoFilesUpdate::default()
        };
        self.update_geofiles(node_id, &update).await
    }

    /// Get subscription user information from a full subscription URL.
    pub async fn get_user_subscription_info(
        &self,
        url: &str,
    ) -> PanelResult<SubscriptionUserResponse> {
        let target = format!("{}/info", url.trim_end_matches('/'));
        let raw = self
            .send(Method::GET, &target, Query::new(), Body::None, false)
            .await?;
        decode(raw)
    }

    /// Get subscription usage from a full subscription URL.
    pub async fn get_sub_user_usage_by_url(
        &self,
        url: &str,
        params: &SubUsageParams,
    ) -> PanelResult<UserUsageStatsList> {
        let target = format!("{}/usage", url.trim_end_matches('/'));
        let raw = self
            .send(
                Method::GET,
                &target,
                sub_usage_query(params),
                Body::None,
                false,
            )
            .await?;
        decode(raw)
    }

    /// Fetch subscription links from a full subscription URL, dropping placeholder entries.
    pub async fn get_user_subscription_with_client_type(
        &self,
        client_type: &str,
        url: &str,
    ) -> PanelResult<Vec<String>> {
        let target = format!("{}/{client_type}", url.trim_end_matches('/'));
        let raw = self
            .send(Method::GET, &target, Query::new(), Body::None, false)
            .await?;
        let text = String::from_utf8_lossy(&raw.body);
        Ok(py_splitlines(py_strip(&text))
            .into_iter()
            .filter(|line| !line.is_empty() && !line.contains("@0.0.0.0:"))
            .map(str::to_owned)
            .collect())
    }
}
