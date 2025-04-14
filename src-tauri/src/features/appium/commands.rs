use log::error;
use tauri::{command, State};

use super::constants::APPIUM_TIMEOUT;
use super::services::AppiumState;
use super::wait::wait_for_appium_ready;

#[command]
pub async fn start_appium(state: State<'_, AppiumState>) -> Result<(), String> {
    state.start_appium().await?;

    // Appium サーバーの起動を待機
    wait_for_appium_ready(APPIUM_TIMEOUT).await.map_err(|e| {
        error!("{}", e);
        e
    })?;
    Ok(())
}

#[command]
pub fn stop_appium(state: State<'_, AppiumState>) -> Result<(), String> {
    state.stop_appium()
}
