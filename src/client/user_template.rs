use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetUserTemplatesParams {
    pub ids: Option<Vec<i64>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl PanelClient {
    /// Create a user template.
    pub async fn create_user_template(
        &self,
        template: &UserTemplateCreate,
    ) -> PanelResult<UserTemplateResponse> {
        self.call(
            Method::POST,
            "/api/user_template",
            Query::new(),
            json_body(template)?,
        )
        .await
    }

    /// Get a user template by id.
    pub async fn get_user_template(&self, template_id: i64) -> PanelResult<UserTemplateResponse> {
        self.call(
            Method::GET,
            &format!("/api/user_template/{template_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify a user template by id.
    pub async fn modify_user_template(
        &self,
        template_id: i64,
        template: &UserTemplateModify,
    ) -> PanelResult<UserTemplateResponse> {
        self.call(
            Method::PUT,
            &format!("/api/user_template/{template_id}"),
            Query::new(),
            json_body(template)?,
        )
        .await
    }

    /// Remove a user template by id.
    pub async fn remove_user_template(&self, template_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/user_template/{template_id}"),
            Query::new(),
        )
        .await
    }

    /// List user templates.
    pub async fn get_user_templates(
        &self,
        params: &GetUserTemplatesParams,
    ) -> PanelResult<Vec<UserTemplateResponse>> {
        let query = Query::new()
            .list("ids", params.ids.as_deref())
            .opt("offset", params.offset)
            .opt("limit", params.limit);
        self.call(Method::GET, "/api/user_templates", query, Body::None)
            .await
    }

    /// List user templates with only their id and name.
    pub async fn get_user_templates_simple(
        &self,
        params: &SimpleListParams,
    ) -> PanelResult<UserTemplatesSimpleResponse> {
        self.call(
            Method::GET,
            "/api/user_templates/simple",
            simple_list_query(params),
            Body::None,
        )
        .await
    }

    /// Delete several user templates.
    pub async fn bulk_delete_user_templates(
        &self,
        bulk: &BulkUserTemplateSelection,
    ) -> PanelResult<RemoveUserTemplatesResponse> {
        self.call(
            Method::POST,
            "/api/user_templates/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Disable several user templates.
    pub async fn bulk_disable_user_templates(
        &self,
        bulk: &BulkUserTemplateSelection,
    ) -> PanelResult<BulkUserTemplatesActionResponse> {
        self.call(
            Method::POST,
            "/api/user_templates/bulk/disable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }

    /// Enable several user templates.
    pub async fn bulk_enable_user_templates(
        &self,
        bulk: &BulkUserTemplateSelection,
    ) -> PanelResult<BulkUserTemplatesActionResponse> {
        self.call(
            Method::POST,
            "/api/user_templates/bulk/enable",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }
}
