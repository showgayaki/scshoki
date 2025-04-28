use log::{debug, error, info};
use thirtyfour::prelude::*;

use super::capabilities::android::capabilities as android_capabilities;
use super::capabilities::ios::capabilities as ios_capabilities;
use super::capabilities::ios::capabilities_first_open as ios_capabilities_first_open;
use super::constants::NAVIGATION_ELEMTNT_FOR_HEIGHT;
use super::url::format_url;

use crate::constants::APPIUM_SERVER_URL;
use crate::features::device::constants::{DEVICE_OS, IDEVICE_OS_VERSION, IDEVICE_UDID};
use crate::utils::wait::wait_ms;

pub struct DriverContext {
    pub driver: WebDriver,
    pub navigationbar_height: f64,
}

pub async fn create_webdriver(browser: &str, url: &str) -> Result<DriverContext, String> {
    info!("Creating WebDriver for {}", browser);
    let device_os = DEVICE_OS.lock().unwrap().clone();

    // OSごとのWebDriverを取得
    let (driver, navigationbar_height) = match device_os.as_str() {
        // iOSの場合は最初にページを開いておく必要がある
        "iOS" => {
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

            // ブラウザ下部のナビゲーションバーの高さを取得
            // ブラウザが開くまでちょっと待たないと取れない模様
            let navigationbar_height = get_navigationbar_height(&driver_first_open, browser).await;

            // Appiumセッションを終了
            if let Err(e) = driver_first_open.quit().await {
                error!("Failed to quit session: {}", e);
            }

            let caps = ios_capabilities(&device_os, &device_udid, &ios_version)?;
            debug!("WebDriver capabilities: {:?}", caps);

            let driver = webrdiver(caps).await?;
            (driver, navigationbar_height)
        }
        "Android" => {
            let caps = android_capabilities(browser, &device_os).await?;
            debug!("WebDriver capabilities: {:?}", caps);
            let driver = webrdiver(caps).await?;

            driver
                .goto(url)
                .await
                .map_err(|e| format!("Failed to navigate to URL: {}", e))?;

            (driver, 0.0)
        }
        _ => return Err("Unsupported device OS".to_string()),
    };

    Ok(DriverContext {
        driver,
        navigationbar_height,
    })
}

async fn create_webdriver_first_open(
    device_os: &str,
    device_udid: &str,
    ios_version: &str,
) -> Result<WebDriver, String> {
    info!("Creating WebDriver for first open");
    let caps = ios_capabilities_first_open(device_os, device_udid, ios_version)
        .map_err(|e| format!("Failed to create iOS capabilities: {}", e))?;

    debug!("WebDriver capabilities: {:?}", caps);
    webrdiver(caps).await
}

async fn webrdiver(caps: Capabilities) -> Result<WebDriver, String> {
    WebDriver::new(&*APPIUM_SERVER_URL, caps)
        .await
        .map_err(|e| format!("Failed to start WebDriver: {}", e))
}

async fn get_navigationbar_height(driver: &WebDriver, browser: &str) -> f64 {
    const RETRY_COUNT: u32 = 3;

    if let Some(element) = NAVIGATION_ELEMTNT_FOR_HEIGHT.get(browser) {
        for _ in 0..RETRY_COUNT {
            if let Ok(source) = driver.source().await {
                debug!("Page Source:\n{}", source);
            } else {
                error!("Failed to get page source");
            }
            debug!("Get {} height on {}", element.identifier, browser);

            match driver.find(By::Id(element.identifier)).await {
                Ok(found_element) => match found_element.rect().await {
                    Ok(element_rect) => {
                        debug!(
                            "{} Rect - x: {}, y: {}, width: {}, height: {}",
                            element.identifier,
                            element_rect.x,
                            element_rect.y,
                            element_rect.width,
                            element_rect.height,
                        );
                        return element_rect.height;
                    }
                    Err(e) => {
                        error!("Error occurred while getting rect: {}", e);
                    }
                },
                Err(e) => {
                    error!("Error occurred while finding element: {}", e);
                }
            }
            wait_ms(300).await;
        }
        element.default_height
    } else {
        error!("No identifier found for browser: {}", browser);
        0.0
    }
}
