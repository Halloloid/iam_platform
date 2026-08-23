use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;


mod common;

// helper for creating org and and returning token
async fn setup_org(pool:PgPool,email:&str) -> (axum::Router,String,String) {
    let app = common::build_app(pool);
    let token = common::register_and_login(app.clone(), email).await;

    let (_,org_body) = common::request_json_auth(app.clone(), json!({"name":"Test Org"}), "POST","/organization", &token).await;

    let org_id = org_body["id"].as_str().unwrap().to_string();

    (app,token,org_id)
}

// Create Role Tests
#[sqlx::test]
async fn test_create_role(pool:PgPool){
    let (app,token,org_id) = setup_org(pool, "create_role@test.com").await;

    let (status,body) = common::request_json_auth(app, json!({"name":"Manager"}), "POST", &format!("/organization/{}/role",org_id), &token).await;

    assert_eq!(status,StatusCode::CREATED);
    assert!(body["message"].is_string());
}

#[sqlx::test]
async fn test_create_duplicate_role_fails(pool:PgPool){
    let (app,token,org_id) = setup_org(pool, "dup@test.com").await;

    common::request_json_auth(app.clone(), json!({"name":"Manager"}), "POST", &format!("/organization/{}/role",org_id), &token).await;

    let (status,_) = common::request_json_auth(app, json!({"name":"Manager"}), "POST", &format!("/organization/{}/role",org_id), &token).await;

    assert_eq!(status,StatusCode::CONFLICT);
}

#[sqlx::test]
async fn test_create_owner_role_reserved_fails(pool:PgPool){
    let (app,token,org_id) = setup_org(pool, "reserved@test.com").await;

    let (status,_) = common::request_json_auth(app, json!({"name":"owner"}), "POST", &format!("/organization/{}/role",org_id), &token).await;

    assert_eq!(status,StatusCode::CONFLICT);
}

#[sqlx::test]
async fn test_create_role_without_permission_fails(pool:PgPool){
    let (app,_,org_id) = setup_org(pool, "no_perm@test.com").await;

    let token2 = common::register_and_login(app.clone(), "interuder@test.com").await;

    let (status,_) = common::request_json_auth(app, json!({"name":"Hacker role"}), "POST",&format!("/organization/{}/role",org_id), &token2).await;

    assert_eq!(status,StatusCode::FORBIDDEN)
}

// Get Roles -------------------
