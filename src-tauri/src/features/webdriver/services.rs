use log::{debug, error, info};
use serde_json::{Map, Value};
use thirtyfour::error::WebDriverErrorInfo;
use thirtyfour::prelude::*;
use tokio_util::sync::CancellationToken;

use crate::constants::{APPIUM_SERVER_URL, DEVICE_OS, IDEVICE_OS_VERSION, IDEVICE_UDID};
use crate::utils::wait::wait_for_page_load;

use super::constants::WEBVIEW_BUNDLE_IDS;
use super::infrastructure::capabilities::{android_capabilities, ios_capabilities};
use super::infrastructure::context::{get_contexts, set_context};
use super::infrastructure::navigationbar::get_navigationbar_height;
use super::infrastructure::url::format_url;
use super::infrastructure::webdriver::webdriver;
use crate::utils::cancel::check_cancellation;

pub struct DriverContext {
    pub driver: WebDriver,
    pub navigationbar_height: f64,
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
    token: CancellationToken,
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

            let formated_url = format_url(url, browser);
            info!("Formatted URL: {}", formated_url);
            driver
                .goto(&formated_url)
                .await
                .map_err(|e| format!("Failed to navigate to URL: {}", e))?;

            // キャンセルチェック
            check_cancellation(&token, Some(&driver)).await?;

            // ブラウザ下部のナビゲーションバーの高さを取得
            let navigationbar_height = get_navigationbar_height(&driver, browser).await;
            debug!("Navigation bar height: {}", navigationbar_height);

            // キャンセルチェック
            check_cancellation(&token, Some(&driver)).await?;

            // コンテキストを適切なWEBVIEWに切り替える
            driver = switch_to_target_context(browser, &driver, &APPIUM_SERVER_URL, caps).await?;

            // キャンセルチェック
            check_cancellation(&token, Some(&driver)).await?;

            (driver, navigationbar_height)
        }
        "Android" => {
            let caps = android_capabilities(browser, &device_os).await?;

            debug!("WebDriver capabilities: {:?}", caps);
            let driver = webdriver(&APPIUM_SERVER_URL, caps).await?;
            // キャンセルチェック
            check_cancellation(&token, Some(&driver)).await?;

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

/// 指定したURLに一致するWEBVIEW contextを優先的に選び、なければ最大IDのWEBVIEWを返す
async fn switch_to_target_context(
    browser: &str,
    driver: &WebDriver,
    appium_server_url: &str,
    caps: Map<String, Value>,
) -> Result<WebDriver, String> {
    debug!("Selecting best context for browser: {}", browser);

    let mut driver = driver.clone();
    let mut session_id = driver.session_id().to_string();
    let contexts = get_contexts(&session_id, appium_server_url)
        .await
        .map_err(|e| e.to_string())?;
    info!("Available contexts: {:?}", contexts);

    // Firefoxの場合は、同じURLが開かれているタブがあるときに
    // 新しくタブを開かずにそのタブを使用されるため、今回開かれたアクティブなタブを探す
    if browser == "firefox" {
        // コンテキスト切り替えできるものがアクティブ
        for context in contexts {
            if context.starts_with("WEBVIEW_") {
                debug!("Context: {}", context);
                let caps_clone = caps.clone();

                // gotoでページを開いたばっかりなので、set_contextできたタブが
                // テストするページが開かれたタブのはず
                let is_error = match set_context(&session_id, &APPIUM_SERVER_URL, &context).await {
                    Ok(()) => {
                        break;
                    }
                    Err(e) => {
                        error!("set_context failed for {}: {}", context, e);
                        true
                    }
                };

                if is_error {
                    // コンテキスト切り替えに失敗した場合は、セッションを削除して再作成
                    debug!("Deleting session: {}", session_id);
                    if let Err(e) = driver.quit().await {
                        error!("Failed to quit driver: {}", e);
                    }
                    let new_driver = webdriver(appium_server_url, caps_clone).await?;
                    driver = new_driver;
                    session_id = driver.session_id().to_string();
                    debug!("Sesssion recreated: {}", session_id);

                    // 一度get_contextsを実行しないと、次のset_contextで失敗するっぽい
                    let _ = get_contexts(&session_id, &APPIUM_SERVER_URL).await;

                    continue;
                }
            }
        }
    } else {
        // Firefox 以外は新しいタブで開かれるので最大 page_id の WEBVIEW を使用
        let context = contexts
            .iter()
            .filter(|c| c.starts_with("WEBVIEW_"))
            .max_by_key(|c| {
                c.split('.')
                    .nth(1)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0)
            })
            .cloned()
            .ok_or("No valid WEBVIEW context found".to_string())?;

        let _ = set_context(&session_id, &APPIUM_SERVER_URL, &context).await;
    }
    Ok(driver)
}
