use log::{debug, info};
use serde_json::json;
use thirtyfour::prelude::*;

use crate::config::constants::{APPIUM_PORT, DEVELOPMENT_TEAM};

pub async fn capabilities(
    device_os: Option<String>,
    device_udid: Option<String>,
    ios_version: Option<String>,
) -> Result<Capabilities, String> {
    info!("Creating WebDriver iOS capabilities");
    let mut caps = Capabilities::new();

    caps.insert("appium:automationName".to_string(), json!("XCUITest"));
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
    caps.insert("appium:useNewWDA".to_string(), json!(true));
    caps.insert(
        "appium:updatedWDABundleId".to_string(),
        json!("com.facebook.WebDriverAgentRunner"),
        // json!("com.google.chrome.ios"),
    );
    caps.insert(
        "appium:additionalWebviewBundleIds".to_string(),
        json!([
            "com.apple.mobilesafari",
            "com.google.chrome.ios",
            "org.mozilla.ios.Firefox",
        ]),
    );
    caps.insert("appium:autoWebview".to_string(), json!(true));

    debug!("WebDriver capabilities: {:?}", caps);
    Ok(caps)
}
