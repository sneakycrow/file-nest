use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
};

use serde::Deserialize;
use sqlx::PgPool;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{templates::WatchTemplate, AppState};

#[derive(Deserialize)]
pub struct WatchQuery {
    v: String, // The ID of the video to get
}

/// Handle streaming of MP4 video files
pub async fn handle_stream_mp4(
    Query(params): Query<WatchQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Check if the query is empty
    if params.v.is_empty() {
        return (StatusCode::BAD_REQUEST, "Video ID is required").into_response();
    }
    // Get the file path for the video stored in the database
    let video_path = match get_raw_video_by_id(&state.db, params.v).await {
        Ok(path) => path,
        Err(_) => return (StatusCode::NOT_FOUND, "Video not found").into_response(),
    };
    // Try to open the file
    match File::open(video_path).await {
        Ok(file) => {
            // Convert the file into a stream
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            // Create the response with appropriate headers
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "video/mp4"),
                    (header::ACCEPT_RANGES, "bytes"),
                ],
                body,
            )
                .into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "File not found").into_response(),
    }
}

async fn get_raw_video_by_id(db: &PgPool, id: String) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT path FROM videos WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(row.0)
}

/// Handle streaming of HLS video files
pub async fn handle_stream_video(Query(params): Query<WatchQuery>) -> WatchTemplate {
    WatchTemplate {
        video_id: params.v.clone(),
        stream_url: format!("/uploads/{}.m3u8", params.v),
    }
}
