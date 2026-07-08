use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{config::response_config::AppError, models::organization::Organization};

pub async fn create_organization(
    user_id: Uuid,
    name: String,
    pool: &Pool<Postgres>,
) -> Result<Uuid, AppError> {
    let mut transaction = pool.begin().await.map_err(|_| AppError::Database)?;

    let org = sqlx::query!(
        "INSERT INTO organizations (name,created_by) VALUES ($1,$2) RETURNING id",
        name,
        user_id
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    sqlx::query!(
        "INSERT INTO membership (user_id,org_id) VALUES ($1,$2)",
        user_id,
        org.id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    let role = sqlx::query!(
        "INSERT INTO roles (name,org_id) VALUES ('Owner',$1) RETURNING id",
        org.id
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    sqlx::query!(
        "INSERT INTO role_permissions (role_id,permission_id)
         SELECT $1,id FROM permissions",
        role.id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    sqlx::query!(
        "INSERT INTO member_roles (user_id,org_id,role_id) VALUES ($1,$2,$3)",
        user_id,
        org.id,
        role.id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| AppError::Database)?;

    transaction.commit().await.map_err(|_| AppError::Database)?;

    Ok(org.id)
}

pub async fn all_organizations_asc(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    cursor: Option<DateTime<Utc>>,
    limit: i64,
) -> Result<Vec<Organization>, AppError> {
    let data = sqlx::query_as!(
        Organization,
        "SELECT o.id,o.name,o.created_at FROM organizations o 
        INNER JOIN membership m 
        ON m.org_id = o.id 
        WHERE m.user_id = $1 
        AND o.is_deleted = false 
        AND ($2::timestamptz IS NULL OR o.created_at > $2) 
        ORDER BY o.created_at ASC
        LIMIT $3",
        user_id,
        cursor,
        limit
    )
    .fetch_all(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(data)
}
pub async fn all_organizations_desc(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    cursor: Option<DateTime<Utc>>,
    limit: i64,
) -> Result<Vec<Organization>, AppError> {
    let data = sqlx::query_as!(
        Organization,
        "SELECT o.id,o.name,o.created_at FROM organizations o 
        INNER JOIN membership m 
        ON m.org_id = o.id 
        WHERE m.user_id = $1 
        AND o.is_deleted = false 
        AND ($2::timestamptz IS NULL OR o.created_at > $2) 
        ORDER BY o.created_at DESC
        LIMIT $3",
        user_id,
        cursor,
        limit
    )
    .fetch_all(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(data)
}

pub async fn one_org(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    org_id: Uuid,
) -> Result<Organization, AppError> {
    let org = sqlx::query_as!(
        Organization,
        "SELECT o.id,o.name,o.created_at FROM organizations o 
        INNER JOIN membership m ON m.org_id = o.id
        WHERE m.user_id = $1 AND
        o.id = $2 AND
        o.is_deleted = false",
        user_id,
        org_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| AppError::Database)?;

    if let Some(o) = org {
        Ok(o)
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn check_permission(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    org_id: Uuid,
    permission: &str,
) -> Result<bool, AppError> {
    let res = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM member_roles mr
        INNER JOIN role_permissions rp ON rp.role_id = mr.role_id
        INNER JOIN permissions p ON p.id = rp.permission_id
        WHERE mr.user_id = $1 AND 
        mr.org_id = $2 AND
        p.name = $3",
        user_id,
        org_id,
        permission
    )
    .fetch_one(pool)
    .await
    .map_err(|_| AppError::Database)?;

    Ok(res.count.unwrap_or(0) > 0)
}

pub async fn update_org_name(
    org_id: Uuid,
    name: String,
    pool: &Pool<Postgres>,
) -> Result<(), AppError> {
    let res = sqlx::query!(
        "UPDATE organizations SET name = $1 WHERE id = $2 AND is_deleted = false",
        name,
        org_id
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::Database)?;

    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}
