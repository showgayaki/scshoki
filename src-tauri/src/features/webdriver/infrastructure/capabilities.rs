use log::{debug, info};
use serde_json::json;
use std::env;
use std::sync::LazyLock;
use thirtyfour::prelude::*;

use crate::constants::{APPIUM_PORT, HOST_OS};

use super::super::constants::CHROME_DRIVER_PATH;

pub async fn android_capabilities(browser: &str, device_os: &str) -> Result<Capabilities, String> {
    info!("Creating WebDriver Android capabilities for {}", browser);

    let mut caps = Capabilities::new();
    caps.insert("browserName".to_string(), json!(browser));

    match browser {
        "chrome" => {
            caps.insert("platformName".to_string(), json!(device_os));
            caps.insert("appium:automationName".to_string(), json!("UiAutomator2"));
            caps.insert(
                "appium:chromedriverExecutable".to_string(),
                json!(*CHROME_DRIVER_PATH),
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
        _ => return Err("Unsupported browser".to_string()),
    };

    debug!("WebDriver Android capabilities: {:?}", caps);
    Ok(caps)
}

pub fn ios_capabilities(
    device_os: &str,
    device_udid: &str,
    ios_version: &str,
    bundle_id: &str,
) -> Result<Capabilities, String> {
    info!("Creating WebDriver iOS capabilities");

    const WDA_IDENTIFIER: &str = "com.facebook.WebDriverAgentRunner";
    static DEVELOPMENT_TEAM: LazyLock<String> =
        LazyLock::new(|| env::var("DEVELOPMENT_TEAM").unwrap_or_else(|_| "Unknown".to_string()));

    let mut caps = Capabilities::new();

    caps.insert("appium:automationName".to_string(), json!("XCUITest"));
    caps.insert("platformName".to_string(), json!(device_os));
    caps.insert("port".to_string(), json!(APPIUM_PORT));
    caps.insert("startIWDP".to_string(), json!(true));
    caps.insert("appium:udid".to_string(), json!(device_udid));
    caps.insert("appium:deviceName".to_string(), json!("iPhone"));
    caps.insert("appium:platformVersion".to_string(), json!(ios_version));
    caps.insert("appium:noReset".to_string(), json!(true));
    caps.insert("appium:xcodeOrgId".to_string(), json!(*DEVELOPMENT_TEAM));
    caps.insert(
        "appium:xcodeSigningId".to_string(),
        json!("Developer ID Application"),
    );
    caps.insert(
        "appium:updatedWDABundleId".to_string(),
        json!(&WDA_IDENTIFIER),
    );
    caps.insert(
        "appium:additionalWebviewBundleIds".to_string(),
        json!([bundle_id]),
    );

    caps.insert("appium:autoWebview".to_string(), json!(false));
    caps.insert("appium:useNewWDA".to_string(), json!(true));

    Ok(caps)
}
