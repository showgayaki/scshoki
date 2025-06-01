use log::debug;
use tauri::{AppHandle, Emitter};

pub fn emit_screenshot_status(app_handle: &AppHandle, message: &str) {
    debug!("Emitting screenshot status: {}", message);
    let _ = app_handle.emit("screenshot_status", message);
}
