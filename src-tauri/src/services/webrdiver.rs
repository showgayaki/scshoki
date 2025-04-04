use log::{debug, error, info};
use serde_json::json;
use thirtyfour::prelude::*;

use crate::config::constants::{
    APPIUM_PORT, APPIUM_SERVER_URL, DEVELOPMENT_TEAM, DEVICE_OS, DEVICE_UDID, HOST_OS, IOS_VERSION,
};
use crate::setup::ensure::ensure_chromedriver;

pub async fn create_webdriver(browser: &str) -> Result<WebDriver, String> {
    debug!("Creating WebDriver for {}", browser);

    let device_os = DEVICE_OS.lock().unwrap().clone();
    let device_udid = DEVICE_UDID.lock().unwrap().clone();
    let ios_version = IOS_VERSION.lock().unwrap().clone();

    let mut caps = Capabilities::new();
    caps.insert("browserName".to_string(), json!(browser));

    let automation_name = {
        match device_os.as_deref() {
            Some("iOS") => {
                caps.insert("platformName".to_string(), json!(device_os));
                caps.insert("port".to_string(), json!(APPIUM_PORT));
                // caps.insert("startIWDP".to_string(), json!(true));
                caps.insert("appium:udid".to_string(), json!(device_udid));
                caps.insert("appium:deviceName".to_string(), json!("iPhone"));
                caps.insert("appium:platformVersion".to_string(), json!(ios_version));
                caps.insert("appium:noReset".to_string(), json!(true));
                caps.insert("appium:xcodeOrgId".to_string(), json!(*DEVELOPMENT_TEAM));
                caps.insert(
                    "appium:xcodeSigningId".to_string(),
                    json!("Developer ID Application"),
                );
                // com.facebook.WebDriverAgentRunner.xctrunner
                caps.insert(
                    "appium:updatedWDABundleId".to_string(),
                    json!("com.facebook.WebDriverAgentRunner"),
                    // json!("com.google.chrome.ios"),
                );
                caps.insert(
                    "appium:additionalWebviewBundleIds".to_string(),
                    json!([
                        "com.facebook.WebDriverAgentRunner",
                        "com.google.chrome.ios",
                        "com.apple.mobilesafari"
                    ]),
                );
                caps.insert("appium:autoWebview".to_string(), json!(true));
                "XCUITest"
            }
            Some("Android") => "UiAutomator2",
            _ => return Err("Unsupported device OS".to_string()),
        }
    };

    match browser {
        "chrome" => {
            let chromedriver_path = ensure_chromedriver()?;
            let chromedriver_str = chromedriver_path
                .to_str()
                .ok_or("Invalid chromedriver path")?;

            // caps.insert("platformName".to_string(), json!(device_os));
            caps.insert("appium:automationName".to_string(), json!(automation_name));
            // caps.insert("appium:automationName".to_string(), json!("UiAutomator2"));
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
            caps.insert(
                "appium:bundleId".to_string(),
                json!("com.apple.mobilesafari"),
            );

            // caps.insert("platformName".to_string(), json!(DEVICE_OS));
            // caps.insert("port".to_string(), json!(APPIUM_PORT));
            // caps.insert("startIWDP".to_string(), json!(true));
            // caps.insert("appium:udid".to_string(), json!(DEVICE_UDID));
            // caps.insert("appium:automationName".to_string(), json!(automation_name));
            // // caps.insert("appium:automationName".to_string(), json!("XCUITest"));
            // caps.insert("appium:browserName".to_string(), json!(browser));
            // caps.insert("appium:deviceName".to_string(), json!("iPhone"));
            // caps.insert("appium:platformVersion".to_string(), json!(IOS_VERSION));
            // caps.insert("appium:noReset".to_string(), json!(true));
            // caps.insert("appium:xcodeOrgId".to_string(), json!(*DEVELOPMENT_TEAM));
            // caps.insert(
            //     "appium:xcodeSigningId".to_string(),
            //     json!("Developer ID Application"),
            // );
            // caps.insert("appium:updatedWDABundleId".to_string(), json!(IDENTIFIER));
            // caps.insert("appium:useNewWDA".to_string(), json!(true));
        }
        _ => return Err("Unsupported browser".to_string()),
    };

    debug!("WebDriver capabilities: {:?}", caps);
    WebDriver::new(&*APPIUM_SERVER_URL, caps)
        .await
        .map_err(|e| format!("Failed to start WebDriver: {}", e))
}
