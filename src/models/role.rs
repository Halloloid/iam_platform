use serde::{self, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RoleCreation {
    pub name: String,
}

#[derive(Debug,Serialize)]
pub struct Role{
    pub id : Uuid,
    pub name:String
}
