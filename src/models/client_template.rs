use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkClientTemplateSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClientTemplateCreate {
    pub name: String,
    pub template_type: ClientTemplateType,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl ClientTemplateCreate {
    pub fn new(
        name: impl Into<String>,
        template_type: ClientTemplateType,
        content: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            template_type,
            content: content.into(),
            is_default: None,
            extra: Extra::default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ClientTemplateModify {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ClientTemplateResponse {
    pub id: i64,
    pub name: String,
    pub template_type: ClientTemplateType,
    pub content: String,
    pub is_default: bool,
    pub is_system: bool,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ClientTemplateResponseList {
    pub count: i64,
    #[serde(default = "default_template_list")]
    pub templates: Option<Vec<ClientTemplateResponse>>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_template_list() -> Option<Vec<ClientTemplateResponse>> {
    Some(Vec::new())
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ClientTemplateSimple {
    pub id: i64,
    pub name: String,
    pub template_type: ClientTemplateType,
    pub is_default: bool,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ClientTemplatesSimpleResponse {
    pub templates: Vec<ClientTemplateSimple>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RemoveClientTemplatesResponse {
    pub templates: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}
