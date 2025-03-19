use log::{debug, info};
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration, Instant};

pub async fn wait_for_page_load(driver: &WebDriver, url: &str) -> Result<(), String> {
    debug!("wait_for_page_load");

    let timeout = Duration::from_secs(10); // 最大10秒待つ
    let start_time = Instant::now();

    while start_time.elapsed() < timeout {
        let ready_state = driver
            .execute("return document.readyState", vec![])
            .await
            .map_err(|e| format!("Failed to check page ready state: {}", e))?
            .json() // ScriptRet → serde_json::Value に変換
            .as_str() // serde_json::Value → Option<&str> に変換
            .ok_or("Failed to parse document.readyState")? // Noneならエラー
            .to_string(); // String に変換

        if ready_state == "complete" {
            info!("[{}] is loaded.", url);
            return Ok(());
        }

        sleep(Duration::from_millis(500)).await; // 0.5秒ごとに再チェック
    }

    Err("Timed out waiting for page to load".to_string())
}

pub async fn wait_for_scroll_complete(driver: &WebDriver) -> Result<(), String> {
    let timeout = std::time::Duration::from_secs(5); // 最大5秒待つ
    let start_time = std::time::Instant::now();
    let mut last_scroll_y = -1.0;

    while start_time.elapsed() < timeout {
        let current_scroll_y = driver
            .execute("return window.scrollY;", vec![])
            .await
            .map_err(|e| format!("Failed to get scroll position: {}", e))?
            .json()
            .as_f64()
            .unwrap_or(0.0);

        if (current_scroll_y - last_scroll_y).abs() < f64::EPSILON {
            return Ok(());
        }

        last_scroll_y = current_scroll_y;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await; // 200msごとにチェック
    }

    Err("Timed out waiting for scroll to complete".to_string())
}

pub async fn wait_for_elements_hidden(driver: &WebDriver, selectors: &str) -> Result<(), String> {
    let timeout = std::time::Duration::from_secs(5); // 最大5秒待つ
    let start_time = std::time::Instant::now();

    let script = format!(
        r#"
        return Array.from(document.querySelectorAll("{}"))
            .every(e => getComputedStyle(e).visibility === "hidden");
        "#,
        selectors
    );

    while start_time.elapsed() < timeout {
        let result = driver
            .execute(&script, vec![])
            .await
            .map_err(|e| format!("Failed to check element visibility: {}", e))?
            .json()
            .as_bool()
            .unwrap_or(false);

        if result {
            return Ok(());
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }

    Err("Timed out waiting for elements to become hidden".to_string())
}
