use axum::http::StatusCode;
use sqlx::PgPool;

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
