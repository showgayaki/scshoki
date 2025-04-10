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
use commands::setup::{
    check_installed_binaries, setup_appium, setup_chromedriver, setup_geckodriver, setup_node,
};
use config::constants::paths::BINARY_DIR;
use config::env::{add_to_path, HOST_ARCH, HOST_OS};
use infrastructure::logger::init_logger;
use services::appium::AppiumState;
use services::device::detect::detect_device;

fn main() {
    init_logger(); // ロガーの初期化
    info!("Application started on {}({}).", HOST_OS, HOST_ARCH);

    // `~/.scshoki/bin` をPATHに設定
    add_to_path(&BINARY_DIR);

    // USBで接続されたデバイスを取得
    detect_device();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppiumState {
            process: Arc::new(Mutex::new(None)),
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
            check_installed_binaries,
            setup_node,
            setup_appium,
            setup_chromedriver,
            setup_geckodriver,
            is_ios_dependencies_installed,
            start_appium,
            stop_appium,
            take_screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Scshoki");
}
