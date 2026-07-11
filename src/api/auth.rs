use axum::http::HeaderMap;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::{db::clients, error::AppError, models::Client};

pub fn extract_bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

pub fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn authenticate_client(
    pool: &SqlitePool,
    headers: &HeaderMap,
    client_name: &str,
) -> Result<Client, AppError> {
    let key = extract_bearer(headers).ok_or(AppError::Unauthorized)?;
    let client = clients::find_by_name(pool, client_name)
        .await?
        .ok_or(AppError::Unauthorized)?;
    if client.api_key_hash != hash_key(&key) {
        return Err(AppError::Unauthorized);
    }
    Ok(client)
}
