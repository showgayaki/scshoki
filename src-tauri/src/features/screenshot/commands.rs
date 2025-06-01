use log::debug;
use tauri::{command, AppHandle};

use crate::types::screenshot::{ScreenshotParams, ScreenshotResponse};
use crate::usecases::screenshot;

use super::constants::CANCEL_TOKEN;

#[command]
pub async fn take_screenshot(
    app_handle: AppHandle,
    params: ScreenshotParams,
) -> Result<ScreenshotResponse, String> {
    debug!("take_screenshot command called");
    screenshot::take_screenshot(&app_handle, params).await
}

#[tauri::command]
pub fn cancel_screenshot() -> bool {
    debug!("cancel_screenshot command called");
    if let Some(token) = CANCEL_TOKEN.lock().unwrap().take() {
        token.cancel();
        token.is_cancelled()
    } else {
        false
    }
}
