use std::net::SocketAddr;

use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rawform::{app, cli, config::Config, db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rawform=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;

    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("create-client") => {
            let name = args
                .get(1)
                .ok_or_else(|| anyhow::anyhow!("Usage: rawform create-client <name>"))?;
            let pool = db::connect(&config.database_url).await?;
            db::migrate(&pool).await?;
            cli::create_client(&pool, name).await?;
        }
        _ => {
            let pool = db::connect(&config.database_url).await?;
            db::migrate(&pool).await?;
            let router = app(pool);
            let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
            tracing::info!("Listening on {addr}");
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, router).await?;
        }
    }

    Ok(())
}
