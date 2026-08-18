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

pub async fn post_json(app: Router, path: &str, body: Value) -> (StatusCode, Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
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
    post_json(
        app.clone(),
        "/auth/register",
        json!({
            "email":email,
            "password":"password123",
            "name":"Test User"
        }),
    )
    .await;

    let (_, body) = post_json(
        app.clone(),
        "/auth/login",
        json!({
            "email":email,
            "password":"password123"
        }),
    )
    .await;

    body["access_token"].as_str().unwrap().to_string()
}
