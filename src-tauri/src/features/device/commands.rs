use log::debug;
use tauri::{command, AppHandle};

use crate::constants::{DEFAULT_DEVICE_VALUE, DEVICE_MANUFACTURE, DEVICE_OS, DEVICE_PRODUCT_NAME};

use super::services::start_usb_hotplug_monitor;

#[command]
pub fn start_usb_monitor(app: AppHandle) {
    start_usb_hotplug_monitor(app);
}

#[command]
pub fn init_devive_info() {
    debug!("toast_shown_ack called!!!");
    let mut os = DEVICE_OS.lock().unwrap();
    let mut product_name = DEVICE_PRODUCT_NAME.lock().unwrap();
    let mut manufacturer = DEVICE_MANUFACTURE.lock().unwrap();

    *os = DEFAULT_DEVICE_VALUE.to_string();
    *product_name = DEFAULT_DEVICE_VALUE.to_string();
    *manufacturer = DEFAULT_DEVICE_VALUE.to_string();
}
