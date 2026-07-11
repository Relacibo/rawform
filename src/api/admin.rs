use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::{Value, json};

use super::serde_util::deserialize_maybe;
use crate::{
    db::DbPool,
    db::forms::{self, FormPatch},
    error::AppError,
    models::Form,
};

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

fn form_json(v: &Form) -> Result<Value, AppError> {
    Ok(json!({
        "id": v.id,
        "external_id": v.external_id,
        "data": v.data_json()?,
        "is_active": v.is_active,
        "webhook_url": v.webhook_url,
        "submit_token": v.submit_token,
        "created_at": v.created_at,
        "updated_at": v.updated_at,
    }))
}

pub async fn get_form(
    State(pool): State<DbPool>,
    Path(admin_token): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let view = forms::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(form_json(&view)?))
}

pub async fn put_form(
    State(pool): State<DbPool>,
    Path(admin_token): Path<String>,
    Json(body): Json<PutAdminBody>,
) -> Result<impl IntoResponse, AppError> {
    let view = forms::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    let data = serde_json::to_string(&body.data).unwrap();
    let updated = forms::patch(
        &pool,
        view.id,
        FormPatch {
            data: Some(data),
            webhook_url: body.webhook_url,
            is_active: None,
        },
    )
    .await?;
    Ok(Json(form_json(&updated)?))
}

pub async fn patch_form(
    State(pool): State<DbPool>,
    Path(admin_token): Path<String>,
    Json(body): Json<PatchAdminBody>,
) -> Result<impl IntoResponse, AppError> {
    let view = forms::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    let updated = forms::patch(
        &pool,
        view.id,
        FormPatch {
            data: body
                .data
                .as_ref()
                .map(|d| serde_json::to_string(d).unwrap()),
            webhook_url: body.webhook_url,
            is_active: body.is_active,
        },
    )
    .await?;
    Ok(Json(form_json(&updated)?))
}
