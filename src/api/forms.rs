use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    db::{clients, forms},
    error::AppError,
};

fn extract_bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

fn hash_key(key: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

#[derive(Deserialize)]
pub struct FormPath {
    client_name: String,
    external_id: String,
}

#[derive(Deserialize)]
pub struct PutFormBody {
    pub data: Value,
    pub webhook_url: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchFormBody {
    pub data: Option<Value>,
    pub webhook_url: Option<String>,
    pub is_active: Option<bool>,
}

async fn authenticate(
    pool: &SqlitePool,
    headers: &HeaderMap,
    client_name: &str,
) -> Result<crate::models::Client, AppError> {
    let key = extract_bearer(headers).ok_or(AppError::Unauthorized)?;
    let client = clients::find_by_name(pool, client_name)
        .await?
        .ok_or(AppError::Unauthorized)?;
    if client.api_key_hash != hash_key(&key) {
        return Err(AppError::Unauthorized);
    }
    Ok(client)
}

pub async fn put_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath { client_name, external_id }): Path<FormPath>,
    Json(body): Json<PutFormBody>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate(&pool, &headers, &client_name).await?;
    let data = serde_json::to_string(&body.data).unwrap();
    let admin_token = Uuid::new_v4().to_string();
    let submit_token = Uuid::new_v4().to_string();
    let form = forms::upsert(
        &pool,
        client.id,
        &external_id,
        &data,
        &admin_token,
        &submit_token,
        body.webhook_url.as_deref(),
    )
    .await?;
    Ok((StatusCode::OK, Json(json!({
        "id": form.id,
        "external_id": form.external_id,
        "admin_token": form.admin_token,
        "submit_token": form.submit_token,
        "data": body.data,
    }))))
}

pub async fn get_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath { client_name, external_id }): Path<FormPath>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate(&pool, &headers, &client_name).await?;
    let form = forms::find_by_client_and_external(&pool, client.id, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(json!({
        "id": form.id,
        "external_id": form.external_id,
        "data": form.data_json()?,
        "is_active": form.is_active,
        "webhook_url": form.webhook_url,
        "admin_token": form.admin_token,
        "submit_token": form.submit_token,
    })))
}

pub async fn patch_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath { client_name, external_id }): Path<FormPath>,
    Json(body): Json<PatchFormBody>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate(&pool, &headers, &client_name).await?;
    let form = forms::find_by_client_and_external(&pool, client.id, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // TODO: apply partial updates (data, webhook_url, is_active)
    let _ = body;

    Ok(Json(json!({ "id": form.id })))
}
