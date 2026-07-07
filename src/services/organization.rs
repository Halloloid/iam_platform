use sqlx::{Pool, Postgres};

use crate::{
    config::response_config::AppError, models::organization::Organization,
    repositories::organization::all_organizations,
};

pub async fn all_org_service(pool: &Pool<Postgres>) -> Result<Vec<Organization>, AppError> {
    let data = all_organizations(pool).await?;

    Ok(data)
}
