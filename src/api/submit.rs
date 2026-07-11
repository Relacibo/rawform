use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

use crate::{db::forms, error::AppError};

#[derive(Deserialize)]
pub struct SubmitBody {
    pub values: Value, // { field_name: value, ... }
}

pub async fn post_submit(
    State(pool): State<SqlitePool>,
    Path(submit_token): Path<String>,
    Json(_body): Json<SubmitBody>,
) -> Result<impl IntoResponse, AppError> {
    let form = forms::find_by_submit_token(&pool, &submit_token)
        .await?
        .ok_or(AppError::NotFound)?;

    // TODO: validate submitted values against form definition
    // TODO: store submission in a submissions table
    // TODO: fire webhook if form.webhook_url is set

    Ok((StatusCode::ACCEPTED, Json(json!({ "ok": true, "form_id": form.id }))))
}
