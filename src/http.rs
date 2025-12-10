use std::net::SocketAddrV4;

use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod health;
pub mod middleware;
pub mod result;
pub mod user;

use crate::apidoc::ApiDoc;
use crate::http::middleware::authorization::authorize_middleware;
use crate::state::AppState;

async fn init_router(app_state: &AppState) -> anyhow::Result<Router> {
    let health_router = Router::new().route("/health", get(health::health_check));

    let user_router = Router::new().route("/{user_id}", get(user::get_user));

    let api_router = Router::new()
        .nest("/users", user_router)
        .route_layer(from_fn_with_state(app_state.clone(), authorize_middleware));

    let auth_router = Router::new().route("/login", post(auth::login_user));

    let doc_router = SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi());

    let router = Router::new()
        .merge(doc_router)
        .nest("/check", health_router)
        .nest("/auth", auth_router)
        .nest("/api", api_router)
        .with_state(app_state.clone());

    Ok(router)
}

async fn bind_addr(host: &str, port: u16) -> anyhow::Result<TcpListener> {
    let host = host.parse().map_err(|e| {
        anyhow::anyhow!(
            "Error when parsing listening host address '{}': {}",
            host,
            e
        )
    })?;

    let addr: SocketAddrV4 = SocketAddrV4::new(host, port);

    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("Error when binding to address {}: {}", addr, e))?;

    tracing::debug!("Server now listening on {}", addr);

    Ok(listener)
}

async fn signal_term() {
    tracing::debug!("SIGNAL TERM receiver installed");

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL-C signal handler");

    tracing::debug!("SIGNAL TERM received, shutting down gracefully...");
}

pub async fn run_server(app_state: &AppState) -> anyhow::Result<()> {
    // Initialize tokio TCP listener.
    let server_host = &app_state.config().server_host;
    let server_port = app_state.config().server_port;

    let listener = bind_addr(server_host, server_port).await?;

    // Initialize make service on router.
    let router = init_router(app_state).await?;

    axum::serve(listener, router)
        .with_graceful_shutdown(signal_term())
        .await
        .map_err(|e| anyhow::anyhow!("Error running server: {}", e))?;

    Ok(())
}
