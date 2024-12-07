mod command;
mod obs;

use obs::ObsHandler;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn run() {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "stream_uploader=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            // Initialize OBS handler
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match ObsHandler::new(app_handle).await {
                    Ok(handler) => {
                        if let Err(e) = handler.start_monitoring().await {
                            tracing::error!("Error monitoring OBS: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to connect to OBS: {}", e);
                    }
                }
            });
            // Initialize tray icon
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // Hides the window without closing the app from the system tray
                #[cfg(not(target_os = "macos"))]
                {
                    window.hide().unwrap();
                }

                #[cfg(target_os = "macos")]
                {
                    tauri::AppHandle::hide(&window.app_handle()).unwrap();
                }
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
