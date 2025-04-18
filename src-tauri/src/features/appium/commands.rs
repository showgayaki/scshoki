use tauri::{command, State};

use super::services::AppiumState;

#[command]
pub async fn start_appium(
    state: State<'_, AppiumState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state.start_appium(app).await
}

#[command]
pub fn stop_appium(state: State<'_, AppiumState>) -> Result<(), String> {
    state.stop_appium()
}
