use std::sync::Arc;

use axum::extract::Multipart;
use axum::extract::State;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::templates::UploadTemplate;
use crate::AppState;

pub async fn handle_upload_mp4(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> UploadTemplate {
    let file_id = nanoid::nanoid!();
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
        let query = sqlx::query("INSERT INTO videos (id, path) VALUES ($1, $2)")
            .bind(&file_id)
            .bind(file_path);
        query.execute(&state.db).await.unwrap();
        UploadTemplate::new().upload_id(&file_id.to_string())
    } else {
        UploadTemplate::new().upload_error("Some error uploading file")
    }
}
