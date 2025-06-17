use chrono::Local;
use log::{debug, error, info};
use std::fs;
use tauri::AppHandle;
use tokio_util::sync::CancellationToken;

use crate::constants::{
    APPIUM_SERVER_URL, DEVICE_OS, IDEVICE_OS_VERSION, IDEVICE_UDID, SCREENSHOT_DIR,
};
use crate::features::device::services::get_display_info;
use crate::features::screenshot::constants::{status_messages, CANCEL_TOKEN};
use crate::features::screenshot::services::{
    combine_screenshots, notify_screenshot_status, screenshot_full_page,
};
use crate::features::webdriver::services::{create_webdriver, format_url, goto_and_wait};
use crate::types::screenshot::{
    ScreenshotContext, ScreenshotParams, ScreenshotResponse, WebdriverParams,
};

pub async fn take_screenshot(
    app_handle: &AppHandle,
    params: ScreenshotParams,
) -> Result<ScreenshotResponse, String> {
    let token = CancellationToken::new();
    {
        let mut token_lock = CANCEL_TOKEN.lock().unwrap();
        *token_lock = Some(token.clone());
    }

    debug!("take_screenshot called with params: {:?}", params);

    let ScreenshotParams {
        base_url,
        target_page_paths,
        hidden_elements,
        selected_browsers,
        basic_auth_username,
        basic_auth_password,
    } = params;

    info!(
        "Taking screenshot of {} for browsers: {:?}",
        base_url, selected_browsers
    );

    if selected_browsers.is_empty() {
        return Err("No browsers selected for screenshot".to_string());
    }

    let datetime_now = Local::now().format("%Y%m%d-%H%M%S").to_string();

    for browser in selected_browsers {
        info!("Starting screenshot process for {}", browser);
        let device_os = DEVICE_OS.lock().unwrap().clone();

        let browser_lower = browser.to_lowercase();
        let device_os_version = IDEVICE_OS_VERSION.lock().unwrap().clone();
        let device_udid = IDEVICE_UDID.lock().unwrap().clone();
        let webdriver_params = WebdriverParams {
            appium_server_url: &APPIUM_SERVER_URL,
            device_os: device_os.as_str(),
            device_os_version: device_os_version.as_str(),
            device_udid: device_udid.as_str(),
            browser: browser_lower.as_str(),
            base_url: base_url.as_str(),
            basic_auth_username: basic_auth_username.as_deref().unwrap_or(""),
            basic_auth_password: basic_auth_password.as_deref().unwrap_or(""),
            token: &token,
        };

        notify_screenshot_status(app_handle, status_messages::CREATING_WEBDRIVER);
        match create_webdriver(webdriver_params).await {
            Ok(driver_context) => {
                notify_screenshot_status(app_handle, status_messages::CREATED_WEBDRIVER);
                let driver = driver_context.driver;
                // Density取得
                notify_screenshot_status(app_handle, status_messages::GETTING_DISPLAY_INFO);
                get_display_info(&driver, &device_os).await;

                for (i, page_path) in target_page_paths.iter().enumerate() {
                    let page_context = format_url(&base_url, page_path)?;
                    info!("Target URL: {}", page_context.url);

                    let page_url = page_context.url.as_str();
                    notify_screenshot_status(
                        app_handle,
                        status_messages::taking(&browser, &page_context.path),
                    );

                    // 最初が"/"の時は、driverの作成時にすでに開いているのでgoto()しない
                    if i == 0 && page_path == "/" {
                        {} // 何もしない
                    } else {
                        info!("[{}] Navigating to {}", browser, page_url);
                        if let Err(e) = goto_and_wait(&driver, page_url).await {
                            error!("Failed to navigate to {}: {}", page_context.url, e);
                        }
                    }

                    let screenshot_context = ScreenshotContext {
                        driver: &driver,
                        hidden_elements: &hidden_elements,
                        datetime_now: &datetime_now,
                        device_os: &device_os,
                        browser: &browser,
                        page_path: &page_context.path_for_filename,
                        navigationbar_height: driver_context.navigationbar_height,
                        token: &token,
                    };

                    // スクロールしながらスクリーンショットを撮影
                    match screenshot_full_page(screenshot_context).await {
                        Ok(screenshots) => {
                            notify_screenshot_status(app_handle, status_messages::COMBINING);
                            let final_screenshot = match combine_screenshots(&screenshots) {
                                Ok(img) => {
                                    notify_screenshot_status(
                                        app_handle,
                                        status_messages::success(&browser, page_path),
                                    );
                                    img
                                }
                                Err(e) => {
                                    notify_screenshot_status(
                                        app_handle,
                                        status_messages::error(&browser, page_path),
                                    );
                                    error!("[{}] Failed to combine screenshots: {}", browser, e);
                                    continue;
                                }
                            };

                            let screenshot_path = SCREENSHOT_DIR.join(format!(
                                "{}_{}_{}_{}_full.png",
                                datetime_now, device_os, browser, page_context.path_for_filename
                            ));
                            if let Err(e) = fs::write(&screenshot_path, final_screenshot) {
                                error!("[{}] Failed to save screenshot: {}", browser, e);
                            } else {
                                info!("[{}] Screenshot saved at {:?}", browser, screenshot_path);
                            }
                        }
                        Err(e) => {
                            notify_screenshot_status(
                                app_handle,
                                status_messages::error(&browser, page_path),
                            );
                            error!("[{}] Failed to capture screenshots: {}", browser, e);
                            // キャンセルチェック
                            if token.is_cancelled() {
                                return Ok(ScreenshotResponse {
                                    success: false,
                                    cancelled: token.is_cancelled(),
                                    path: "".to_string(),
                                    error: Some(e),
                                });
                            }
                        }
                    }
                }

                // セッションを終了
                if let Err(e) = driver.quit().await {
                    error!("[{}] Failed to quit session: {}", browser, e);
                }
            }
            Err(e) => {
                error!("[{}] WebDriver creation failed: {}", browser, e);
                // キャンセルチェック
                if token.is_cancelled() {
                    return Ok(ScreenshotResponse {
                        success: false,
                        cancelled: token.is_cancelled(),
                        path: "".to_string(),
                        error: Some(e),
                    });
                }
            }
        }
    }

    Ok(ScreenshotResponse {
        success: true,
        cancelled: token.is_cancelled(),
        path: "".to_string(),
        error: None,
    })
}
