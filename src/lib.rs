use tracing_subscriber::EnvFilter;

mod apidoc;
mod cache;
mod config;
mod http;
mod jwt_codec;
mod model;
mod repo;
mod result_trace;
mod service;
mod state;

use crate::{cache::Cache, config::AppConfig, jwt_codec::JwtCodec, repo::Repo, state::AppState};

async fn init_env() -> anyhow::Result<()> {
    dotenvy::dotenv().map_err(|e| anyhow::anyhow!("Error when loading env: {}", e))?;

    tracing::debug!("Environment variables loaded from .env file");

    Ok(())
}

async fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::debug!("Logger initialized");
}

async fn init_config() -> anyhow::Result<AppConfig> {
    let config = AppConfig::try_from_file(None)
        .map_err(|e| anyhow::anyhow!("Error when loading config: {}", e))?;

    tracing::debug!("Configuration loaded: {:?}", config);

    Ok(config)
}

fn init_jwt_codec() -> anyhow::Result<JwtCodec> {
    let jwt_codec =
        JwtCodec::new().map_err(|e| anyhow::anyhow!("Error when initializing JWT codec: {}", e))?;

    tracing::debug!("JWT codec initialized");

    Ok(jwt_codec)
}

async fn init_repo() -> anyhow::Result<Repo> {
    let database = Repo::new()
        .await
        .map_err(|e| anyhow::anyhow!("Error when initializing repo: {}", e))?;

    database
        .ping()
        .await
        .map_err(|e| anyhow::anyhow!("Error when PING repo: {}", e))?;

    tracing::debug!("Repo connected");

    Ok(database)
}

async fn init_cache() -> anyhow::Result<Cache> {
    let cache =
        Cache::new().map_err(|e| anyhow::anyhow!("Error when initializing cache: {}", e))?;

    // Test the Redis with an initial PING command to ensure connectivity
    cache
        .ping()
        .await
        .map_err(|e| anyhow::anyhow!("Error when PING cache: {}", e))?;

    tracing::debug!("Redis connected");

    Ok(cache)
}

pub async fn run() -> anyhow::Result<()> {
    init_env().await?;

    init_logger().await;

    let config = init_config().await?;

    let jwt_codec = init_jwt_codec()?;

    let cache = init_cache().await?;

    let repo = init_repo().await?;

    let app_state = AppState::new(config, repo, cache, jwt_codec);

    http::run_server(&app_state).await?;

    Ok(())
}
