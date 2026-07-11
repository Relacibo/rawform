use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Submission {
    pub id: i64,
    pub form_id: i64,
    pub values: String, // JSON string; parse to Value when needed
    pub submitted_at: String,
}
