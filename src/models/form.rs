use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Form {
    pub id: i64,
    pub client_id: i64,
    pub external_id: String,
    pub data: String,
    pub admin_token: String,
    pub submit_token: String,
    pub webhook_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Form {
    pub fn data_json(&self) -> anyhow::Result<Value> {
        serde_json::from_str::<Value>(&self.data).context("invalid form data JSON")
    }
}
