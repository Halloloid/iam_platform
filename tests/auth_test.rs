use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;
mod common;

// Register Tests ----------

#[sqlx::test]
async fn test_register_success(pool: PgPool) {
    let app = common::build_app(pool);

    let (status, _) = common::request_json_no_auth(
        app,
        "POST",
        "/auth/register",
        json!({
            "email":"test@123.com",
            "password":"password123",
            "name":"Test User"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
}

#[sqlx::test]
async fn test_register_duplicate_emails_fails(pool: PgPool) {
    let app = common::build_app(pool);

    let payload = json!({
        "email":"duplicat@123.com",
        "password":"password@123",
        "name":"Test User"
    });

    common::request_json_no_auth(app.clone(), "POST", "/auth/register", payload.clone()).await;

    let (status, _) =
        common::request_json_no_auth(app, "POST", "/auth/register", payload.clone()).await;

    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test]
async fn test_register_invalid_email(pool: PgPool) {
    let app = common::build_app(pool);

    let (status, _) = common::request_json_no_auth(
        app,
        "POST",
        "/auth/register",
        json!({
            "email":"notavalidemail",
            "password":"password@123",
            "name":"Test User"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// Login -----
#[sqlx::test]
async fn test_login_success(pool: PgPool) {
    let app = common::build_app(pool);

    common::request_json_no_auth(
        app.clone(),
        "POST",
        "/auth/register",
        json!({
            "email":"login@test.com",
            "password":"password123",
            "name":"Test User"
        }),
    )
    .await;

    let (status, body) = common::request_json_no_auth(
        app,
        "POST",
        "/auth/login",
        json!({
            "email":"login@test.com",
            "password":"password123"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());
}

#[sqlx::test]
async fn test_login_wrong_password_fails(pool: PgPool) {
    let app = common::build_app(pool);

    common::request_json_no_auth(
        app.clone(),
        "POST",
        "/auth/register",
        json!({
            "email":"login@test.com",
            "password":"password123",
            "name":"Test User"
        }),
    )
    .await;

    let (status, _) = common::request_json_no_auth(
        app,
        "POST",
        "/auth/login",
        json!({
            "email":"login@test.com",
            "password":"wrongpassword123"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_login_non_existent_user_fails(pool: PgPool) {
    let app = common::build_app(pool);

    let (status, _) = common::request_json_no_auth(
        app,
        "POST",
        "/auth/login",
        json!({
            "email":"nobody@test.com",
            "password":"password123"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// Protected Routes--------

#[sqlx::test]
async fn test_get_me_without_token_fails(pool: PgPool) {
    let app = common::build_app(pool);

    let (status, _) = common::get_json(app, "/user/me", None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_get_me_with_valid_token_succeeds(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "getme@test.com").await;

    let (status, body) = common::get_json(app, "/user/me", Some(&token)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["email"], "getme@test.com");
}
