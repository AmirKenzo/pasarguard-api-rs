use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkGroup {
    pub group_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_group_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_no_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admins: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct BulkGroupSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BulkGroupsActionResponse {
    pub groups: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GroupCreate {
    pub name: String,
    pub inbound_tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GroupModify {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GroupResponse {
    pub name: String,
    #[serde(default = "default_some_empty_vec")]
    pub inbound_tags: Option<Vec<String>>,
    #[serde(default = "default_some_false")]
    pub is_disabled: Option<bool>,
    pub id: i64,
    #[serde(default = "default_some_zero")]
    pub total_users: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

fn default_some_empty_vec() -> Option<Vec<String>> {
    Some(Vec::new())
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GroupSimple {
    pub id: i64,
    pub name: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GroupsResponse {
    pub groups: Vec<GroupResponse>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GroupsSimpleResponse {
    pub groups: Vec<GroupSimple>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RemoveGroupsResponse {
    pub groups: Vec<String>,
    pub count: i64,
    #[serde(flatten)]
    pub extra: Extra,
}
