use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

use crate::{db::forms, error::AppError, models::Form};

#[derive(Deserialize)]
pub struct SubmitBody {
    pub values: Value,
}

pub async fn get_form(
    State(pool): State<SqlitePool>,
    Path(submit_token): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let form = forms::find_by_submit_token(&pool, &submit_token)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(json!({
        "id": form.id,
        "external_id": form.external_id,
        "data": form.data_json()?,
        "is_active": form.is_active,
    })))
}

pub async fn post_submit(
    State(pool): State<SqlitePool>,
    Path(submit_token): Path<String>,
    Json(body): Json<SubmitBody>,
) -> Result<impl IntoResponse, AppError> {
    let form = forms::find_by_submit_token(&pool, &submit_token)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(url) = form.webhook_url.clone() {
        fire_webhook(url, &form, &body.values);
    }

    Ok((StatusCode::ACCEPTED, Json(json!({ "ok": true, "form_id": form.id }))))
}

fn fire_webhook(url: String, form: &Form, values: &Value) {
    let payload = json!({
        "event": "form.submission",
        "form_id": form.id,
        "external_id": form.external_id,
        "values": values,
    });

    tokio::spawn(async move {
        let client = reqwest::Client::new();
        match client.post(&url).json(&payload).send().await {
            Ok(resp) if resp.status().is_success() => {
                tracing::debug!("Webhook delivered to {url}: {}", resp.status());
            }
            Ok(resp) => {
                tracing::warn!("Webhook to {url} returned non-success: {}", resp.status());
            }
            Err(e) => {
                tracing::warn!("Webhook to {url} failed: {e}");
            }
        }
    });
}
