use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{extract::Multipart, http::StatusCode};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::AppState;

pub async fn handle_upload_mp4(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let file_id = nanoid::nanoid!();
    let mut file_written = false;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "file" && !file_written {
            let file_path = format!("{}/{}.mp4", state.config.uploads_dir, file_id);
            let mut file = File::create(&file_path).await.unwrap();
            file.write_all(&data).await.unwrap();
            file_written = true;
        }
    }

    if file_written {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
