use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Role {
    pub name: String,
}
