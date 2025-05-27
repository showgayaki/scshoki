use log::debug;
use tauri::command;

use crate::types::screenshot::ScreenshotParams;
use crate::usecases::screenshot;

#[command]
pub async fn take_screenshot(params: ScreenshotParams) -> Result<(), String> {
    debug!("take_screenshot command called");
    screenshot::take_screenshot(params).await
}
