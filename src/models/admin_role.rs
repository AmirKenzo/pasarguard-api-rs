use super::*;
use crate::datetime::PanelDateTime;
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RoleLimits {
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_users: Option<i64>,
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_limit_min: Option<i64>,
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_limit_max: Option<i64>,
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_min: Option<i64>,
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_max: Option<i64>,
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_hwid_per_user: Option<i64>,
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_hwid_per_user: Option<i64>,
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_hold_timeout_min: Option<i64>,
    #[serde(
        default,
        deserialize_with = "lax_int::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_hold_timeout_max: Option<i64>,
    #[serde(flatten)]
    pub extra: Extra,
}

impl RoleLimits {
    pub fn max_users(value: i64) -> Self {
        Self {
            max_users: Some(value),
            ..Self::default()
        }
    }

    pub fn from_value(value: Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

/// A role permission that is either a plain flag or a `PermissionScope`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermissionValue {
    Bool(bool),
    Scope(PermissionScope),
}

impl From<bool> for PermissionValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<PermissionScope> for PermissionValue {
    fn from(value: PermissionScope) -> Self {
        Self::Scope(value)
    }
}

impl Serialize for PermissionValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bool(flag) => serializer.serialize_bool(*flag),
            Self::Scope(PermissionScope::None) => serializer.serialize_none(),
            Self::Scope(scope) => {
                let mut map = Map::new();
                map.insert("scope".into(), Value::from(scope.as_i64()));
                map.serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for PermissionValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let scope = match &value {
            Value::Bool(flag) => return Ok(Self::Bool(*flag)),
            Value::Number(_) => lax_int::from_value(&value),
            Value::Object(map) => map.get("scope").and_then(lax_int::from_value),
            _ => None,
        };
        scope
            .and_then(PermissionScope::from_i64)
            .map(Self::Scope)
            .ok_or_else(|| serde::de::Error::custom(format!("invalid permission: {value}")))
    }
}

fn permission_unset(value: &Option<PermissionValue>) -> bool {
    matches!(
        value,
        None | Some(PermissionValue::Scope(PermissionScope::None))
    )
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CRUDPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_simple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminsPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_simple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_usage: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodesPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_simple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_core: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HostsPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HwidsPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct APIKeysPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub read: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub read_simple: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub update: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub delete: Option<PermissionValue>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SettingsPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_general: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersPermissions {
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub create: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub read: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub read_simple: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub update: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub delete: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub reset_usage: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub revoke_sub: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub set_owner: Option<PermissionValue>,
    #[serde(default, skip_serializing_if = "permission_unset")]
    pub activate_next_plan: Option<PermissionValue>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RoleFeatures {
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub can_use_reset_strategy: Option<bool>,
    #[serde(default = "default_some_true", skip_serializing_if = "Option::is_none")]
    pub can_use_next_plan: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RoleAccess {
    #[serde(
        default = "default_some_false",
        skip_serializing_if = "Option::is_none"
    )]
    pub require_template: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_template_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_group_ids: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RolePermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<UsersPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admins: Option<AdminsPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<NodesPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<CRUDPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<HostsPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<CRUDPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_templates: Option<CRUDPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<CRUDPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<SettingsPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hwids: Option<HwidsPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_roles: Option<CRUDPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_keys: Option<APIKeysPermissions>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct AdminRoleCreate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RolePermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<RoleLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<RoleFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<RoleAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid: Option<RoleHWIDSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_when_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_users_when_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_users_when_disabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminRoleData {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default = "default_some_empty_string")]
    pub name: Option<String>,
    #[serde(default = "default_some_false")]
    pub is_owner: Option<bool>,
    #[serde(default)]
    pub permissions: Option<RolePermissions>,
    #[serde(default)]
    pub limits: Option<RoleLimits>,
    #[serde(default)]
    pub features: Option<RoleFeatures>,
    #[serde(default)]
    pub access: Option<RoleAccess>,
    #[serde(default)]
    pub hwid: Option<RoleHWIDSettings>,
    #[serde(default = "default_some_false")]
    pub disabled_when_limited: Option<bool>,
    #[serde(default = "default_some_true")]
    pub disconnect_users_when_limited: Option<bool>,
    #[serde(default = "default_some_true")]
    pub disconnect_users_when_disabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct AdminRoleModify {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RolePermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<RoleLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<RoleFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<RoleAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid: Option<RoleHWIDSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_when_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_users_when_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_users_when_disabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminRoleResponse {
    pub name: String,
    #[serde(default)]
    pub permissions: Option<RolePermissions>,
    #[serde(default)]
    pub limits: Option<RoleLimits>,
    #[serde(default)]
    pub features: Option<RoleFeatures>,
    #[serde(default)]
    pub access: Option<RoleAccess>,
    #[serde(default)]
    pub hwid: Option<RoleHWIDSettings>,
    #[serde(default = "default_some_false")]
    pub disabled_when_limited: Option<bool>,
    #[serde(default = "default_some_true")]
    pub disconnect_users_when_limited: Option<bool>,
    #[serde(default = "default_some_true")]
    pub disconnect_users_when_disabled: Option<bool>,
    pub id: i64,
    pub is_owner: bool,
    pub created_at: PanelDateTime,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminRoleSimple {
    pub id: i64,
    pub name: String,
    pub is_owner: bool,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminRolesResponse {
    pub roles: Vec<AdminRoleResponse>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminRolesSimpleResponse {
    pub roles: Vec<AdminRoleSimple>,
    pub total: i64,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct OwnerCreateRequest {
    pub key: String,
    pub username: String,
    pub password: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct OwnerResetRequest {
    pub key: String,
    pub password: String,
    #[serde(flatten)]
    pub extra: Extra,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct OwnerUpgradeRequest {
    pub key: String,
    pub username: String,
    #[serde(flatten)]
    pub extra: Extra,
}
