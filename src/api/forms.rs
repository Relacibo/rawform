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
    db::forms::{self, FormPatch},
    error::AppError,
    models::Form,
};
use super::{auth::authenticate_client, serde_util::deserialize_maybe};

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

fn form_json(form: &Form) -> Result<Value, AppError> {
    Ok(json!({
        "id": form.id,
        "external_id": form.external_id,
        "data": form.data_json()?,
        "is_active": form.is_active,
        "webhook_url": form.webhook_url,
        "admin_token": form.admin_token,
        "submit_token": form.submit_token,
        "created_at": form.created_at,
        "updated_at": form.updated_at,
    }))
}

pub async fn put_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath { client_name, external_id }): Path<FormPath>,
    Json(body): Json<PutFormBody>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;
    let data = serde_json::to_string(&body.data).unwrap();
    let webhook_url = body.webhook_url.flatten();
    let admin_token = Uuid::new_v4().to_string();
    let submit_token = Uuid::new_v4().to_string();
    let form = forms::upsert(
        &pool,
        client.id,
        &external_id,
        &data,
        &admin_token,
        &submit_token,
        webhook_url.as_deref(),
    )
    .await?;
    Ok((StatusCode::OK, Json(form_json(&form)?)))
}

pub async fn get_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath { client_name, external_id }): Path<FormPath>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;
    let form = forms::find_by_client_and_external(&pool, client.id, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(form_json(&form)?))
}

pub async fn patch_form(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(FormPath { client_name, external_id }): Path<FormPath>,
    Json(body): Json<PatchFormBody>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;
    let form = forms::find_by_client_and_external(&pool, client.id, &external_id)
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
