use log::{error, info};
// use serde_json::json;
use std::fs;
use tauri::command;

use super::constants::SCREENSHOT_DIR;
use super::services::{capture_full_page, combine_screenshots};
use crate::features::webdriver::services::create_webdriver;
use crate::utils::wait::wait_for_page_load;

#[command]
pub async fn take_screenshot(url: String, hidden_elements: String) -> Result<(), String> {
    info!("Taking screenshot of {}", url);
    let driver = create_webdriver("chrome", &url).await?;

    // ページの完全読み込みを待つ
    if let Err(e) = wait_for_page_load(&driver, &url).await {
        error!("{}", e);
    }

    // スクロールしながらスクリーンショットを撮影
    let screenshots = capture_full_page(&driver, &hidden_elements).await?;

    // // WDAのアプリを終了
    // if let Err(e) = driver
    //     .execute(
    //         "mobile: terminateApp",
    //         vec![json!({"bundleId": "com.facebook.WebDriverAgentRunner"})],
    //     )
    //     .await
    // {
    //     error!("Failed to terminate app: {}", e);
    // }

    // セッションを終了
    if let Err(e) = driver.quit().await {
        error!("Failed to quit session: {}", e);
    }

    let final_screenshot = combine_screenshots(&screenshots)?;
    let screenshot_path = SCREENSHOT_DIR.join("screenshot.png");
    fs::write(&screenshot_path, final_screenshot)
        .map_err(|e| format!("Failed to save screenshot: {}", e))?;

    info!("Saved screenshot to {:?}", screenshot_path);

    Ok(())
}
