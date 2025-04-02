use serde_json::json;
use thirtyfour::prelude::*;

use crate::config::constants::{
    APPIUM_PORT, APPIUM_SERVER_URL, DEVELOPMENT_TEAM, DEVICE_OS, DEVICE_UDID, HOST_OS, IDENTIFIER,
    IOS_VERSION,
};
use crate::setup::ensure::ensure_chromedriver;

pub async fn create_webdriver(browser: &str) -> Result<WebDriver, String> {
    let mut caps = Capabilities::new();
    caps.insert("browserName".to_string(), json!(browser));

    match browser {
        "chrome" => {
            let chromedriver_path = ensure_chromedriver()?;
            let chromedriver_str = chromedriver_path
                .to_str()
                .ok_or("Invalid chromedriver path")?;

            caps.insert("platformName".to_string(), json!(DEVICE_OS));
            caps.insert("appium:automationName".to_string(), json!("UiAutomator2"));
            caps.insert(
                "appium:chromedriverExecutable".to_string(),
                json!(chromedriver_str),
            );
        }
        "firefox" => {
            let host_os = match HOST_OS {
                "macos" => "mac",
                other => other,
            };
            caps.insert("platformName".to_string(), json!(host_os));
            caps.insert("appium:automationName".to_string(), json!("Gecko"));
            caps.insert(
                "moz:firefoxOptions".to_string(),
                json!({
                    "androidPackage": "org.mozilla.firefox",
                }),
            );
        }
        "safari" => {
            caps.insert("platformName".to_string(), json!(DEVICE_OS));
            caps.insert("port".to_string(), json!(APPIUM_PORT));
            caps.insert("startIWDP".to_string(), json!(true));
            caps.insert("appium:udid".to_string(), json!(DEVICE_UDID));
            caps.insert("appium:automationName".to_string(), json!("XCUITest"));
            caps.insert("appium:browserName".to_string(), json!(browser));
            caps.insert("appium:deviceName".to_string(), json!("iPhone"));
            caps.insert("appium:platformVersion".to_string(), json!(IOS_VERSION));
            caps.insert("appium:noReset".to_string(), json!(true));
            caps.insert("appium:xcodeOrgId".to_string(), json!(*DEVELOPMENT_TEAM));
            caps.insert(
                "appium:xcodeSigningId".to_string(),
                json!("Developer ID Application"),
            );
            caps.insert("appium:updatedWDABundleId".to_string(), json!(IDENTIFIER));
            caps.insert("appium:useNewWDA".to_string(), json!(true));
        }
        _ => return Err("Unsupported browser".to_string()),
    };

    WebDriver::new(&*APPIUM_SERVER_URL, caps)
        .await
        .map_err(|e| format!("Failed to start WebDriver: {}", e))
}
