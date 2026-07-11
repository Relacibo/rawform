use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    api::auth::hash_key,
    db::{clients, definitions, instances},
};

pub async fn create_client(pool: &SqlitePool, name: &str) -> anyhow::Result<()> {
    let api_key = format!("rawform_{}", Uuid::new_v4());
    let hash = hash_key(&api_key);
    let client = clients::insert(pool, name, &hash).await?;
    println!("Created client '{}' (id: {})", client.name, client.id);
    println!("API Key (shown once): {api_key}");
    Ok(())
}

pub async fn create_form(
    pool: &SqlitePool,
    client_name: &str,
    external_id: &str,
    api_key: &str,
    webhook_url: Option<&str>,
) -> anyhow::Result<()> {
    let client = clients::find_by_name(pool, client_name)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Client '{client_name}' not found or inactive"))?;

    if client.api_key_hash != hash_key(api_key) {
        anyhow::bail!("Invalid API key for client '{client_name}'");
    }

    let def = definitions::insert(pool, client.id, "{}").await?;
    let admin_token = Uuid::new_v4().to_string();
    let submit_token = Uuid::new_v4().to_string();
    let form = instances::upsert(
        pool,
        client.id,
        external_id,
        def.id,
        &admin_token,
        &submit_token,
        webhook_url,
    )
    .await?;

    println!(
        "Created form '{client_name}/{external_id}' (id: {})",
        form.id
    );
    println!("Admin token:  {}", form.admin_token);
    println!("Submit token: {}", form.submit_token);
    println!("Builder URL:  /builder.html?token={}", form.admin_token);
    println!("Form URL:     /form.html?token={}", form.submit_token);
    Ok(())
}
