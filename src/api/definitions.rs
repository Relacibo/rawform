use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

use super::auth::authenticate_client;
use crate::{db::definitions, error::AppError};

#[derive(Deserialize)]
pub struct ClientPath {
    client_name: String,
}

#[derive(Deserialize)]
pub struct DefinitionPath {
    client_name: String,
    definition_id: i64,
}

#[derive(Deserialize)]
pub struct CreateDefinitionBody {
    pub data: Value,
}

/// POST /api/v1/definitions/:client_name — create a standalone definition.
pub async fn create_definition(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(ClientPath { client_name }): Path<ClientPath>,
    Json(body): Json<CreateDefinitionBody>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;
    let data = serde_json::to_string(&body.data).unwrap();
    let def = definitions::insert(&pool, client.id, &data).await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": def.id,
            "client_id": def.client_id,
            "data": body.data,
            "created_at": def.created_at,
        })),
    ))
}

/// DELETE /api/v1/definitions/:client_name/:definition_id
pub async fn delete_definition(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(DefinitionPath {
        client_name,
        definition_id,
    }): Path<DefinitionPath>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;

    match definitions::delete(&pool, definition_id, client.id).await {
        Err(sqlx::Error::Database(e)) if e.message().contains("FOREIGN KEY") => {
            Err(AppError::BadRequest(
                "Definition is still referenced by a form. Reassign or delete the form first."
                    .into(),
            ))
        }
        Err(e) => Err(AppError::Database(e)),
        Ok(false) => Err(AppError::NotFound),
        Ok(true) => Ok(StatusCode::NO_CONTENT),
    }
}
