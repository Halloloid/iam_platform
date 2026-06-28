use serde::{self, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct MemberRole{
    pub user_id : Uuid,
    pub org_id : Uuid,
    pub role_id : Uuid
}