use log::{debug, error, info};
use serde_json::json;
use std::fs;
use tauri::command;
use tauri::State;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration, Instant};

use crate::config::constants::{APPIUM_SERVER_URL, APPIUM_TIMEOUT, SCREENSHOT_DIR};
use crate::services::appium::AppiumState;
use crate::services::screenshot::{capture_full_page, combine_screenshots};
use crate::services::wait::wait_for_page_load;

// Appiumが起動完了するまで `/status` をポーリング
async fn wait_for_appium_ready(timeout: Duration) -> Result<(), String> {
    debug!("wait_for_appium_ready");
    let start_time = Instant::now();
    let client = reqwest::Client::new();

    while start_time.elapsed() < timeout {
        if let Ok(response) = client
            .get(format!("{}/status", APPIUM_SERVER_URL))
            .send()
            .await
        {
            if response.status().is_success() {
                return Ok(()); // Appium起動完了
            }
        }
        sleep(Duration::from_millis(500)).await; // 500ms 待って再試行
    }

    Err("Timed out waiting for Appium to be ready".to_string())
}

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

    let mut caps = DesiredCapabilities::chrome();
    caps.insert_base_capability("platformName".to_string(), json!("Android"));
    caps.insert_base_capability(
        "appium:options".to_string(),
        json!({
            "deviceName": "your_device_name",
            "automationName": "UiAutomator2",
        }),
    );

    let driver = WebDriver::new(APPIUM_SERVER_URL, caps)
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

    // ページの完全読み込みを待つ
    wait_for_page_load(&driver, &formatted_url).await?;

    // スクロールしながらスクリーンショットを撮影
    let screenshots = capture_full_page(&driver, &hidden_elements).await?;

    let final_screenshot = combine_screenshots(screenshots)?;
    let screenshot_path = SCREENSHOT_DIR.join("screenshot.png");
    fs::write(&screenshot_path, final_screenshot)
        .map_err(|e| format!("Failed to save screenshot: {}", e))?;

    info!("Saved screenshot to {:?}", screenshot_path);

    // セッションを終了
    if let Err(e) = driver.quit().await {
        error!("Failed to quit session: {}", e);
    }

    // Appiumサーバーを停止
    if let Err(e) = state.stop_appium() {
        error!("Failed to stop Appium: {}", e);
    }

    Ok(())
}
