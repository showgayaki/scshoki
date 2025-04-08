mod commands;
mod config;
mod infrastructure;
mod services;
mod utils;

use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State, WindowEvent};

use commands::appium::{start_appium, stop_appium};
use commands::ios_dependencies::is_ios_dependencies_installed;
use commands::screenshot::take_screenshot;
use config::constants::appium::APPIUM_TIMEOUT;
use config::constants::paths::BINARY_DIR;
use config::env::{add_to_path, HOST_ARCH, HOST_OS};
use infrastructure::dependencies::setup::check_or_install;
use infrastructure::logger::init_logger;
use services::appium::AppiumState;
use services::device::detect::detect_device;
use utils::wait::wait_for_appium_ready;

fn main() {
    init_logger(); // ロガーの初期化
    info!("Application started on {}({}).", HOST_OS, HOST_ARCH);

    // `~/.scshoki/bin` をPATHに設定
    add_to_path(&BINARY_DIR);

    // USBで接続されたデバイスを取得
    detect_device();

    // バイナリのチェック＆インストール
    check_or_install();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppiumState {
            process: Arc::new(Mutex::new(None)),
        })
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state: State<AppiumState> = app_handle.state();
                // Appiumサーバーの起動
                if let Err(e) = state.start_appium().await {
                    error!("{}", e);
                } else if let Err(e) = wait_for_appium_ready(APPIUM_TIMEOUT).await {
                    // Appiumサーバーの起動チェック
                    error!("{}", e);
                }
            });
            Ok(())
        })
        .on_window_event(|app, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                let state: State<AppiumState> = app.state();
                if let Err(e) = state.stop_appium() {
                    error!("{}", e);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            start_appium,
            stop_appium,
            is_ios_dependencies_installed,
            take_screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Scshoki");
}
