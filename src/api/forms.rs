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
    db::{
        definitions,
        instances::{self, InstancePatch},
    },
    error::AppError,
    models::InstanceView,
};

#[derive(Deserialize)]
pub struct FormPath {
    client_name: String,
    external_id: String,
}

/// PUT body: supply either `data` (auto-creates a new definition) or an
/// existing `definition_id`. Exactly one must be present.
#[derive(Deserialize)]
pub struct PutFormBody {
    pub data: Option<Value>,
    pub definition_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_maybe")]
    pub webhook_url: Option<Option<String>>,
}

#[derive(Deserialize)]
pub struct PatchFormBody {
    /// If set, creates a new definition and assigns it.
    pub data: Option<Value>,
    /// If set, assigns an existing definition (must belong to same client).
    pub definition_id: Option<i64>,
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
        "admin_token": v.admin_token,
        "submit_token": v.submit_token,
        "created_at": v.created_at,
        "updated_at": v.updated_at,
    }))
}

/// Resolve the definition_id from PUT/PATCH body: either use an existing one
/// (verifying client ownership) or create a new one from `data`.
async fn resolve_definition(
    pool: &SqlitePool,
    client_id: i64,
    data: Option<&Value>,
    definition_id: Option<i64>,
) -> Result<i64, AppError> {
    match (data, definition_id) {
        (Some(d), None) => {
            let json = serde_json::to_string(d).unwrap();
            let def = definitions::insert(pool, client_id, &json).await?;
            Ok(def.id)
        }
        (None, Some(id)) => {
            let def = definitions::find_by_id(pool, id)
                .await?
                .ok_or(AppError::NotFound)?;
            if def.client_id != client_id {
                return Err(AppError::Unauthorized);
            }
            Ok(id)
        }
        (Some(_), Some(_)) => Err(AppError::BadRequest(
            "Provide either 'data' or 'definition_id', not both.".into(),
        )),
        (None, None) => Err(AppError::BadRequest(
            "Provide either 'data' or 'definition_id'.".into(),
        )),
    }
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
    let def_id =
        resolve_definition(&pool, client.id, body.data.as_ref(), body.definition_id).await?;
    let webhook_url = body.webhook_url.flatten();
    let admin_token = Uuid::new_v4().to_string();
    let submit_token = Uuid::new_v4().to_string();
    let view = instances::upsert(
        &pool,
        client.id,
        &external_id,
        def_id,
        &admin_token,
        &submit_token,
        webhook_url.as_deref(),
    )
    .await?;
    Ok((StatusCode::OK, Json(instance_json(&view)?)))
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
    let view = instances::find_by_client_and_external(&pool, client.id, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(instance_json(&view)?))
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
    let view = instances::find_by_client_and_external(&pool, client.id, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let definition_id = match (body.data.as_ref(), body.definition_id) {
        (Some(_), Some(_)) => {
            return Err(AppError::BadRequest(
                "Provide either 'data' or 'definition_id', not both.".into(),
            ));
        }
        (Some(d), None) => {
            let json = serde_json::to_string(d).unwrap();
            let def = definitions::insert(&pool, client.id, &json).await?;
            Some(def.id)
        }
        (None, Some(id)) => {
            let def = definitions::find_by_id(&pool, id)
                .await?
                .ok_or(AppError::NotFound)?;
            if def.client_id != client.id {
                return Err(AppError::Unauthorized);
            }
            Some(id)
        }
        (None, None) => None,
    };

    let updated = instances::patch(
        &pool,
        view.id,
        InstancePatch {
            definition_id,
            webhook_url: body.webhook_url,
            is_active: body.is_active,
        },
    )
    .await?;
    Ok(Json(instance_json(&updated)?))
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
    let view = instances::find_by_client_and_external(&pool, client.id, &external_id)
        .await?
        .ok_or(AppError::NotFound)?;
    instances::delete(&pool, view.id, client.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
