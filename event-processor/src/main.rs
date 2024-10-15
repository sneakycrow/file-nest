mod server;

use std::sync::Arc;

use queue::{runner::run_worker, PostgresQueue};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CONCURRENCY: usize = 50;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "event_processor=debug,db=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Create a database connection
    let db = db::connect_to_database().await?;
    // Initialize a queue
    tracing::debug!("Initializing queue");
    let queue = Arc::new(PostgresQueue::new(db.clone()));
    // Pass our queue (shared ref) to our runner
    let worker_queue = queue.clone();
    tokio::spawn(async move { run_worker(worker_queue, CONCURRENCY, &db).await });
    // Start our server to start receiving job requests
    tracing::debug!("Starting gRPC server");
    server::start_server(queue)
        .await
        .expect("Could not start gRPC server");
    Ok(())
}
