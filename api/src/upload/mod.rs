use std::sync::Arc;

use axum::extract::Multipart;
use axum::extract::State;
use grpc::video_processing::raw_video_processor_client::RawVideoProcessorClient;
use grpc::video_processing::ProcessRawVideoRequest;
use sqlx::types::chrono;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tonic::Request;

use crate::templates::UploadTemplate;
use crate::AppState;

pub async fn handle_upload_mp4(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> UploadTemplate {
    let file_id = nanoid::nanoid!(10);
    let file_path = format!("{}/{}.mp4", state.config.uploads_dir, file_id);
    let mut file_written = false;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "file" && !file_written {
            let mut file = File::create(&file_path).await.unwrap();
            file.write_all(&data).await.unwrap();
            file_written = true;
        }
    }

    if file_written {
        // First, save the raw video
        let video = save_raw_upload(&state.db, file_id, file_path)
            .await
            .map_err(|err| return UploadTemplate::new().upload_error(&err))
            .expect("Could not save raw video");
        // Second, send a job to the video processing queue to process this video
        send_video_processing_request(video.id.clone(), video.raw_file_path)
            .await
            .expect("Could send processing request");
        // Lastly, render the upload complete template
        // TODO: Add processing status into template
        UploadTemplate::new().upload_id(&video.id)
    } else {
        UploadTemplate::new().upload_error("Some error uploading file")
    }
}

/// A Postgres representation of a Video
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Video {
    id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,

    raw_file_path: String,
    processed_file_path: String,
    processing_status: String,
}

/// Saves a raw video upload to the database
async fn save_raw_upload(
    db: &Pool<Postgres>,
    id: String,
    file_path: String,
) -> Result<Video, String> {
    let now = chrono::Utc::now();
    let video = Video {
        id,
        created_at: now,
        updated_at: now,
        raw_file_path: file_path,
        processed_file_path: "".to_string(),
        processing_status: "pending".to_string(),
    };
    let query = sqlx::query(
        "INSERT INTO videos (id, created_at, updated_at, raw_file_path, processing_status)
            VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&video.id)
    .bind(&video.created_at)
    .bind(&video.updated_at)
    .bind(&video.raw_file_path)
    .bind(&video.processing_status);

    query.execute(db).await.unwrap();

    Ok(video)
}

/// Sends a job to the video processor service to queue a video processing job
async fn send_video_processing_request(video_id: String, file_path: String) -> Result<(), String> {
    let queue_url = std::env::var("QUEUE_URL").expect("QUEUE_URL must be set");
    let mut client = RawVideoProcessorClient::connect(queue_url)
        .await
        .map_err(|e| format!("Could not connect to RawVideoProcessorClient {}", e))?;

    let request = Request::new(ProcessRawVideoRequest {
        id: video_id,
        path: file_path,
    });
    let response = client
        .process_raw_video(request)
        .await
        .map_err(|e| format!("Could not send video processing request {}", e));

    tracing::debug!("Job processing response: {:?}", response);
    Ok(())
}
