use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

mod common;

// helper for creating org and and returning token
async fn setup_org(pool: PgPool, email: &str) -> (axum::Router, String, String) {
    let app = common::build_app(pool);
    let token = common::register_and_login(app.clone(), email).await;

    let (_, org_body) = common::request_json_auth(
        app.clone(),
        json!({"name":"Test Org"}),
        "POST",
        "/organization",
        &token,
    )
    .await;

    let org_id = org_body["id"].as_str().unwrap().to_string();

    (app, token, org_id)
}

// Create Role Tests
#[sqlx::test]
async fn test_create_role(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "create_role@test.com").await;

    let (status, body) = common::request_json_auth(
        app,
        json!({"name":"Manager"}),
        "POST",
        &format!("/organization/{}/role", org_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["message"].is_string());
}

#[sqlx::test]
async fn test_create_duplicate_role_fails(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "dup@test.com").await;

    common::request_json_auth(
        app.clone(),
        json!({"name":"Manager"}),
        "POST",
        &format!("/organization/{}/role", org_id),
        &token,
    )
    .await;

    let (status, _) = common::request_json_auth(
        app,
        json!({"name":"Manager"}),
        "POST",
        &format!("/organization/{}/role", org_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test]
async fn test_create_owner_role_reserved_fails(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "reserved@test.com").await;

    let (status, _) = common::request_json_auth(
        app,
        json!({"name":"owner"}),
        "POST",
        &format!("/organization/{}/role", org_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test]
async fn test_create_role_without_permission_fails(pool: PgPool) {
    let (app, _, org_id) = setup_org(pool, "no_perm@test.com").await;

    let token2 = common::register_and_login(app.clone(), "interuder@test.com").await;

    let (status, _) = common::request_json_auth(
        app,
        json!({"name":"Hacker role"}),
        "POST",
        &format!("/organization/{}/role", org_id),
        &token2,
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN)
}

// Get Roles -------------------
#[sqlx::test]
async fn test_get_roles_include_owner(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "get_role@test.com").await;

    let (status, body) =
        common::get_json(app, &format!("/organization/{}/role", org_id), Some(&token)).await;

    assert_eq!(status, StatusCode::OK);

    let roles = body.as_array().unwrap();

    let has_owner = roles.iter().any(|r| r["name"] == "owner");
    assert!(has_owner);
}

// #[sqlx::test]
// async fn test_get_roles_non_member_fails(pool: PgPool) {
//     let (app, _target, org_id) = setup_org(pool, "get_roles_owner@test.com").await;

//     let token2 = common::register_and_login(app.clone(), "nonmemberrole@test.com").await;

//     let (status, _) = common::get_json(
//         app,
//         &format!("/organization/{}/role", org_id),
//         Some(&token2),
//     )
//     .await;

//     assert_eq!(status, StatusCode::FORBIDDEN);
// }

//Update Role-----------
#[sqlx::test]
async fn test_rename_role_success(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "rename_role@test.com").await;

    let (_, role_body) = common::request_json_auth(
        app.clone(),
        json!({"name":"Manager"}),
        "POST",
        &format!("/organization/{}/role", org_id),
        &token,
    )
    .await;

    let role_id = role_body["id"].as_str().unwrap();

    let (status, body) = common::request_json_auth(
        app,
        json!({"name":"Senior Manager"}),
        "PATCH",
        &format!("/organization/{}/role/{}", org_id, role_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["message"].is_string());
}

#[sqlx::test]
async fn test_rename_owner_role_fails(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "rename_owner@test.com").await;

    let (_, role_body) = common::get_json(
        app.clone(),
        &format!("/organization/{}/role", org_id),
        Some(&token),
    )
    .await;

    let owner_role_id = role_body
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["name"] == "owner")
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();

    let (status, _) = common::request_json_auth(
        app,
        json!({"name":"Super Leader"}),
        "PATCH",
        &format!("/organization/{}/role/{}", org_id, owner_role_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

//-- Delete Role---------
#[sqlx::test]
async fn test_delete_custom_role_success(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "delete_role@test.com").await;

    let (_, role_body) = common::request_json_auth(
        app.clone(),
        json!({"name":"TempRole"}),
        "POST",
        &format!("/organization/{}/role", org_id),
        &token,
    )
    .await;

    let role_id = role_body["id"].as_str().unwrap();

    let (status, body) = common::request_json_auth(
        app,
        json!({}),
        "DELETE",
        &format!("/organization/{}/role/{}", org_id, role_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["message"].is_string());
}

#[sqlx::test]
async fn test_delete_owner_role_fails(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "delete_owner@test.com").await;

    let (_, role_body) = common::get_json(
        app.clone(),
        &format!("/organization/{}/role", org_id),
        Some(&token),
    )
    .await;

    let owner_id = role_body
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["name"] == "owner")
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();

    let (status, _) = common::request_json_auth(
        app,
        json!({}),
        "DELETE",
        &format!("/organization/{}/role/{}", org_id, owner_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[sqlx::test]
async fn test_delete_role_in_use_fails(pool: PgPool) {
    let (app, token, org_id) = setup_org(pool, "role_in_use@test.com").await;

    let (_, role_body) = common::request_json_auth(
        app.clone(),
        json!({"name":"Viewer"}),
        "POST",
        &format!("/organization/{}/role", org_id),
        &token,
    )
    .await;

    let role_id = role_body["id"].as_str().unwrap().to_string();

    let token2 = common::register_and_login(app.clone(), "viewer_user@test.com").await;

    let (_, me_body) = common::get_json(app.clone(), "/user/me", Some(&token2)).await;

    let user2_id = me_body["id"].as_str().unwrap().to_string();

    // lets give user 2 memeber ship for our org using token
    common::request_json_auth(
        app.clone(),
        json!({"email":"viewer_user@test.com"}),
        "POST",
        &format!("/organization/{}/member", org_id),
        &token,
    )
    .await;

    common::request_json_auth(
        app.clone(),
        json!({"id":role_id}),
        "POST",
        &format!("/organization/{}/member/{}/role", org_id, user2_id),
        &token,
    )
    .await;

    let (status, _) = common::request_json_auth(
        app,
        json!({}),
        "DELETE",
        &format!("/organization/{}/role/{}", org_id, role_id),
        &token,
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
}
