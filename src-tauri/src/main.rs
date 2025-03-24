mod commands;
mod config;
mod services;
mod utils;

use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State, WindowEvent};

use commands::appium::{start_appium, stop_appium};
use commands::screenshot::take_screenshot;
use config::constants::BINARY_DIR;
use services::appium::AppiumState;
use services::binaries::init_binaries;
use services::logger::init_logger;
use services::setup::{ensure_appium, ensure_chromedriver, ensure_geckodriver, ensure_node};

fn main() {
    init_logger(); // ロガーの初期化
    info!("Application started.");

    // `~/.scshoki/bin` をPATHに設定
    let old_path = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("{}:{}", BINARY_DIR.display(), old_path));

    // バイナリ用ディレクトリのチェック
    if let Err(e) = init_binaries() {
        error!("Failed to create binaries directory: {}", e);
    }

    // Node.jsのチェック＆ダウンロード
    if let Err(e) = ensure_node() {
        error!("Failed to setup Node.js: {}", e);
        std::process::exit(1);
    }

    // Appiumのチェック＆ダウンロード
    if let Err(e) = ensure_appium() {
        error!("Failed to install Appium: {}", e);
    }

    // ChromeDriverのチェック＆ダウンロード
    if let Err(e) = ensure_chromedriver() {
        error!("Failed to ensure ChromeDriver: {}", e);
    }

    //  GeckoDriverのチェック＆ダウンロード
    if let Err(e) = ensure_geckodriver() {
        error!("Failed to ensure GeckoDriver: {}", e);
    }

    tauri::Builder::default()
        .manage(AppiumState {
            process: Arc::new(Mutex::new(None)),
        })
        .on_window_event(|app, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                let state: State<AppiumState> = app.state();
                if let Err(e) = state.stop_appium() {
                    error!("Failed to stop Appium: {}", e);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            start_appium,
            stop_appium,
            take_screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Scshoki");
}
