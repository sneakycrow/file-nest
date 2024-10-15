use crate::error::Error;
use crate::{Job, Message, Queue};
use futures::{stream, StreamExt};
use sqlx::{Pool, Postgres};
use std::{sync::Arc, time::Duration};

pub async fn run_worker(queue: Arc<dyn Queue>, concurrency: usize, db_conn: &Pool<Postgres>) {
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
                tracing::debug!("Starting job {}", job.id);
                let job_id = job.id;

                let res = match handle_job(job, db_conn).await {
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

async fn handle_job(job: Job, db: &Pool<Postgres>) -> Result<(), Error> {
    match job.message {
        message @ Message::ProcessRawVideo { .. } => {
            tracing::debug!("Processing raw video: {:?}", &message);
            // Get the required data to parse the video
            let (input_path, video_id) = match &message {
                Message::ProcessRawVideo { path, video_id } => (path, video_id),
            };
            tracing::debug!(
                "Processing video: input_path={}, video_id={}",
                input_path,
                video_id
            );
            // Create our HLS stream from the mp4
            let output_path = format!("{}.m3u8", input_path.trim_end_matches(".mp4"));
            let output = std::process::Command::new("ffmpeg")
                .args(&[
                    "-i",
                    input_path,
                    "-c:v",
                    "libx264",
                    "-c:a",
                    "aac",
                    "-f",
                    "hls",
                    "-hls_time",
                    "10",
                    "-hls_list_size",
                    "0",
                    &output_path,
                ])
                .output()
                .map_err(|e| Error::VideoProcessingError(e.to_string()))?;

            if !output.status.success() {
                tracing::error!("Error processing video into hls");
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(Error::VideoProcessingError(error.to_string()));
            }
            // Update the video ID status
            sqlx::query("UPDATE videos SET processing_status = 'processed' WHERE id = $1")
                .bind(&video_id)
                .execute(db)
                .await
                .map_err(|e| Error::VideoProcessingError(e.to_string()))?;
            tracing::debug!("Successfully processed video {}", &video_id);
        }
    };

    Ok(())
}
