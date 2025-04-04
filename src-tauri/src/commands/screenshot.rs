use log::{debug, error, info};
use std::fs;
use tauri::command;
use tauri::State;

use crate::config::constants::{APPIUM_TIMEOUT, SCREENSHOT_DIR};
use crate::services::appium::AppiumState;
use crate::services::screenshot::{capture_full_page, combine_screenshots};
use crate::services::webrdiver::create_webdriver;
use crate::utils::wait::{wait_for_appium_ready, wait_for_page_load};

#[command]
pub async fn take_screenshot(
    state: State<'_, AppiumState>,
    url: String,
    hidden_elements: String,
) -> Result<(), String> {
    debug!("take_screenshot");

    // Appiumサーバーを起動
    if let Err(e) = state.start_appium().await {
        error!("Failed to start Appium: {}", e);
        return Err(format!("Failed to start Appium: {}", e));
    }

    // Appiumの起動を待機
    if let Err(e) = wait_for_appium_ready(APPIUM_TIMEOUT).await {
        error!("Appium did not start in time: {}", e);
        return Err(e);
    }

    info!("Taking screenshot of {}", url);
    let driver = create_webdriver("chrome").await?;

    let formatted_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
        // url.replace("https://", "googlechrome://")
        //     .replace("http://", "googlechrome://")
    } else {
        format!("http://{}", url)
    };

    driver
        .goto(&formatted_url)
        .await
        .map_err(|e| format!("Failed to navigate to URL: {}", e))?;

    // ページの完全読み込みを待つ
    if let Err(e) = wait_for_page_load(&driver, &formatted_url).await {
        error!("{}", e);
    }

    // スクロールしながらスクリーンショットを撮影
    let screenshots = capture_full_page(&driver, &hidden_elements).await?;

    // セッションを終了
    if let Err(e) = driver.quit().await {
        error!("Failed to quit session: {}", e);
    }

    // Appiumサーバーを停止
    if let Err(e) = state.stop_appium() {
        error!("Failed to stop Appium: {}", e);
    }

    let final_screenshot = combine_screenshots(screenshots)?;
    let screenshot_path = SCREENSHOT_DIR.join("screenshot.png");
    fs::write(&screenshot_path, final_screenshot)
        .map_err(|e| format!("Failed to save screenshot: {}", e))?;

    info!("Saved screenshot to {:?}", screenshot_path);

    Ok(())
}
