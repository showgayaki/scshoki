use tauri::{command, State};
use crate::services::appium::AppiumState;


// Appium を起動する（Tauri コマンド）
#[command]
pub async fn start_appium(state: State<'_, AppiumState>) -> Result<(), String> {
    state.start_appium().await
}

// Appium を停止する（Tauri コマンド）
#[command]
pub fn stop_appium(state: State<'_, AppiumState>) -> Result<(), String> {
    state.stop_appium()
}
