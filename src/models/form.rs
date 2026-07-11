use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Form {
    pub id: i64,
    pub data: String, // stored as JSON string; parse to Value when needed
    pub is_active: bool,
    pub client_id: i64,
    pub external_id: String,
    pub admin_token: String,
    pub submit_token: String,
    pub webhook_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Form {
    pub fn data_json(&self) -> anyhow::Result<Value> {
        Ok(serde_json::from_str::<Value>(&self.data)?)
    }
}

/// Shape of the `data` field — the actual form definition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormDefinition {
    pub title: Option<String>,
    pub elements: Vec<FormElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormElement {
    Text(TextElement),
    Textarea(TextareaElement),
    Dropdown(DropdownElement),
    Checkbox(CheckboxElement),
    // TODO: add more element types
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    pub label: String,
    pub name: String, // derived from label, overridable
    pub required: bool,
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextareaElement {
    pub label: String,
    pub name: String,
    pub required: bool,
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownElement {
    pub label: String,
    pub name: String,
    pub required: bool,
    pub options: Vec<DropdownOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownOption {
    pub label: String,
    pub value: String, // derived from label, overridable
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxElement {
    pub label: String,
    pub name: String,
    pub required: bool,
}
