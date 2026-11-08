use pasarguard_api::Credential;
use pasarguard_api::models::*;
use pasarguard_api::query::Query;
use serde_json::{Value, json};

#[test]
fn credential_detects_api_keys() {
    assert!(Credential::from_token("pg_key_abc").is_api_key());
    assert!(!Credential::from_token("eyJhbGciOi").is_api_key());
    assert_eq!(
        Credential::from_token("pg_key_abc").header(),
        Some(("x-api-key", "pg_key_abc".to_owned()))
    );
    assert_eq!(
        Credential::from_token("jwt").header(),
        Some(("authorization", "Bearer jwt".to_owned()))
    );
}

#[test]
fn user_response_keeps_unknown_fields() {
    let user: UserResponse = serde_json::from_value(json!({
        "id": 7,
        "username": "alice",
        "status": "active",
        "used_traffic": 1024,
        "created_at": "2025-01-02T03:04:05",
        "a_field_from_a_newer_panel": true
    }))
    .unwrap();
    assert_eq!(user.username, "alice");
    assert_eq!(user.status, UserStatus::Active);
    assert_eq!(
        user.extra.get("a_field_from_a_newer_panel"),
        Some(&Value::Bool(true))
    );
}

#[test]
fn request_models_skip_unset_fields() {
    let body = serde_json::to_value(UserModify {
        note: Some("vip".into()),
        ..UserModify::default()
    })
    .unwrap();
    assert_eq!(body, json!({ "note": "vip" }));
}

#[test]
fn node_create_always_sends_keep_alive() {
    let node = NodeCreate::new(
        "de-1",
        "1.2.3.4",
        NodeConnectionType::Grpc,
        "-----BEGIN CERTIFICATE-----",
        60,
        1,
        "key",
    );
    let body = serde_json::to_value(&node).unwrap();
    assert_eq!(body["keep_alive"], json!(60));
    assert_eq!(body["connection_type"], json!("grpc"));
}

#[test]
fn permission_values_accept_flags_and_scopes() {
    let perms: UsersPermissions = serde_json::from_value(json!({
        "create": true,
        "read": 2,
        "update": { "scope": 1 }
    }))
    .unwrap();
    assert_eq!(perms.create, Some(PermissionValue::Bool(true)));
    assert_eq!(
        perms.read,
        Some(PermissionValue::Scope(PermissionScope::All))
    );
    assert_eq!(
        perms.update,
        Some(PermissionValue::Scope(PermissionScope::Own))
    );
}

#[test]
fn enums_use_panel_spelling() {
    assert_eq!(
        serde_json::to_value(ProxyHostALPN::Http11).unwrap(),
        json!("http/1.1")
    );
    assert_eq!(
        "long-polling".parse::<RunMethod>(),
        Ok(RunMethod::LongPolling)
    );
    assert_eq!(FlowOption::None.as_str(), "");
}

#[test]
fn list_query_params_repeat_the_key() {
    let query = Query::new()
        .list(
            "status",
            Some(&[UserStatus::Active, UserStatus::OnHold][..]),
        )
        .opt("limit", Some(10_i64));
    assert_eq!(query.encode(), "status=active&status=on_hold&limit=10");
}
