use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgListener, PgPool};
use tokio::sync::broadcast::Sender;

pub mod routes;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    VideoProcessingStatus { video_id: String, status: String },
}
