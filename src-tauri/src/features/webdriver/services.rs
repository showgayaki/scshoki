use log::{debug, error, info};
use thirtyfour::error::WebDriverErrorInfo;
use thirtyfour::prelude::*;
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::constants::{APPIUM_SERVER_URL, DEVICE_OS, IDEVICE_OS_VERSION, IDEVICE_UDID};
use crate::utils::cancel::check_cancellation;
use crate::utils::wait::wait_for_page_load;

use super::constants::WEBVIEW_BUNDLE_IDS;
use super::infrastructure::capabilities::{android_capabilities, ios_capabilities};
use super::infrastructure::context::switch_to_target_context;
use super::infrastructure::navigationbar::get_navigationbar_height;
use super::infrastructure::url::format_base_url;
use super::infrastructure::webdriver::webdriver;

pub struct DriverContext {
    pub driver: WebDriver,
    pub navigationbar_height: f64,
}

pub struct PageContext {
    pub url: String,
    pub path: String,
    pub path_for_filename: String,
}

pub fn format_url(base_url: &str, path: &str) -> Result<PageContext, String> {
    let parsed_url = Url::parse(base_url).expect("Invalid base URL");
    let page_url = if path == "/" {
        parsed_url
    } else {
        parsed_url.join(path).expect("Invalid path for URL join")
    };

    let path = page_url.path().to_string();
    // `/hoge/fuga/` を `hoge-fuga` の形に変換(ファイル名用)
    let path_for_filename = path
        .trim_start_matches('/')
        .trim_end_matches('/')
        .replace("/", "-");

    info!("Formatted URL: {}", page_url);
    info!("Formatted path: {}", path);

    Ok(PageContext {
        url: page_url.to_string(),
        path,
        path_for_filename,
    })
}

pub async fn goto_and_wait(driver: &WebDriver, url: &str) -> WebDriverResult<()> {
    driver.goto(url).await?;
    if let Err(e) = wait_for_page_load(driver, url).await {
        error!("Failed to wait for page load: {}", e);
        return Err(WebDriverError::UnknownError(WebDriverErrorInfo::new(
            e.to_string(),
        )));
    }
    Ok(())
}

pub async fn create_webdriver(
    browser: &str,
    url: &str,
    token: &CancellationToken,
) -> Result<DriverContext, String> {
    info!("Creating WebDriver for {}", browser);
    let device_os = DEVICE_OS.lock().unwrap().clone();

    // OSごとのWebDriverを取得
    let (driver, navigationbar_height) = match device_os.as_str() {
        // iOSの場合は最初に`NATIVE_APP`としてブラウザを開いておく必要がある
        "iOS" => {
            // UDIDとiOSバージョン、ブラウザのBundle IDを取得
            let device_udid = IDEVICE_UDID.lock().unwrap().clone();
            let ios_version = IDEVICE_OS_VERSION.lock().unwrap().clone();
            let bundle_id = WEBVIEW_BUNDLE_IDS
                .get(browser)
                .ok_or_else(|| format!("No bundle ID found for browser: {}", browser))?;

            let caps = ios_capabilities(&device_os, &device_udid, &ios_version, bundle_id)?;
            debug!("WebDriver capabilities: {:?}", caps);

            let mut driver = webdriver(&APPIUM_SERVER_URL, caps.clone()).await?;

            let formated_url = format_base_url(url, browser);
            info!("Formatted URL: {}", formated_url);
            driver
                .goto(&formated_url)
                .await
                .map_err(|e| format!("Failed to navigate to URL: {}", e))?;

            // キャンセルチェック
            check_cancellation(token, Some(&driver)).await?;

            // ブラウザ下部のナビゲーションバーの高さを取得
            let navigationbar_height = get_navigationbar_height(&driver, browser).await;
            debug!("Navigation bar height: {}", navigationbar_height);

            // キャンセルチェック
            check_cancellation(token, Some(&driver)).await?;

            // コンテキストを適切なWEBVIEWに切り替える
            driver = switch_to_target_context(browser, &driver, &APPIUM_SERVER_URL, caps).await?;

            // キャンセルチェック
            check_cancellation(token, Some(&driver)).await?;

            (driver, navigationbar_height)
        }
        "Android" => {
            let caps = android_capabilities(browser, &device_os).await?;

            debug!("WebDriver capabilities: {:?}", caps);
            let driver = webdriver(&APPIUM_SERVER_URL, caps).await?;
            // キャンセルチェック
            check_cancellation(token, Some(&driver)).await?;

            if let Err(e) = goto_and_wait(&driver, url).await {
                return Err(format!("Failed to navigate and wait for URL: {}", e));
            }

            (driver, 0.0)
        }
        _ => return Err("Unsupported device OS".to_string()),
    };

    Ok(DriverContext {
        driver,
        navigationbar_height,
    })
}
