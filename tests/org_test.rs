use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

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
