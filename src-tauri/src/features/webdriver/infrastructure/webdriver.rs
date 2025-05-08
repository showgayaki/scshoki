use thirtyfour::prelude::*;

pub async fn webdriver(appium_server_url: &str, caps: Capabilities) -> Result<WebDriver, String> {
    WebDriver::new(appium_server_url, caps)
        .await
        .map_err(|e| format!("Failed to start WebDriver: {}", e))
}
