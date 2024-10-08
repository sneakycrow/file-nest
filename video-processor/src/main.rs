mod error;
mod job;
mod queue;
mod runner;

use std::{sync::Arc, time::Duration};

pub use error::Error;
use queue::PostgresQueue;
use runner::run_worker;

const CONCURRENCY: usize = 50;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let db = db::connect_to_database().await?;
    let queue = Arc::new(PostgresQueue::new(db.clone()));

    // run worker
    let worker_queue = queue.clone(); // queue is an Arc pointer, so we only copy the reference
    tokio::spawn(async move { run_worker(worker_queue, CONCURRENCY).await });

    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(())
}
