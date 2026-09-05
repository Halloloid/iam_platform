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

#[allow(dead_code)]
pub async fn get_first_permission_id(app: axum::Router, token: &str) -> String {
    let (_, perms_body) = get_json(app, "/permission", Some(token)).await;

    perms_body["data"].as_array().unwrap().first().unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string()
}

#[allow(dead_code)]
pub async fn create_api_key(
    app: Router,
    org_id: &str,
    token: &str,
    perm_id: &str,
) -> (String, String) {
    let (_, body) = request_json_auth(
        app,
        json!({
            "name":"Test Key",
            "permission_ids":[perm_id],
            "expires_in_dayes":1
        }),
        "POST",
        &format!("/organization/{}/api_key", org_id),
        token,
    )
    .await;

    let key_id = body["id"].as_str().unwrap().to_string();
    let raw_key = body["raw_key"].as_str().unwrap().to_string();

    (key_id, raw_key)
}

#[allow(dead_code)]
pub async fn get_session(app: axum::Router, token: &str) -> serde_json::Value {
    let (_, body) = get_json(app, "/session", Some(token)).await;

    body
}
