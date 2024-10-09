mod config;
mod templates;
mod upload;
mod watch;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use config::Config;
use sqlx::PgPool;
use std::{path::PathBuf, sync::Arc};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
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
                .unwrap_or_else(|_| "api=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Initialize our application configuration
    let config = Config::new();
    // Create our directory for storing uploads
    tokio::fs::create_dir_all(&config.uploads_dir)
        .await
        .unwrap_or_else(|_| tracing::warn!("'uploads' directory already exists"));
    // Create our listener on configured address
    let listener = tokio::net::TcpListener::bind(&config.get_address())
        .await
        .unwrap();
    // Initialize a connection to the database
    let db = db::connect_to_database().await.unwrap();
    // Grab our assets and uploads directories for static files
    let assets_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("assets");
    let uploads_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("uploads");
    // Store shared data as state between routes
    let state = Arc::new(AppState { db, config });
    // Initialize our router with the shared state and required routes
    let app = Router::new()
        .route("/", get(templates::index_page))
        .route("/watch", get(watch::mp4::handle_stream_video))
        .route("/upload", post(upload::handle_upload_mp4))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024 * 1024))
        .nest_service("/assets", ServeDir::new(assets_path))
        .nest_service("/uploads", ServeDir::new(uploads_path))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Seconds),
                ),
        );
    // Start the server
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
