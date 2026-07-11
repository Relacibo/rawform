use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

use crate::{
    db::{definitions, instances::{self, InstancePatch}},
    error::AppError,
    models::InstanceView,
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

fn instance_json(v: &InstanceView) -> Result<Value, AppError> {
    Ok(json!({
        "id": v.id,
        "external_id": v.external_id,
        "definition_id": v.definition_id,
        "data": v.data_json()?,
        "is_active": v.is_active,
        "webhook_url": v.webhook_url,
        "submit_token": v.submit_token,
        "created_at": v.created_at,
        "updated_at": v.updated_at,
    }))
}

pub async fn get_form(
    State(pool): State<SqlitePool>,
    Path(admin_token): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let view = instances::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(instance_json(&view)?))
}

pub async fn put_form(
    State(pool): State<SqlitePool>,
    Path(admin_token): Path<String>,
    Json(body): Json<PutAdminBody>,
) -> Result<impl IntoResponse, AppError> {
    let view = instances::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;
    let data = serde_json::to_string(&body.data).unwrap();
    let def = definitions::insert(&pool, view.client_id, &data).await?;
    let updated = instances::patch(&pool, view.id, InstancePatch {
        definition_id: Some(def.id),
        webhook_url: body.webhook_url,
        is_active: None,
    }).await?;
    Ok(Json(instance_json(&updated)?))
}

pub async fn patch_form(
    State(pool): State<SqlitePool>,
    Path(admin_token): Path<String>,
    Json(body): Json<PatchAdminBody>,
) -> Result<impl IntoResponse, AppError> {
    let view = instances::find_by_admin_token(&pool, &admin_token)
        .await?
        .ok_or(AppError::NotFound)?;

    let definition_id = if let Some(data_val) = &body.data {
        let data = serde_json::to_string(data_val).unwrap();
        let def = definitions::insert(&pool, view.client_id, &data).await?;
        Some(def.id)
    } else {
        None
    };

    let updated = instances::patch(&pool, view.id, InstancePatch {
        definition_id,
        webhook_url: body.webhook_url,
        is_active: body.is_active,
    }).await?;
    Ok(Json(instance_json(&updated)?))
}
