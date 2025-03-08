mod config;
mod commands;
mod services;

use std::sync::{Arc, Mutex};
use tauri::{Manager, State, WindowEvent};

use services::appium::AppiumState;
use commands::appium::{start_appium, stop_appium};
use commands::screenshot::take_screenshot;


fn main() {
    tauri::Builder::default()
        .manage(AppiumState {
            process: Arc::new(Mutex::new(None)),
        })
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state: State<AppiumState> = app_handle.state();
                if let Err(e) = state.start_appium().await {
                    eprintln!("Failed to start Appium: {}", e);
                }
            });
            Ok(())
        })
        .on_window_event(|app, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                let state: State<AppiumState> = app.state();
                if let Err(e) = state.stop_appium() {
                    eprintln!("Failed to stop Appium: {}", e);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            start_appium,
            stop_appium,
            take_screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
