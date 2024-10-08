use crate::error::Error;
use crate::queue::{Job, Message, Queue};
use futures::{stream, StreamExt};
use std::{sync::Arc, time::Duration};

pub async fn run_worker(queue: Arc<dyn Queue>, concurrency: usize) {
    loop {
        let jobs = match queue.pull(concurrency as i32).await {
            Ok(jobs) => jobs,
            Err(err) => {
                // Trace the error
                tracing::error!("runner: error pulling jobs {}", err);
                // Go to sleep and try again
                tokio::time::sleep(Duration::from_millis(500)).await;
                Vec::new()
            }
        };

        let number_of_jobs = jobs.len();
        if number_of_jobs > 0 {
            tracing::debug!("Fetched {} jobs", number_of_jobs);
        }

        stream::iter(jobs)
            .for_each_concurrent(concurrency, |job| async {
                let job_id = job.id;

                let res = match handle_job(job).await {
                    Ok(_) => queue.delete_job(job_id).await,
                    Err(err) => {
                        println!("run_worker: handling job({}): {}", job_id, &err);
                        queue.fail_job(job_id).await
                    }
                };

                match res {
                    Ok(_) => {}
                    Err(err) => {
                        println!("run_worker: deleting / failing job: {}", &err);
                    }
                }
            })
            .await;

        tokio::time::sleep(Duration::from_millis(125)).await;
    }
}

async fn handle_job(job: Job) -> Result<(), Error> {
    match job.message {
        message @ Message::ProcessRawVideo { .. } => {
            println!("Processing raw video: {:?}", &message);
        }
    };

    Ok(())
}
