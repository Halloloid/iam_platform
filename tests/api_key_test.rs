use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::common::get_first_permission_id;

mod common;

#[sqlx::test]
async fn test_create_api_key_success(pool: PgPool) {
    let (app, token, org_id) = common::setup_org(pool, "create_api@test.com").await;

    let perm_id = common::get_first_permission_id(app.clone(), &token).await;

    let (status, body) = common::request_json_auth(
        app,
        json!({
            "name":"Test Key",
            "permission_ids":[perm_id],
            "expires_in_dayes":1
        }),
        "POST",
        &format!("/organization/{}/api_key", org_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["id"].is_string());
    assert!(body["raw_key"].is_string());
    assert!(body["raw_key"].as_str().unwrap().starts_with("iam_"));
}

#[sqlx::test]
async fn test_create_api_key_show_only_once(pool: PgPool) {
    let (app, token, org_id) = common::setup_org(pool, "show_once@test.com").await;

    let perm_id = get_first_permission_id(app.clone(), &token).await;

    let (_, body) = common::request_json_auth(
        app.clone(),
        json!({
            "name":"Test Key",
            "permission_ids":[perm_id],
            "expires_in_dayes":1
        }),
        "POST",
        &format!("/organization/{}/api_key", org_id),
        &token,
    )
    .await;

    assert!(body["raw_key"].is_string());

    let (_, body) = common::get_json(
        app,
        &format!("/organization/{}/api_key", org_id),
        Some(&token),
    )
    .await;

    let keys = body["data"].as_array().unwrap();

    assert!(
        keys.iter().any(|r| r.get("raw_key").is_none()),
        "raw key must not appear in the list"
    );
}

#[sqlx::test]
async fn test_create_api_key_without_scopes_fails(pool: PgPool) {
    let (app, token, org_id) = common::setup_org(pool, "without_scope_fails@test.com").await;

    let (status, _) = common::request_json_auth(
        app,
        json!({
            "name":"Test Key",
            "permission_ids":[],
            "expires_in_dayes":1
        }),
        "POST",
        &format!("/organization/{}/api_key", org_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[sqlx::test]
async fn test_create_api_key_without_permission_fails(pool: PgPool) {
    let (app, _owner_token, org_id) = common::setup_org(pool, "key_no_permission@test.com").await;

    let token2 = common::register_and_login(app.clone(), "outsider_key@test.com").await;

    let perm_id = common::get_first_permission_id(app.clone(), &token2).await;

    let (status, _) = common::request_json_auth(
        app,
        json!({
            "name":"Test Key",
            "permission_ids":[perm_id],
            "expires_in_dayes":1
        }),
        "POST",
        &format!("/organization/{}/api_key", org_id),
        &token2,
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}
