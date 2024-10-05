mod config;
mod db;
mod upload;
mod watch;

use axum::{routing::get, routing::post, Router};
use config::Config;
use sqlx::PgPool;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {
    db: PgPool,
    config: Config,
}

#[tokio::main]
async fn main() {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Initialize our server configuration
    let config = Config::new();
    // Create our directory for storing uploads
    tokio::fs::create_dir_all(&config.server.uploads_dir)
        .await
        .unwrap_or_else(|_| tracing::warn!("'uploads' directory already exists"));
    // Create our listener on configured address
    let listener = tokio::net::TcpListener::bind(&config.get_address())
        .await
        .unwrap();
    // Initialize a connection to the database and store it in state
    let db = db::connect_to_database().await.unwrap();
    let state = Arc::new(AppState { db, config });
    // Initialize our router with the shared state and required routes
    let app = Router::new()
        .route("/watch", get(watch::mp4::handle_stream_mp4))
        .route("/upload", post(upload::handle_upload_mp4))
        .with_state(state);
    // Start the server
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
