use std::fs;
use serde_json::json;
use tauri::command;
use tracing::{info, error};
use thirtyfour::prelude::*;

use crate::config::constants;
use crate::services::screenshot::{capture_full_page, combine_screenshots};


#[command]
pub async fn take_screenshot(url: String, hidden_elements: String) -> Result<(), String> {
    info!("Taking screenshot of {}", url);

    let mut caps = DesiredCapabilities::chrome();
    caps.insert_base_capability("platformName".to_string(), serde_json::json!("Android"));
    caps.insert_base_capability(
        "appium:options".to_string(),
        json!({
            "deviceName": "your_device_name",
            "automationName": "UiAutomator2",
        }),
    );

    let driver = WebDriver::new("http://127.0.0.1:4723/", caps)
        .await
        .map_err(|e| format!("Failed to connect to Appium: {}", e))?;

    let formatted_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("http://{}", url)
    };

    driver
        .goto(&formatted_url)
        .await
        .map_err(|e| format!("Failed to navigate to URL: {}", e))?;

    // スクロールしながらスクリーンショットを撮影
    let screenshots = capture_full_page(&driver, &hidden_elements).await?;

    let final_screenshot = combine_screenshots(screenshots)?;
    let screenshot_path = constants::SCREENSHOT_DIR.join("screenshot.png");
    fs::write(&screenshot_path, final_screenshot)
        .map_err(|e| format!("Failed to save screenshot: {}", e))?;

    info!("Saved screenshot to {:?}", screenshot_path);

    // セッションを終了
    if let Err(e) = driver.quit().await {
        error!("Failed to quit session: {}", e);
    }

    Ok(())
}
