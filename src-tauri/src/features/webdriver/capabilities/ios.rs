use log::info;
use serde_json::json;
use thirtyfour::prelude::*;

use crate::constants::{APPIUM_PORT, DEVELOPMENT_TEAM};
use crate::features::appium::constants::WDA_IDENTIFIER;

pub fn capabilities(
    device_os: &str,
    device_udid: &str,
    ios_version: &str,
    bundle_id: &str,
) -> Result<Capabilities, String> {
    info!("Creating WebDriver iOS capabilities");
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
