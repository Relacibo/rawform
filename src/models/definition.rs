use serde::{Deserialize, Serialize};

/// DB row for the form_definitions table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Definition {
    pub id: i64,
    pub client_id: i64,
    pub data: String,
    pub created_at: String,
}
