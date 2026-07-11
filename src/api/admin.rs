use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

use crate::{db::forms, error::AppError};

#[derive(Deserialize)]
pub struct PutAdminBody {
    pub data: Value,
    pub webhook_url: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchAdminBody {
    pub data: Option<Value>,
    pub webhook_url: Option<String>,
    pub is_active: Option<bool>,
}

pub async fn get_form(
    State(pool): State<SqlitePool>,
    Path(admin_token): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let form = forms::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(json!({
        "id": form.id,
        "external_id": form.external_id,
        "data": form.data_json()?,
        "is_active": form.is_active,
        "webhook_url": form.webhook_url,
        "submit_token": form.submit_token,
    })))
}

pub async fn put_form(
    State(pool): State<SqlitePool>,
    Path(admin_token): Path<String>,
    Json(body): Json<PutAdminBody>,
) -> Result<impl IntoResponse, AppError> {
    let form = forms::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    let data = serde_json::to_string(&body.data).unwrap();
    let updated = forms::replace(&pool, form.id, &data, body.webhook_url.as_deref()).await?;
    Ok(Json(json!({ "id": updated.id })))
}

pub async fn patch_form(
    State(pool): State<SqlitePool>,
    Path(admin_token): Path<String>,
    Json(body): Json<PatchAdminBody>,
) -> Result<impl IntoResponse, AppError> {
    let form = forms::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    // TODO: apply partial updates
    let _ = body;
    Ok(Json(json!({ "id": form.id })))
}
