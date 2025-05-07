use log::{debug, info};
use tauri::{AppHandle, Emitter};
use tokio::time::{sleep, Duration, Instant};

use crate::constants::APPIUM_SERVER_URL;

// Appiumが起動完了するまで `/status` をポーリング
pub async fn wait_for_appium_ready(app: AppHandle, timeout: Duration) -> Result<(), String> {
    debug!("wait_for_appium_ready");
    let start_time = Instant::now();
    let client = reqwest::Client::new();

    while start_time.elapsed() < timeout {
        if let Ok(response) = client
            .get(format!("{}/status", &*APPIUM_SERVER_URL))
            .send()
            .await
        {
            if response.status().is_success() {
                info!("Appium server started.");
                let _ = app.emit("appium_ready", ());
                return Ok(()); // Appium起動完了
            }
        }
        sleep(Duration::from_millis(500)).await; // 500ms 待って再試行
    }

    Err("Timed out waiting for Appium to be ready".to_string())
}
