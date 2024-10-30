use futures_util::{pin_mut, StreamExt};
use obws::Client;
use serde_json::json;
use tauri::Emitter;
use tracing::info;

pub struct ObsHandler {
    client: Client,
    app: tauri::AppHandle,
}

impl ObsHandler {
    pub async fn new(
        app: tauri::AppHandle,
    ) -> Result<Self, Box<(dyn Send + Sync + std::error::Error + 'static)>> {
        match Client::connect("localhost", 4455, Some("your-password")).await {
            Ok(client) => {
                info!("Client connected!");

                // Emit connected status to frontend
                app.emit("obs-status", json!({ "connected": true }))
                    .unwrap();

                Ok(Self { client, app })
            }
            Err(e) => {
                // Emit disconnected status to frontend
                app.emit("obs-status", json!({ "connected": false }))
                    .unwrap();
                Err(Box::new(e))
            }
        }
    }

    pub async fn start_monitoring(
        &self,
    ) -> Result<(), Box<(dyn Send + Sync + std::error::Error + 'static)>> {
        let events = self.client.events()?;
        pin_mut!(events);

        // Set up periodic connection check
        let app_handle = self.app.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
            let client = Client::connect("localhost", 4455, Some("your-password"))
                .await
                .expect("Failed to create client for connection check");
            loop {
                interval.tick().await;

                // Check if still connected
                match client.general().version().await {
                    Ok(_) => {
                        app_handle
                            .emit("obs-status", json!({ "connected": true }))
                            .unwrap();
                    }
                    Err(_) => {
                        app_handle
                            .emit("obs-status", json!({ "connected": false }))
                            .unwrap();
                    }
                }
            }
        });

        while let Some(event) = events.next().await {
            match event {
                obws::events::Event::RecordStateChanged {
                    active,
                    state,
                    path,
                } => {
                    // Emit recording state to frontend
                    self.app
                        .emit(
                            "recording-status",
                            json!({
                                "isRecording": active,
                                "currentFile": path,
                                "state": state
                            }),
                        )
                        .unwrap();

                    info!("Recording state changed - active: {active:?}, state: {state:?}, path: {path:?}");
                }
                _ => {
                    info!("Some other event happened {event:?}")
                }
            }
        }

        // If we exit the event loop, we're disconnected
        self.app
            .emit("obs-status", json!({ "connected": false }))
            .unwrap();
        Ok(())
    }
}
