mod commands;
mod config;
mod infrastructure;
mod services;
mod setup;
mod utils;

use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State, WindowEvent};

use commands::appium::{start_appium, stop_appium};
use commands::screenshot::take_screenshot;
use config::constants::{BINARY_DIR, HOST_ARCH, HOST_OS};
use config::env::add_to_path;
use infrastructure::binaries::init_binaries;
use infrastructure::logger::init_logger;
use services::appium::AppiumState;
use services::device::detect::detect_device;
use setup::ensure::{ensure_appium, ensure_chromedriver, ensure_geckodriver, ensure_node};

fn main() {
    init_logger(); // ロガーの初期化
    info!("Application started on {}({}).", HOST_OS, HOST_ARCH);

    // `~/.scshoki/bin` をPATHに設定
    add_to_path(&BINARY_DIR);

    // USBで接続されたデバイスを取得
    detect_device();

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
