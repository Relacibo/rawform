use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub api_key_hash: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}
