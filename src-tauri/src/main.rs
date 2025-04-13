mod constants;
mod env;
mod features;
mod utils;

use log::{error, info};
use rusb::Context;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State, WindowEvent};

use constants::{BINARY_DIR, HOST_ARCH, HOST_OS};
use env::add_to_path;
use features::appium::commands::{start_appium, stop_appium};
use features::appium::services::AppiumState;
use features::dependencies::commands::is_ios_dependencies_installed;
use features::dependencies::commands::{
    check_installed_binaries, install_appium, install_chromedriver, install_geckodriver,
    install_nodejs,
};
use features::device::constants::USB_CONTEXT;
use features::device::detect::start_usb_hotplug_monitor;
use features::screenshot::commands::take_screenshot;
use utils::logger::init_logger;

fn main() {
    init_logger(); // ロガーの初期化
    info!("Application started on {}({}).", HOST_OS, HOST_ARCH);

    // `~/.scshoki/bin` をPATHに設定
    add_to_path(&BINARY_DIR);

    // USE_CONTEXTを初期化
    USB_CONTEXT.set(Context::new().unwrap()).ok();
    // USBで接続されたデバイスを取得
    start_usb_hotplug_monitor();

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
            install_nodejs,
            install_appium,
            install_chromedriver,
            install_geckodriver,
            is_ios_dependencies_installed,
            start_appium,
            stop_appium,
            take_screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Scshoki");
}
