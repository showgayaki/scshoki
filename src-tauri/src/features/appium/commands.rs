use tauri::{command, AppHandle, State};

use super::services::AppiumState;

#[command]
pub async fn start_appium(
    state: State<'_, AppiumState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    state.start_appium(app_handle).await
}

#[command]
pub fn stop_appium(state: State<'_, AppiumState>) -> Result<(), String> {
    state.stop_appium()
}
