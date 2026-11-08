use super::*;

impl PanelClient {
    /// Create the owner admin with the setup key.
    pub async fn create_owner(&self, request: &OwnerCreateRequest) -> PanelResult<AdminDetails> {
        let raw = self
            .send(
                Method::POST,
                "/api/setup/owner",
                Query::new(),
                json_body(request)?,
                false,
            )
            .await?;
        decode(raw)
    }

    /// Delete the owner admin with the setup key.
    pub async fn delete_owner(&self, key: &str) -> PanelResult<()> {
        self.send(
            Method::DELETE,
            "/api/setup/owner",
            Query::new().set("key", key),
            Body::None,
            false,
        )
        .await
        .map(|_| ())
    }

    /// Upgrade an existing admin to owner with the setup key.
    pub async fn upgrade_owner(&self, request: &OwnerUpgradeRequest) -> PanelResult<AdminDetails> {
        let raw = self
            .send(
                Method::POST,
                "/api/setup/owner/upgrade",
                Query::new(),
                json_body(request)?,
                false,
            )
            .await?;
        decode(raw)
    }

    /// Reset the owner password with the setup key.
    pub async fn reset_owner_password(
        &self,
        request: &OwnerResetRequest,
    ) -> PanelResult<AdminDetails> {
        let raw = self
            .send(
                Method::PATCH,
                "/api/setup/owner",
                Query::new(),
                json_body(request)?,
                false,
            )
            .await?;
        decode(raw)
    }
}
