use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetClientTemplatesParams {
    pub ids: Option<Vec<i64>>,
    pub template_type: Option<ClientTemplateType>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetClientTemplatesSimpleParams {
    pub ids: Option<Vec<i64>>,
    pub template_type: Option<ClientTemplateType>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub all: Option<bool>,
}

impl Default for GetClientTemplatesSimpleParams {
    fn default() -> Self {
        Self {
            ids: None,
            template_type: None,
            offset: None,
            limit: None,
            search: None,
            sort: None,
            all: Some(false),
        }
    }
}

impl PanelClient {
    /// Create a new client template.
    pub async fn create_client_template(
        &self,
        template: &ClientTemplateCreate,
    ) -> PanelResult<ClientTemplateResponse> {
        self.call(
            Method::POST,
            "/api/client_template",
            Query::new(),
            json_body(template)?,
        )
        .await
    }

    /// Get a client template by its ID.
    pub async fn get_client_template(
        &self,
        template_id: i64,
    ) -> PanelResult<ClientTemplateResponse> {
        self.call(
            Method::GET,
            &format!("/api/client_template/{template_id}"),
            Query::new(),
            Body::None,
        )
        .await
    }

    /// Modify a client template by its ID.
    pub async fn modify_client_template(
        &self,
        template_id: i64,
        template: &ClientTemplateModify,
    ) -> PanelResult<ClientTemplateResponse> {
        self.call(
            Method::PUT,
            &format!("/api/client_template/{template_id}"),
            Query::new(),
            json_body(template)?,
        )
        .await
    }

    /// Remove a client template by its ID.
    pub async fn remove_client_template(&self, template_id: i64) -> PanelResult<()> {
        self.call_unit(
            Method::DELETE,
            &format!("/api/client_template/{template_id}"),
            Query::new(),
        )
        .await
    }

    /// Get a list of client templates.
    pub async fn get_client_templates(
        &self,
        params: &GetClientTemplatesParams,
    ) -> PanelResult<ClientTemplateResponseList> {
        let query = Query::new()
            .list("ids", params.ids.as_deref())
            .opt("template_type", params.template_type)
            .opt("offset", params.offset)
            .opt("limit", params.limit);
        self.call(Method::GET, "/api/client_templates", query, Body::None)
            .await
    }

    /// Get a lightweight client template list.
    pub async fn get_client_templates_simple(
        &self,
        params: &GetClientTemplatesSimpleParams,
    ) -> PanelResult<ClientTemplatesSimpleResponse> {
        let query = Query::new()
            .list("ids", params.ids.as_deref())
            .opt("template_type", params.template_type)
            .opt("offset", params.offset)
            .opt("limit", params.limit)
            .opt("search", params.search.as_deref())
            .opt("sort", params.sort.as_deref())
            .opt("all", params.all);
        self.call(
            Method::GET,
            "/api/client_templates/simple",
            query,
            Body::None,
        )
        .await
    }

    /// Delete the selected client templates.
    pub async fn bulk_delete_client_templates(
        &self,
        bulk: &BulkClientTemplateSelection,
    ) -> PanelResult<RemoveClientTemplatesResponse> {
        self.call(
            Method::POST,
            "/api/client_templates/bulk/delete",
            Query::new(),
            json_body(bulk)?,
        )
        .await
    }
}
