use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListApiKeysParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub key_id: Option<i64>,
    pub name: Option<String>,
    pub status: Option<APIKeyStatus>,
}

impl PanelClient {
    /// Create a new API key.
    pub async fn create_api_key(
        &self,
        api_key: &APIKeyCreate,
    ) -> PanelResult<APIKeyCreateResponse> {
        self.call(
            Method::POST,
            "/api/api_key",
            Query::new(),
            json_body(api_key)?,
        )
        .await
    }

    /// Get an API key by id.
    pub async fn get_api_key(&self, key_id: i64) -> PanelResult<APIKeyResponse> {
        self.call(
            Method::GET,
            &format!("/api/api_key/{key_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify an existing API key.
    pub async fn modify_api_key(
        &self,
        key_id: i64,
        api_key: &APIKeyUpdate,
    ) -> PanelResult<APIKeyResponse> {
        self.call(
            Method::PATCH,
            &format!("/api/api_key/{key_id}"),
            Query::new(),
            json_body(api_key)?,
        )
        .await
    }

    /// Remove an API key.
    pub async fn remove_api_key(&self, key_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/api_key/{key_id}"),
            Query::new(),
        )
        .await
    }

    /// Revoke an API key and issue a new secret for it.
    pub async fn revoke_api_key(&self, key_id: i64) -> PanelResult<APIKeyCreateResponse> {
        self.call(
            Method::POST,
            &format!("/api/api_key/{key_id}/revoke"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// List API keys with optional filters and pagination.
    pub async fn list_api_keys(&self, params: &ListApiKeysParams) -> PanelResult<APIKeysResponse> {
        let p = params;
        let query = Query::new()
            .opt("offset", p.offset)
            .opt("limit", p.limit)
            .opt("key_id", p.key_id)
            .opt("name", p.name.as_deref())
            .opt("status", p.status.as_ref());
        self.call(Method::GET, "/api/api_keys", query, Body::None)
            .await
    }

    /// Delete the selected API keys.
    pub async fn bulk_delete_api_keys(
        &self,
        bulk: &BulkAPIKeySelection,
    ) -> PanelResult<RemoveAPIKeysResponse> {
        self.call(
            Method::POST,
            "/api/api_keys/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }
}
