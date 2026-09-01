use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::common::get_first_permission_id;

mod common;

// Api Key Creation

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

// List Api Keys -----
#[sqlx::test]
async fn test_list_api_keys_shows_scopes(pool: PgPool) {
    let (app, owner_token, org_id) = common::setup_org(pool, "list_key@test.com").await;

    let perm_id = common::get_first_permission_id(app.clone(), &owner_token).await;

    common::create_api_key(app.clone(), &org_id, &owner_token, &perm_id).await;

    let (status, body) = common::get_json(
        app,
        &format!("/organization/{}/api_key", org_id),
        Some(&owner_token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);

    let keys = body["data"].as_array().unwrap();

    assert_eq!(keys.len(), 1);

    assert!(keys[0]["scopes"].is_array());
    assert!(keys[0]["scopes"].as_array().unwrap().len() > 0);
}

//-- Revoke API Keys -----
#[sqlx::test]
async fn test_revoke_api_key_success(pool: PgPool) {
    let (app, owner_token, org_id) = common::setup_org(pool, "revoke_key@test.com").await;

    let perm_id = common::get_first_permission_id(app.clone(), &owner_token).await;

    let (key_id, _) = common::create_api_key(app.clone(), &org_id, &owner_token, &perm_id).await;

    let (status, _) = common::request_json_auth(
        app,
        json!({}),
        "DELETE",
        &format!("/organization/{}/api_key/{}", org_id, key_id),
        &owner_token,
    )
    .await;

    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[sqlx::test]
async fn test_revoke_already_revoked_key_fails(pool: PgPool) {
    let (app, owner_token, org_id) = common::setup_org(pool, "revoke_key@test.com").await;

    let perm_id = common::get_first_permission_id(app.clone(), &owner_token).await;

    let (key_id, _) = common::create_api_key(app.clone(), &org_id, &owner_token, &perm_id).await;

    common::request_json_auth(
        app.clone(),
        json!({}),
        "DELETE",
        &format!("/organization/{}/api_key/{}", org_id, key_id),
        &owner_token,
    )
    .await;

    let (status, _) = common::request_json_auth(
        app,
        json!({}),
        "DELETE",
        &format!("/organization/{}/api_key/{}", org_id, key_id),
        &owner_token,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}
