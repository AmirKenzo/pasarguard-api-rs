use super::*;

impl PanelClient {
    /// Get system and user statistics, optionally scoped to one admin.
    pub async fn get_system_stats(&self, admin_username: Option<&str>) -> PanelResult<SystemStats> {
        let query = Query::new().opt("admin_username", admin_username);
        self.call(Method::GET, "/api/system", query, Body::None)
            .await
    }

    /// Get user statistics, optionally scoped to one admin.
    pub async fn get_system_users_stats(
        &self,
        admin_username: Option<&str>,
    ) -> PanelResult<SystemUsersStats> {
        let query = Query::new().opt("admin_username", admin_username);
        self.call(Method::GET, "/api/system/users", query, Body::None)
            .await
    }

    /// Get host resource statistics.
    pub async fn get_system_resource_stats(&self) -> PanelResult<SystemResourceStats> {
        self.call(
            Method::GET,
            "/api/system/resources",
            Query::new(),
            Body::None,
        )
        .await
    }

    /// List inbound tags.
    pub async fn get_inbounds(&self) -> PanelResult<Vec<String>> {
        self.call(Method::GET, "/api/inbounds", Query::new(), Body::None)
            .await
    }

    /// List inbounds with protocol and network details.
    pub async fn get_inbound_details(&self) -> PanelResult<Vec<InboundSummary>> {
        self.call(
            Method::GET,
            "/api/inbounds/details",
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Get the health of the background workers.
    pub async fn get_workers_health(&self) -> PanelResult<WorkersHealth> {
        self.call(Method::GET, "/api/workers/health", Query::new(), Body::None)
            .await
    }

    /// List WireGuard subnets and their address usage.
    pub async fn get_wireguard_subnets(&self) -> PanelResult<Vec<WireGuardSubnetUsage>> {
        self.call(
            Method::GET,
            "/api/wireguard/subnets",
            Query::new(),
            Body::None,
        )
        .await
    }
}
