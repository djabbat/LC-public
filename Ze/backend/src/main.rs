use axum::{Router, Server};
use std::net::SocketAddr;
use tracing_subscriber;
use ze_backend::{config::Config, db, error::AppError, routes};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("ze_backend=debug,tower_http=debug")
        .init();

    tracing::info!("Starting Ze backend server...");

    // Load configuration
    let config = Config::from_env()?;
    tracing::debug!("Loaded config: {:?}", config);

    // Initialize database connection pool and run migrations
    let pool = db::init(&config.database_url).await?;

    // Build application with routes
    let app = Router::new()
        .nest("/api", routes::api_routes())
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    // Graceful shutdown
    let graceful = axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .with_graceful_shutdown(shutdown_signal());

    graceful.await?;
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("Shutdown signal received");
}