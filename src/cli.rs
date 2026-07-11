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

pub async fn create_definition(
    pool: &SqlitePool,
    client_name: &str,
    api_key: &str,
    data: &str,
) -> anyhow::Result<()> {
    let client = auth(pool, client_name, api_key).await?;
    // Validate that data is valid JSON
    let _: serde_json::Value =
        serde_json::from_str(data).map_err(|e| anyhow::anyhow!("Invalid JSON: {e}"))?;
    let def = definitions::insert(pool, client.id, data).await?;
    println!("Created definition (id: {})", def.id);
    println!("  client:     {client_name}");
    println!("  created_at: {}", def.created_at);
    Ok(())
}

pub async fn create_form(
    pool: &SqlitePool,
    client_name: &str,
    external_id: &str,
    api_key: &str,
    data: Option<&str>,
    definition_id: Option<i64>,
    webhook_url: Option<&str>,
) -> anyhow::Result<()> {
    let client = auth(pool, client_name, api_key).await?;

    let def_id = match (data, definition_id) {
        (Some(d), None) => {
            let _: serde_json::Value =
                serde_json::from_str(d).map_err(|e| anyhow::anyhow!("Invalid JSON: {e}"))?;
            definitions::insert(pool, client.id, d).await?.id
        }
        (None, Some(id)) => {
            let def = definitions::find_by_id(pool, id)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Definition {id} not found"))?;
            if def.client_id != client.id {
                anyhow::bail!("Definition {id} does not belong to client '{client_name}'");
            }
            id
        }
        (None, None) => {
            // Default: empty form schema
            definitions::insert(pool, client.id, "{}").await?.id
        }
        (Some(_), Some(_)) => {
            anyhow::bail!("Provide either --data or --definition-id, not both");
        }
    };

    let admin_token = Uuid::new_v4().to_string();
    let submit_token = Uuid::new_v4().to_string();
    let form = instances::upsert(
        pool,
        client.id,
        external_id,
        def_id,
        &admin_token,
        &submit_token,
        webhook_url,
    )
    .await?;

    println!(
        "Created form '{client_name}/{external_id}' (id: {})",
        form.id
    );
    println!("  definition_id: {}", form.definition_id);
    println!("  admin_token:   {}", form.admin_token);
    println!("  submit_token:  {}", form.submit_token);
    println!("  Builder URL:   /builder.html?token={}", form.admin_token);
    println!("  Form URL:      /form.html?token={}", form.submit_token);
    if let Some(url) = webhook_url {
        println!("  webhook_url:   {url}");
    }
    Ok(())
}

async fn auth(
    pool: &SqlitePool,
    client_name: &str,
    api_key: &str,
) -> anyhow::Result<crate::models::Client> {
    let client = clients::find_by_name(pool, client_name)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Client '{client_name}' not found or inactive"))?;
    if client.api_key_hash != hash_key(api_key) {
        anyhow::bail!("Invalid API key for client '{client_name}'");
    }
    Ok(client)
}
