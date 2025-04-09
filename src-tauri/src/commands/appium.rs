use log::error;
use tauri::{command, State};

use crate::config::constants::appium::APPIUM_TIMEOUT;
use crate::services::appium::AppiumState;
use crate::utils::wait::wait_for_appium_ready;

#[command]
pub async fn start_appium(state: State<'_, AppiumState>) -> Result<(), String> {
    state.start_appium().await.map_err(|e| {
        error!("{}", e);
        e
    })?;

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
