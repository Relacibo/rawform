use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::{auth::authenticate_client, serde_util::deserialize_maybe};
use crate::{
    db::forms::{self, FormPatch},
    error::AppError,
    models::Form,
};

#[derive(Deserialize)]
pub struct FormPath {
    client_name: String,
    external_id: String,
}

#[derive(Deserialize)]
pub struct PutFormBody {
    pub data: Value,
    #[serde(default, deserialize_with = "deserialize_maybe")]
    pub webhook_url: Option<Option<String>>,
}

#[derive(Deserialize)]
pub struct PatchFormBody {
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
        "admin_token": v.admin_token,
        "submit_token": v.submit_token,
        "created_at": v.created_at,
        "updated_at": v.updated_at,
    }))
}

pub async fn put_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath {
        client_name,
        external_id,
    }): Path<FormPath>,
    Json(body): Json<PutFormBody>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;
    let data = serde_json::to_string(&body.data).unwrap();
    let webhook_url = body.webhook_url.flatten();
    let admin_token = Uuid::new_v4().to_string();
    let submit_token = Uuid::new_v4().to_string();
    let view = forms::upsert(
        &pool,
        client.id,
        &external_id,
        &data,
        &admin_token,
        &submit_token,
        webhook_url.as_deref(),
    )
    .await?;
    Ok((StatusCode::OK, Json(form_json(&view)?)))
}

pub async fn get_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath {
        client_name,
        external_id,
    }): Path<FormPath>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;
    let view = forms::find_by_client_and_external(&pool, client.id, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(form_json(&view)?))
}

pub async fn patch_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath {
        client_name,
        external_id,
    }): Path<FormPath>,
    Json(body): Json<PatchFormBody>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;
    let view = forms::find_by_client_and_external(&pool, client.id, &external_id)
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

pub async fn delete_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath {
        client_name,
        external_id,
    }): Path<FormPath>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;
    let view = forms::find_by_client_and_external(&pool, client.id, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;
    forms::delete(&pool, view.id, client.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
