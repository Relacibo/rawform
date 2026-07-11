use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::SqlitePool;

use crate::{db::definitions, error::AppError};
use super::auth::authenticate_client;

#[derive(serde::Deserialize)]
pub struct DefinitionPath {
    client_name: String,
    definition_id: i64,
}

pub async fn delete_definition(
    State(pool): State<SqlitePool>,
    headers: axum::http::HeaderMap,
    Path(DefinitionPath { client_name, definition_id }): Path<DefinitionPath>,
) -> Result<impl IntoResponse, AppError> {
    let client = authenticate_client(&pool, &headers, &client_name).await?;

    match definitions::delete(&pool, definition_id, client.id).await {
        Err(sqlx::Error::Database(e)) if e.message().contains("FOREIGN KEY") => {
            Err(AppError::BadRequest(
                "Definition is still referenced by a form instance. Update or delete the form first.".into(),
            ))
        }
        Err(e) => Err(AppError::Database(e)),
        Ok(false) => Err(AppError::NotFound),
        Ok(true) => Ok(StatusCode::NO_CONTENT),
    }
}
