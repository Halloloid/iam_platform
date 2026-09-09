use axum::http::StatusCode;
use sqlx::PgPool;
mod common;

// Personal Audit Logs ----------
#[sqlx::test]
async fn test_get_personal_logs(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "personal_logs@test.com").await;

    let (status, body) = common::get_json(app.clone(), "/audit-logs", Some(&token)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
}

#[sqlx::test]
async fn test_personal_logs_contain_login_event(pool: PgPool) {
    let app = common::build_app(pool);

    let token = common::register_and_login(app.clone(), "personal_logs@test.com").await;

    let (_, body) = common::get_json(app.clone(), "/audit-logs", Some(&token)).await;

    let logs = body["data"].as_array().unwrap();

    assert!(logs.len() >= 1);

    let has_login = logs.iter().any(|x| x["action"] == "user:login");

    assert!(has_login, "login event should be in audit logs");
}

#[sqlx::test]
async fn test_personal_logs_without_token_fails(pool: PgPool) {
    let app = common::build_app(pool);

    let (status, _) = common::get_json(app.clone(), "/audit-logs", None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_personal_logs_only_show_own_actions(pool: PgPool) {
    let app = common::build_app(pool);

    let token1 = common::register_and_login(app.clone(), "audit_test1@test.com").await;

    let token2 = common::register_and_login(app.clone(), "audit_test2@test.com").await;

    let (_, me_body) = common::get_json(app.clone(), "/user/me", Some(&token2)).await;

    let user2_id = me_body["id"].as_str().unwrap();

    let (_, body) = common::get_json(app.clone(), "/audit-logs", Some(&token1)).await;

    let logs = body["data"].as_array().unwrap();

    let has_user2_id = logs.iter().any(|x| x["actor_id"] == user2_id);

    assert!(!has_user2_id, "user1 should not see user2 logs");
}
