use log::{error, info};
use serde_json::json;
use std::fs;
use tauri::command;

use crate::config::constants::{DEVICE_OS, SCREENSHOT_DIR};
use crate::services::screenshot::{capture_full_page, combine_screenshots};
use crate::services::webrdiver::{create_webdriver, create_webdriver_first_open};
use crate::utils::wait::wait_for_page_load;

#[command]
pub async fn take_screenshot(url: String, hidden_elements: String) -> Result<(), String> {
    info!("Taking screenshot of {}", url);

    let formatted_url = if url.starts_with("http://") || url.starts_with("https://") {
        // url.clone()
        url.replace("https://", "googlechrome://")
            .replace("http://", "googlechrome://")
    } else {
        format!("http://{}", url)
    };

    // iOSの場合、最初にWebDriverAgentを起動、ページを開いておく
    if DEVICE_OS.lock().unwrap().as_deref().unwrap_or("Unknown") == "iOS" {
        let driver_first_open = create_webdriver_first_open().await?;
        driver_first_open
            .goto(formatted_url.clone())
            .await
            .map_err(|e| format!("Failed to navigate to URL: {}", e))?;

        // ページの完全読み込みを待つ
        if let Err(e) = wait_for_page_load(&driver_first_open, &formatted_url).await {
            error!("{}", e);
        }

        // セッションを終了
        if let Err(e) = driver_first_open.quit().await {
            error!("Failed to quit session: {}", e);
        }
    }

    let driver = create_webdriver("chrome").await?;
    // ページの完全読み込みを待つ
    if let Err(e) = wait_for_page_load(&driver, &url).await {
        error!("{}", e);
    }

    // スクロールしながらスクリーンショットを撮影
    let screenshots = capture_full_page(&driver, &hidden_elements).await?;

    // WDAのアプリを終了
    if let Err(e) = driver
        .execute(
            "mobile: terminateApp",
            vec![json!({"bundleId": "com.facebook.WebDriverAgentRunner"})],
        )
        .await
    {
        error!("Failed to terminate app: {}", e);
    }

    // セッションを終了
    if let Err(e) = driver.quit().await {
        error!("Failed to quit session: {}", e);
    }

    let final_screenshot = combine_screenshots(screenshots)?;
    let screenshot_path = SCREENSHOT_DIR.join("screenshot.png");
    fs::write(&screenshot_path, final_screenshot)
        .map_err(|e| format!("Failed to save screenshot: {}", e))?;

    info!("Saved screenshot to {:?}", screenshot_path);

    Ok(())
}
