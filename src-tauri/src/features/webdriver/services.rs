use log::{debug, error, info};
use thirtyfour::prelude::*;

use super::capabilities::android::capabilities as android_capabilities;
use super::capabilities::ios::capabilities as ios_capabilities;
use super::capabilities::ios::capabilities_first_open as ios_capabilities_first_open;
use super::url::format_url;
use crate::features::appium::constants::APPIUM_SERVER_URL;
use crate::features::device::constants::{DEVICE_OS, IDEVICE_OS_VERSION, IDEVICE_UDID};

pub async fn create_webdriver(browser: &str, url: &str) -> Result<WebDriver, String> {
    info!("Creating WebDriver for {}", browser);
    let device_os = DEVICE_OS.lock().unwrap().clone();

    // OSごとのWebDriverを取得
    let driver = match device_os.as_deref() {
        // iOSの場合は最初にページを開いておく必要がある
        Some("iOS") => {
            // UDIDとiOSバージョンを取得
            let device_udid = IDEVICE_UDID.lock().unwrap().clone();
            let ios_version = IDEVICE_OS_VERSION.lock().unwrap().clone();

            // 最初にWebDriverAgentを起動、ページを開いておく
            let driver_first_open =
                create_webdriver_first_open(&device_os, &device_udid, &ios_version).await?;

            let formated_url = format_url(url, browser);
            info!("Formatted URL: {}", formated_url);
            driver_first_open
                .goto(&formated_url)
                .await
                .map_err(|e| format!("Failed to navigate to URL: {}", e))?;

            // Appiumセッションを終了
            if let Err(e) = driver_first_open.quit().await {
                error!("Failed to quit session: {}", e);
            }

            let caps = ios_capabilities(&device_os, &device_udid, &ios_version).await?;
            debug!("WebDriver capabilities: {:?}", caps);
            webrdiver(caps).await
        }
        Some("Android") => {
            let caps = android_capabilities(browser, &device_os).await?;
            debug!("WebDriver capabilities: {:?}", caps);
            let driver = webrdiver(caps).await?;
            driver
                .goto(url)
                .await
                .map_err(|e| format!("Failed to navigate to URL: {}", e))?;
            Ok(driver)
        }
        _ => return Err("Unsupported device OS".to_string()),
    };

    driver
}

async fn create_webdriver_first_open(
    device_os: &Option<String>,
    device_udid: &Option<String>,
    ios_version: &Option<String>,
) -> Result<WebDriver, String> {
    info!("Creating WebDriver for first open");
    let caps = ios_capabilities_first_open(device_os, device_udid, ios_version)
        .await
        .map_err(|e| format!("Failed to create iOS capabilities: {}", e))?;

    debug!("WebDriver capabilities: {:?}", caps);
    webrdiver(caps).await
}

async fn webrdiver(caps: Capabilities) -> Result<WebDriver, String> {
    WebDriver::new(&*APPIUM_SERVER_URL, caps)
        .await
        .map_err(|e| format!("Failed to start WebDriver: {}", e))
}
