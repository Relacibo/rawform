use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

use crate::{
    db::forms::{self, FormPatch},
    error::AppError,
    models::Form,
};
use super::serde_util::deserialize_maybe;

#[derive(Deserialize)]
pub struct PutAdminBody {
    pub data: Value,
    #[serde(default, deserialize_with = "deserialize_maybe")]
    pub webhook_url: Option<Option<String>>,
}

#[derive(Deserialize)]
pub struct PatchAdminBody {
    pub data: Option<Value>,
    #[serde(default, deserialize_with = "deserialize_maybe")]
    pub webhook_url: Option<Option<String>>,
    pub is_active: Option<bool>,
}

fn form_json(form: &Form) -> Result<Value, AppError> {
    Ok(json!({
        "id": form.id,
        "external_id": form.external_id,
        "data": form.data_json()?,
        "is_active": form.is_active,
        "webhook_url": form.webhook_url,
        "submit_token": form.submit_token,
        "created_at": form.created_at,
        "updated_at": form.updated_at,
    }))
}

pub async fn get_form(
    State(pool): State<SqlitePool>,
    Path(admin_token): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let form = forms::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(form_json(&form)?))
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
    let webhook_url = body.webhook_url.flatten();
    let updated = forms::replace(&pool, form.id, &data, webhook_url.as_deref()).await?;
    Ok(Json(form_json(&updated)?))
}

pub async fn patch_form(
    State(pool): State<SqlitePool>,
    Path(admin_token): Path<String>,
    Json(body): Json<PatchAdminBody>,
) -> Result<impl IntoResponse, AppError> {
    let form = forms::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    let updated = forms::patch(
        &pool,
        form.id,
        FormPatch {
            data: body.data.as_ref().map(|v| serde_json::to_string(v).unwrap()),
            webhook_url: body.webhook_url,
            is_active: body.is_active,
        },
    )
    .await?;
    Ok(Json(form_json(&updated)?))
}
