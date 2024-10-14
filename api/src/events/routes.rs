use std::sync::Arc;

use askama_axum::IntoResponse;
use axum::extract::{ws::WebSocket, State, WebSocketUpgrade};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Serialize;
use sqlx::postgres::PgListener;
use tokio::sync::mpsc;

use crate::AppState;

use super::Event;

pub async fn event_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// The serialized message we send back to the client
#[derive(Serialize)]
struct ServerMessage {
    event: Event,
}

/// A websocket handler that passes upload_status events to the client
/// NOTE: This currently sends _all_ notifications to the subscribed client
/// TODO: Add authorization and filter only events the client cares about
async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    // Split our websocket sender/receiver for communication with the client
    let (mut sender, mut receiver) = socket.split();

    // Create a new sender/receiver for sending/receiving events from Postgres
    let (ev_sender, mut ev_receiver): (mpsc::Sender<Event>, mpsc::Receiver<Event>) =
        mpsc::channel(100);

    // Spawn a task for listening for notifications from Postgres on the upload_status trigger
    tokio::spawn(async move {
        // Start the listener
        let mut listener = PgListener::connect_with(&state.db).await.unwrap();
        listener.listen("upload_status").await.unwrap();
        tracing::debug!("Listening for upload status changes");

        // When we receive a notification, parse it into an Event and pass it to the sender
        while let Ok(notification) = listener.recv().await {
            tracing::debug!("Notification received!");
            let payload = notification.payload();
            let parts: Vec<&str> = payload.split(',').collect();
            tracing::debug!("Payload: {:?}", payload);
            tracing::debug!("Parts: {:?}", parts);
            if parts.len() == 2 {
                let event = Event::VideoProcessingStatus {
                    video_id: parts[0].to_string(),
                    status: parts[1].to_string(),
                };
                tracing::debug!("Outputting parsed event");
                let _ = ev_sender.send(event);
            }
        }
    });
    // Send events to the client
    let mut send_task = tokio::spawn(async move {
        while let Some(event) = ev_receiver.recv().await {
            let server_message = ServerMessage { event };
            if let Ok(json) = serde_json::to_string(&server_message) {
                if sender
                    .send(axum::extract::ws::Message::Text(json))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    });

    // Parse messages from the client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_message)) = receiver.next().await {
            // Process incoming message from client
        }
    });

    // If any of the tasks exit, abort the other one
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
