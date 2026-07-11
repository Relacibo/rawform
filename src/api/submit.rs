use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

use crate::{db::forms, error::AppError, models::Form};

#[derive(Deserialize)]
pub struct ClientExternalPath {
    client_name: String,
    external_id: String,
}

#[derive(Deserialize)]
pub struct SubmitBody {
    pub values: Value,
}

/// Public: get submit_token + form data by client_name + external_id (no auth required).
pub async fn get_token(
    State(pool): State<SqlitePool>,
    Path(ClientExternalPath {
        client_name,
        external_id,
    }): Path<ClientExternalPath>,
) -> Result<impl IntoResponse, AppError> {
    let view = forms::find_by_client_name_and_external(&pool, &client_name, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(json!({
        "submit_token": view.submit_token,
        "data": view.data_json()?,
    })))
}

/// Public: get form definition by submit_token.
pub async fn get_form(
    State(pool): State<SqlitePool>,
    Path(submit_token): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let view = forms::find_by_submit_token(&pool, &submit_token)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(json!({
        "id": view.id,
        "external_id": view.external_id,
        "data": view.data_json()?,
        "is_active": view.is_active,
    })))
}

/// Public: submit form values.
pub async fn post_submit(
    State(pool): State<SqlitePool>,
    Path(submit_token): Path<String>,
    Json(body): Json<SubmitBody>,
) -> Result<impl IntoResponse, AppError> {
    let view = forms::find_by_submit_token(&pool, &submit_token)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(url) = view.webhook_url.clone() {
        fire_webhook(url, &view, &body.values);
    }

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({ "ok": true, "form_id": view.id })),
    ))
}

fn fire_webhook(url: String, view: &Form, values: &Value) {
    let payload = json!({
        "event": "form.submission",
        "form_id": view.id,
        "external_id": view.external_id,
        "values": values,
    });
    tokio::spawn(async move {
        let client = reqwest::Client::new();
        match client.post(&url).json(&payload).send().await {
            Ok(r) if r.status().is_success() => {
                tracing::debug!("Webhook delivered to {url}: {}", r.status())
            }
            Ok(r) => tracing::warn!("Webhook to {url} returned non-success: {}", r.status()),
            Err(e) => tracing::warn!("Webhook to {url} failed: {e}"),
        }
    });
}
