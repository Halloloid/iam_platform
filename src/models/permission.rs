use serde::{self, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
}
