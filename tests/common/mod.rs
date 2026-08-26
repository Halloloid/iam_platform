use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

// App with test pool
pub fn build_app(pool: PgPool) -> Router {
    iam_platform::routes::main_router::main_router(pool)
}

pub async fn request_json_no_auth(
    app: Router,
    method: &str,
    path: &str,
    body: Value,
) -> (StatusCode, Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("Content-Type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);

    (status, json)
}

#[allow(dead_code)]
pub async fn get_json(app: Router, path: &str, token: Option<&str>) -> (StatusCode, Value) {
    let mut builder = Request::builder().method("GET").uri(path);

    if let Some(t) = token {
        builder = builder.header("Authorization", format!("Bearer {}", t));
    }

    let response = app
        .oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);

    (status, json)
}

// registering a user and returns a token - used in many tests
pub async fn register_and_login(app: Router, email: &str) -> String {
    request_json_no_auth(
        app.clone(),
        "POST",
        "/auth/register",
        json!({
            "email":email,
            "password":"password123",
            "name":"Test User"
        }),
    )
    .await;

    let (_, body) = request_json_no_auth(
        app.clone(),
        "POST",
        "/auth/login",
        json!({
            "email":email,
            "password":"password123"
        }),
    )
    .await;

    body["access_token"].as_str().unwrap().to_string()
}

#[allow(dead_code)]
pub async fn request_json_auth(
    app: Router,
    body: Value,
    method: &str,
    path: &str,
    token: &str,
) -> (StatusCode, Value) {
    let reponse = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = reponse.status();
    let bytes = axum::body::to_bytes(reponse.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);

    (status, json)
}

#[allow(dead_code)]
pub async fn setup_org(pool: PgPool, email: &str) -> (axum::Router, String, String) {
    let app = build_app(pool);
    let token = register_and_login(app.clone(), email).await;

    let (_, org_body) = request_json_auth(
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

#[allow(dead_code)]
pub async fn register_user(app: Router, email: &str) -> (String, String) {
    let token = register_and_login(app.clone(), email).await;

    let (_, me_body) = get_json(app, "/user/me", Some(&token)).await;

    let user_id = me_body["id"].as_str().unwrap().to_string();

    (token, user_id)
}
