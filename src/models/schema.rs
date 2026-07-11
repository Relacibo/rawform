use serde::{Deserialize, Serialize};

/// The JSON structure stored in forms.data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormSchema {
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
    // TODO: number, date, email, radio, file
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    pub label: String,
    pub name: String,
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
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxElement {
    pub label: String,
    pub name: String,
    pub required: bool,
}
