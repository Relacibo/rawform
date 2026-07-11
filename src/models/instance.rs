use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// DB row for the form_instances table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Instance {
    pub id: i64,
    pub client_id: i64,
    pub external_id: String,
    pub definition_id: i64,
    pub admin_token: String,
    pub submit_token: String,
    pub webhook_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Instance joined with its current definition's data.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InstanceView {
    pub id: i64,
    pub client_id: i64,
    pub external_id: String,
    pub definition_id: i64,
    pub admin_token: String,
    pub submit_token: String,
    pub webhook_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub data: String,
}

impl InstanceView {
    pub fn data_json(&self) -> anyhow::Result<Value> {
        serde_json::from_str::<Value>(&self.data).context("invalid form data JSON")
    }
}
