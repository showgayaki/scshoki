use serde_json::json;
use thirtyfour::prelude::*;

use crate::config::constants::APPIUM_SERVER_URL;
use crate::services::binaries::ensure_chromedriver;

pub async fn create_webdriver(browser: &str) -> Result<WebDriver, String> {
    let mut caps = Capabilities::new();
    caps.insert("browserName".to_string(), json!(browser));
    caps.insert("platformName".to_string(), json!("mac"));

    match browser {
        "chrome" => {
            let chromedriver_path = ensure_chromedriver()?;
            let chromedriver_str = chromedriver_path
                .to_str()
                .ok_or("Invalid chromedriver path")?;
            caps.insert(
                "appium:chromedriverExecutable".to_string(),
                json!(chromedriver_str),
            );
            caps.insert("appium:automationName".to_string(), json!("UiAutomator2"));
        }
        "firefox" => {
            caps.insert("appium:automationName".to_string(), json!("Gecko"));
            caps.insert(
                "moz:firefoxOptions".to_string(),
                json!({
                    "androidPackage": "org.mozilla.firefox",
                }),
            );
        }
        _ => return Err("Unsupported browser".to_string()),
    };

    WebDriver::new(APPIUM_SERVER_URL, caps)
        .await
        .map_err(|e| format!("Failed to start WebDriver: {}", e))
}
