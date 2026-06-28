use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde::{self, Serialize};

#[derive(Debug,Clone,FromRow,Serialize)]
pub struct Membership{
    pub user_id : Uuid,
    pub org_id : Uuid,
    pub joined_at : DateTime<Utc>
}