use std::net::SocketAddrV4;

use axum::Router;
use axum::routing::{self};
use redis::Client;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tracing::debug;

mod health;

use crate::cache::Cache;
use crate::config::AppConfig;
use crate::state::AppState;

async fn init_database() -> anyhow::Result<DatabaseConnection> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|err| anyhow::anyhow!("Error when acquiring DATABASE_URL: {}", err))?;

    let database = Database::connect(database_url)
        .await
        .map_err(|err| anyhow::anyhow!("Error when connecting to database: {}", err))?;

    debug!("Database connected");

    Ok(database)
}

async fn init_cache() -> anyhow::Result<Cache> {
    let redis_url = std::env::var("REDIS_URL")
        .map_err(|err| anyhow::anyhow!("Error when acquiring REDIS_URL: {}", err))?;

    let client = Client::open(redis_url.as_str())
        .map_err(|err| anyhow::anyhow!("Error when connecting to Redis: {}", err))?;

    let cache = Cache::new(client);

    // Test the Redis with an initial PING command to ensure connectivity
    cache
        .ping_remote()
        .await
        .map_err(|err| anyhow::anyhow!("Error when PING Redis: {}", err))?;

    debug!("Redis connected");

    Ok(cache)
}

async fn new_router(app_config: &AppConfig) -> anyhow::Result<Router> {
    let database = init_database().await?;
    let cache = init_cache().await?;

    let app_state = AppState::new(app_config.clone(), database, cache);

    let health_router = Router::new().route("/check", routing::get(health::health_check));

    let router: Router = Router::new()
        .nest("/health", health_router)
        .with_state(app_state);

    Ok(router)
}

async fn bind_addr(host: &str, port: u16) -> anyhow::Result<TcpListener> {
    let addr: SocketAddrV4 = format!("{}:{}", host, port)
        .parse()
        .map_err(|err| anyhow::anyhow!("Error when parsing listening address: {}", err))?;

    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|err| anyhow::anyhow!("Error when binding to address {}: {}", addr, err))?;

    debug!("Server now listening on {}", addr);

    Ok(listener)
}

async fn signal_term() {
    debug!("SIGNAL TERM receiver installed");

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL-C signal handler");

    debug!("SIGNAL TERM received, shutting down gracefully...");
}

pub async fn run_server(app_config: &AppConfig) -> anyhow::Result<()> {
    let router = new_router(app_config).await?;

    let listener = bind_addr(&app_config.server_host, app_config.server_port).await?;

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(signal_term())
        .await
        .map_err(|err| anyhow::anyhow!("Error running server: {}", err))?;

    Ok(())
}
