use log::{debug, error, info};
use thirtyfour::error::WebDriverErrorInfo;
use thirtyfour::prelude::*;
use url::Url;

use crate::types::screenshot::WebdriverParams;
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

    let page_path = page_url.path().to_string();
    // `/hoge/fuga/` を `hoge-fuga` の形に変換(ファイル名用)
    let path_for_filename = page_path
        .trim_start_matches('/')
        .trim_end_matches('/')
        .replace("/", "-");

    info!("Formatted URL: {}", page_url);
    info!("Formatted path: {}", page_path);

    Ok(PageContext {
        url: page_url.to_string(),
        path: page_path,
        path_for_filename,
    })
}

pub async fn goto_and_wait(driver: &WebDriver, url: &str) -> WebDriverResult<()> {
    debug!("goto_and_wait called with URL: {}", url);
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
    webdriver_params: WebdriverParams<'_>,
) -> Result<DriverContext, String> {
    let WebdriverParams {
        appium_server_url,
        device_os,
        device_os_version,
        device_udid,
        browser,
        base_url,
        token,
    } = webdriver_params;

    info!("Creating WebDriver for {}", browser);
    // OSごとのWebDriverを取得
    let (driver, navigationbar_height) = match device_os {
        // iOSの場合は最初に`NATIVE_APP`としてブラウザを開いておく必要がある
        "iOS" => {
            let bundle_id = WEBVIEW_BUNDLE_IDS
                .get(browser)
                .ok_or_else(|| format!("No bundle ID found for browser: {}", browser))?;

            let caps = ios_capabilities(device_os, device_udid, device_os_version, bundle_id)?;
            debug!("WebDriver capabilities: {:?}", caps);

            let mut driver = webdriver(appium_server_url, caps.clone()).await?;

            let formated_url = format_base_url(base_url, browser);
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
            driver = switch_to_target_context(browser, &driver, appium_server_url, &caps).await?;

            // キャンセルチェック
            check_cancellation(token, Some(&driver)).await?;

            (driver, navigationbar_height)
        }
        "Android" => {
            let caps = android_capabilities(browser, device_os).await?;

            debug!("WebDriver capabilities: {:?}", caps);
            let driver = webdriver(appium_server_url, caps.clone()).await?;
            // キャンセルチェック
            check_cancellation(token, Some(&driver)).await?;

            if let Err(e) = goto_and_wait(&driver, base_url).await {
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
