mod commands;
use std::sync::{Arc, Mutex};
use tauri::{WindowEvent};
use commands::appium::{start_appium, stop_appium};
use commands::screenshot::take_screenshot;

fn main() {
    let app_state = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .setup({
            let state = Arc::clone(&app_state);
            move |_app| {
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = start_appium(state).await {
                        eprintln!("Failed to start Appium: {}", e);
                    }
                });
                Ok(())
            }
        })
        .on_window_event({
            let state = Arc::clone(&app_state);
            move |_window, event| {
                if let WindowEvent::CloseRequested { .. } = event {
                    stop_appium(Arc::clone(&state));
                }
            }
        })
        .invoke_handler(tauri::generate_handler![take_screenshot])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
