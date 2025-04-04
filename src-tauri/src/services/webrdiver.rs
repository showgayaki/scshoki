use log::{debug, info};
use thirtyfour::prelude::*;

use crate::config::constants::{APPIUM_SERVER_URL, DEVICE_OS, DEVICE_UDID, IOS_VERSION};
use crate::services::capabilities::android::capabilities as android_capabilities;
use crate::services::capabilities::ios::capabilities as ios_capabilities;

pub async fn create_webdriver(browser: &str) -> Result<WebDriver, String> {
    info!("Creating WebDriver for {}", browser);
    let device_os = DEVICE_OS.lock().unwrap().clone();

    // OSごとのCapabilitiesを取得
    let caps = match device_os.as_deref() {
        Some("iOS") => {
            // iOSの場合はUDIDとiOSバージョンを取得
            let device_udid = DEVICE_UDID.lock().unwrap().clone();
            let ios_version = IOS_VERSION.lock().unwrap().clone();

            ios_capabilities(device_os, device_udid, ios_version)
                .await
                .map_err(|e| format!("Failed to create iOS capabilities: {}", e))?
        }
        Some("Android") => android_capabilities(browser, device_os)
            .await
            .map_err(|e| format!("Failed to create Android capabilities: {}", e))?,
        _ => return Err("Unsupported device OS".to_string()),
    };

    debug!("WebDriver capabilities: {:?}", caps);
    WebDriver::new(&*APPIUM_SERVER_URL, caps)
        .await
        .map_err(|e| format!("Failed to start WebDriver: {}", e))
}
