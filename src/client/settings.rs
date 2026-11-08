use super::*;

impl PanelClient {
    /// Get all panel settings.
    pub async fn get_settings(&self) -> PanelResult<SettingsSchema> {
        self.call(Method::GET, "/api/settings", Query::new(), Body::None)
            .await
    }

    /// Replace the panel settings and return the stored result.
    pub async fn modify_settings(&self, settings: &SettingsSchema) -> PanelResult<SettingsSchema> {
        self.call(
            Method::PUT,
            "/api/settings",
            Query::new(),
            json_body(settings)?,
        )
        .await
    }

    /// Get the general settings section.
    pub async fn get_general_settings(&self) -> PanelResult<General> {
        self.call(
            Method::GET,
            "/api/settings/general",
            Query::new(),
            Body::None,
        )
        .await
    }
}
