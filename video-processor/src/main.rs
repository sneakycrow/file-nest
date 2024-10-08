mod error;
mod job;
mod queue;
mod runner;

use std::{sync::Arc, time::Duration};

pub use error::Error;
use queue::PostgresQueue;
use runner::run_worker;

const CONCURRENCY: usize = 50;

// TODO: Add intake for passing jobs into queue
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create a database connection
    let db = db::connect_to_database().await?;
    // Initialize a queue
    let queue = Arc::new(PostgresQueue::new(db.clone()));
    // Pass our queue (shared ref) to our runner
    let worker_queue = queue.clone();
    tokio::spawn(async move { run_worker(worker_queue, CONCURRENCY).await });

    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(())
}
