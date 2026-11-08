use super::*;

impl PanelClient {
    /// Get the hardware IDs registered for a user.
    pub async fn get_user_hwids(&self, user_id: i64) -> PanelResult<UserHWIDListResponse> {
        self.call(
            Method::GET,
            &format!("/api/user/{user_id}/hwids"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Delete one hardware ID of a user.
    pub async fn delete_user_hwid(&self, user_id: i64, hwid: &str) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::DELETE,
            &format!("/api/user/{user_id}/hwids/{hwid}"),
            Body::None,
        )
        .await
    }

    /// Delete every hardware ID of a user.
    pub async fn reset_user_hwids(&self, user_id: i64) -> PanelResult<AnyPayload> {
        self.call_any(
            Method::POST,
            &format!("/api/user/{user_id}/hwids/reset"),
            Body::None,
        )
        .await
    }
}
