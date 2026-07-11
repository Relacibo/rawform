use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{api::auth::hash_key, db::clients};

pub async fn create_client(pool: &SqlitePool, name: &str) -> anyhow::Result<()> {
    let api_key = format!("rawform_{}", Uuid::new_v4());
    let hash = hash_key(&api_key);
    let client = clients::insert(pool, name, &hash).await?;
    println!("Created client '{}' (id: {})", client.name, client.id);
    println!("API Key (shown once): {api_key}");
    Ok(())
}
