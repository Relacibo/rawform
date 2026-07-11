use std::net::SocketAddr;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rawform::{app, cli, db};

#[derive(Parser)]
#[command(
    name = "rawform",
    version,
    about = "Minimalist self-hostable form builder"
)]
struct Cli {
    /// SQLite or PostgreSQL connection URL
    #[arg(long, env = "DATABASE_URL", default_value = "sqlite://rawform.db")]
    database_url: String,

    /// Address to listen on
    #[arg(long, env = "HOST", default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(long, env = "PORT", default_value_t = 3000)]
    port: u16,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new API client and print its API key (shown once)
    CreateClient {
        /// Unique client name
        name: String,
    },
}

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

    let args = Cli::parse();
    let pool = db::connect(&args.database_url).await?;
    db::migrate(&pool).await?;

    match args.command {
        Some(Command::CreateClient { name }) => {
            cli::create_client(&pool, &name).await?;
        }
        None => {
            let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;
            tracing::info!("Listening on {addr}");
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, app(pool)).await?;
        }
    }

    Ok(())
}
