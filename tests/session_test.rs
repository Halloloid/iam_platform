use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

// List Session ---------------------
#[sqlx::test]
async fn test_list_session_success(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "session@test.com").await;

    let (status, body) = common::get_json(app, "/session", Some(&token)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert!(body["data"].as_array().unwrap().len() > 0);
}

#[sqlx::test]
async fn test_list_session_without_token_fails(pool: PgPool) {
    let app = common::build_app(pool);

    let (status, _) = common::get_json(app, "/session", None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_current_session_marked_correct(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "current_session@test.com").await;

    let body = common::get_session(app.clone(), &token).await;

    let sessions = body["data"].as_array().unwrap();

    let current_count = sessions
        .iter()
        .filter(|x| x["is_current"].as_bool().unwrap_or(false))
        .count();

    assert_eq!(current_count, 1);
}

#[sqlx::test]
async fn test_same_device_login_updates_session(pool: PgPool) {
    let app = common::build_app(pool);

    common::request_json_no_auth(
        app.clone(),
        "POST",
        "/auth/register",
        json!({
            "email":"multi_session@tets.com",
            "password":"1223Abbc",
            "name":"Test User"
        }),
    )
    .await;

    // will Login Twice in a row

    let (_, _) = common::request_json_no_auth(
        app.clone(),
        "POST",
        "/auth/login",
        json!({
            "email":"multi_session@tets.com",
            "password":"1223Abbc",
        }),
    )
    .await;

    let (_, body2) = common::request_json_no_auth(
        app.clone(),
        "POST",
        "/auth/login",
        json!({
            "email":"multi_session@tets.com",
            "password":"1223Abbc",
        }),
    )
    .await;

    let token = body2["access_token"].as_str().unwrap();

    let body = common::get_session(app, token).await;

    let sessions = body["data"].as_array().unwrap();

    assert_eq!(sessions.len(), 1);
}

#[sqlx::test]
async fn test_api_key_cannot_access_sessions(pool: PgPool) {
    let (app, token, org_id) = common::setup_org(pool, "api_key_fails@test.com").await;

    let (_, perms_body) = common::get_json(app.clone(), "/permission", Some(&token)).await;

    let perm_id = perms_body["data"].as_array().unwrap().first().unwrap()["id"]
        .as_str()
        .unwrap();

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

    let key = body["raw_key"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/session")
                .header("Authorization", format!("Bearer {}", key))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
