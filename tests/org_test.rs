use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::common::register_and_login;

mod common;

// Organization Creation
#[sqlx::test]
async fn test_create_org_success(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "org@test.com").await;

    let (status, body) = common::request_json_auth(
        app,
        json!({
            "name":"Acme Inc"
        }),
        "POST",
        "/organization",
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["id"].is_string());
}

#[sqlx::test]
async fn test_create_org_without_token_fails(pool: PgPool) {
    let app = common::build_app(pool);

    let (status, _) = common::request_json_no_auth(
        app,
        "POST",
        "/organization",
        json!({
            "name":"Acme Inc"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_create_org_empty_name_fails(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "org@test.com").await;

    let (status, _) =
        common::request_json_auth(app, json!({"name":""}), "POST", "/organization", &token).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// Bootstrap transation------

#[sqlx::test]
async fn creater_becomes_member(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "org@test.com").await;

    let (_, org_body) = common::request_json_auth(
        app.clone(),
        json!({"name":"Test Org"}),
        "POST",
        "/organization",
        &token,
    )
    .await;

    let org_id = org_body["id"].as_str().unwrap();

    let (status, body) = common::get_json(
        app,
        &format!("/organization/{}/member", org_id),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].as_array().unwrap().len() > 0);
}

#[sqlx::test]
async fn test_creator_gets_owner_role(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "owner@test.com").await;

    let (_, org_body) = common::request_json_auth(
        app.clone(),
        json!({
            "name":"Acme Inc"
        }),
        "POST",
        "/organization",
        &token,
    )
    .await;

    let org_id = org_body["id"].as_str().unwrap();

    let (status, body) =
        common::get_json(app, &format!("/organization/{}/role", org_id), Some(&token)).await;

    assert_eq!(status, StatusCode::OK);

    let roles = body.as_array().unwrap();

    let has_owner = roles.iter().any(|r| r["name"] == "Owner");

    assert!(has_owner, "Owner Role Should be Created Automatically");
}

#[sqlx::test]
async fn test_owner_has_all_permissions(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "owner@test.com").await;

    let (_, org_body) = common::request_json_auth(
        app.clone(),
        json!({
            "name":"Acme Inc"
        }),
        "POST",
        "/organization",
        &token,
    )
    .await;

    let org_id = org_body["id"].as_str().unwrap();

    let (_, allperms) = common::get_json(app.clone(), "/permission", Some(&token)).await;

    let total_permission = allperms["data"].as_array().unwrap().len();

    let (_, roles_body) = common::get_json(
        app.clone(),
        &format!("/organization/{}/role", org_id),
        Some(&token),
    )
    .await;

    let owner_role = roles_body
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["name"] == "Owner")
        .unwrap()
        .clone();

    let owner_role_id = owner_role["id"].as_str().unwrap();

    let (status, perms_body) = common::get_json(
        app,
        &format!("/organization/{}/role/{}/permission", org_id, owner_role_id),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);

    let owner_permission = perms_body.as_array().unwrap().len();

    assert_eq!(
        owner_permission, total_permission,
        "Owner Role Should have All Permission"
    );
}

// Get Organization -------

#[sqlx::test]
async fn test_list_org_returns_only_user_org(pool: PgPool) {
    let app = common::build_app(pool);

    let token1 = common::register_and_login(app.clone(), "user1@test.com").await;

    common::request_json_auth(
        app.clone(),
        json!({"name":"ACME INC 1"}),
        "POST",
        "/organization",
        &token1,
    )
    .await;

    let token2 = common::register_and_login(app.clone(), "user2@test.com").await;

    common::request_json_auth(
        app.clone(),
        json!({"name":"ACME INC 2"}),
        "POST",
        "/organization",
        &token2,
    )
    .await;

    let (status, body) = common::get_json(app, "/organization", Some(&token1)).await;

    assert_eq!(status, StatusCode::OK);

    let orgs = body["data"].as_array().unwrap();
    assert_eq!(orgs.len(), 1);
    assert_eq!(orgs[0]["name"], "ACME INC 1");
}

// Update Organization ------
#[sqlx::test]
async fn test_update_org_name_success(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "update@test.com").await;

    let (_, org_body) = common::request_json_auth(
        app.clone(),
        json!({"name":"Acme Inc"}),
        "POST",
        "/organization",
        &token,
    )
    .await;

    let org_id = org_body["id"].as_str().unwrap();

    let (status, _) = common::request_json_auth(
        app,
        json!({"name":"New Name"}),
        "PATCH",
        &format!("/organization/{}", org_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::NO_CONTENT);
}


#[sqlx::test]
async fn test_update_org_without_permission_fails(pool: PgPool){
    let app = common::build_app(pool);

    let token1 = register_and_login(app.clone(), "user1@test.com").await;

    let (_,org_body) = common::request_json_auth(app.clone(), json!({"name":"Company 1"}), "POST", "/organization", &token1).await;

    let org_id = org_body["id"].as_str().unwrap();

    let token2 = register_and_login(app.clone(), "user2@test.com").await;

    let (status,_) = common::request_json_auth(app, json!({"name":"Company 2"}), "PATCH", &format!("/organization/{}",org_id), &token2).await;

    assert_eq!(status,StatusCode::FORBIDDEN);
}