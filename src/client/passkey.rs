use super::*;

impl PanelClient {
    /// Get WebAuthn assertion options for passkey login. Sent without credentials.
    pub async fn passkey_login_options(
        &self,
        body: &PasskeyOptionsRequest,
    ) -> PanelResult<AnyPayload> {
        let raw = self
            .send(
                Method::POST,
                "/api/admin/passkey/login/options",
                Query::new(),
                json_body(body)?,
                false,
            )
            .await?;
        Ok(AnyPayload::from_body(&raw.body))
    }

    /// Verify a WebAuthn assertion and issue an admin token. Sent without credentials.
    pub async fn passkey_login_verify(
        &self,
        body: &PasskeyAuthenticationRequest,
    ) -> PanelResult<Token> {
        let raw = self
            .send(
                Method::POST,
                "/api/admin/passkey/login/verify",
                Query::new(),
                json_body(body)?,
                false,
            )
            .await?;
        decode(raw)
    }

    /// List the passkeys of the current admin.
    pub async fn list_passkeys(&self) -> PanelResult<Vec<PasskeyInfo>> {
        self.call(Method::GET, "/api/admin/passkey", Query::new(), Body::None)
            .await
    }

    /// Get WebAuthn registration options for a new passkey of the current admin.
    pub async fn passkey_register_options(&self) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            "/api/admin/passkey/register/options",
            Body::None,
        )
        .await
    }

    /// Verify a WebAuthn attestation and store the passkey for the current admin.
    pub async fn passkey_register_verify(
        &self,
        body: &PasskeyRegistrationRequest,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            "/api/admin/passkey/register/verify",
            json_body(body)?,
        )
        .await
    }

    /// Delete one passkey of the current admin.
    pub async fn delete_passkey(&self, passkey_id: i64) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::DELETE,
            &format!("/api/admin/passkey/{passkey_id}"),
            Body::None,
        )
        .await
    }

    /// List the passkeys of another admin.
    pub async fn list_admin_passkeys(&self, admin_id: i64) -> PanelResult<Vec<PasskeyInfo>> {
        self.call(
            Method::GET,
            &format!("/api/admin/passkey/admins/{admin_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Get WebAuthn registration options for a new passkey of another admin.
    pub async fn admin_passkey_register_options(&self, admin_id: i64) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/admin/passkey/admins/{admin_id}/register/options"),
            Body::None,
        )
        .await
    }

    /// Verify a WebAuthn attestation and store the passkey for another admin.
    pub async fn admin_passkey_register_verify(
        &self,
        admin_id: i64,
        body: &PasskeyRegistrationRequest,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/admin/passkey/admins/{admin_id}/register/verify"),
            json_body(body)?,
        )
        .await
    }

    /// Delete one passkey of another admin.
    pub async fn delete_admin_passkey(
        &self,
        admin_id: i64,
        passkey_id: i64,
    ) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::DELETE,
            &format!("/api/admin/passkey/admins/{admin_id}/{passkey_id}"),
            Body::None,
        )
        .await
    }
}
