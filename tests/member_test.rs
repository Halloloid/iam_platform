use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

mod common;

//--Add Members---------
#[sqlx::test]
async fn test_add_memeber_success(pool: PgPool) {
    let (app, owner_token, org_id) = common::setup_org(pool, "owner@test.com").await;

    common::register_user(app.clone(), "member@test.com").await;

    let (status, _) = common::request_json_auth(
        app,
        json!({"email":"member@test.com"}),
        "POST",
        &format!("/organization/{}/member", org_id),
        &owner_token,
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
}

#[sqlx::test]
async fn test_add_already_existing_member_fails(pool: PgPool) {
    let (app, owner_token, org_id) = common::setup_org(pool, "owner@test.com").await;

    common::register_user(app.clone(), "new_member@test.com").await;

    // adding first time
    common::request_json_auth(
        app.clone(),
        json!({"email":"new_member@test.com"}),
        "POST",
        &format!("/organization/{}/member", org_id),
        &owner_token,
    )
    .await;

    //addind Second Time
    let (status, _) = common::request_json_auth(
        app,
        json!({"email":"new_member@test.com"}),
        "POST",
        &format!("/organization/{}/member", org_id),
        &owner_token,
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test]
async fn test_add_member_without_permission_fails(pool: PgPool) {
    let (app, _owner_token, org_id) = common::setup_org(pool, "owner@test.com").await;

    let (token2, _) = common::register_user(app.clone(), "no_perm_user@test.com").await;

    let (_, _) = common::register_user(app.clone(), "target_user@test.com").await;

    let (status, _) = common::request_json_auth(
        app,
        json!({"email":"target_user@test.com"}),
        "POST",
        &format!("/organization/{}/member", org_id),
        &token2,
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

// Get Members Tests ---------------
#[sqlx::test]
async fn test_list_members_success(pool: PgPool) {
    let (app, owner_token, org_id) = common::setup_org(pool, "owner_member@test.com").await;

    common::register_user(app.clone(), "user_member@test.com").await;

    common::request_json_auth(
        app.clone(),
        json!({"email":"user_member@test.com"}),
        "POST",
        &format!("/organization/{}/member", org_id),
        &owner_token,
    )
    .await;

    let (status, body) = common::get_json(
        app,
        &format!("/organization/{}/member", org_id),
        Some(&owner_token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);

    assert_eq!(body["data"].as_array().unwrap().len(), 2);
}

#[sqlx::test]
async fn test_list_non_member_fails(pool: PgPool) {
    let (app, _, org_id) = common::setup_org(pool, "owner@test.com").await;

    let (token2, _) = common::register_user(app.clone(), "non_member@test.com").await;

    let (status, _) = common::get_json(
        app,
        &format!("/organization/{}/member", org_id),
        Some(&token2),
    )
    .await;
    
    assert_eq!(status, StatusCode::FORBIDDEN);
}

// Remove Member