use chrono::Local;
use log::{debug, error, info};
use std::fs;

use crate::constants::{DEVICE_OS, SCREENSHOT_DIR};
use crate::features::device::services::get_display_info;
use crate::features::screenshot::services::{combine_screenshots, screenshot_full_page};
use crate::features::webdriver::services::create_webdriver;
use crate::types::screenshot::ScreenshotParams;

pub async fn take_screenshot(params: ScreenshotParams) -> Result<(), String> {
    debug!("take_screenshot called with params: {:?}", params);

    let ScreenshotParams {
        url,
        target_page_paths,
        hidden_elements,
        selected_browsers,
    } = params;

    info!(
        "Taking screenshot of {} for browsers: {:?}",
        url, selected_browsers
    );

    if selected_browsers.is_empty() {
        return Err("No browsers selected for screenshot".to_string());
    }

    let datetime_now = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let device_os = DEVICE_OS.lock().unwrap().clone();

    // datetime取得
    for browser in selected_browsers {
        info!("Starting screenshot process for {}", browser);
        let browser_lowercased = browser.to_lowercase();

        match create_webdriver(&browser_lowercased, &url).await {
            Ok(driver_context) => {
                let driver = driver_context.driver;
                // Density取得
                get_display_info(&driver, &device_os).await;

                // スクロールしながらスクリーンショットを撮影
                match screenshot_full_page(
                    &driver,
                    &hidden_elements,
                    &datetime_now,
                    &device_os,
                    &browser,
                    driver_context.navigationbar_height,
                )
                .await
                {
                    Ok(screenshots) => {
                        let final_screenshot = match combine_screenshots(&screenshots) {
                            Ok(img) => img,
                            Err(e) => {
                                error!("[{}] Failed to combine screenshots: {}", browser, e);
                                continue;
                            }
                        };

                        let screenshot_path = SCREENSHOT_DIR.join(format!(
                            "{}_{}_{}_full.png",
                            datetime_now, device_os, browser
                        ));
                        if let Err(e) = fs::write(&screenshot_path, final_screenshot) {
                            error!("[{}] Failed to save screenshot: {}", browser, e);
                        } else {
                            info!("[{}] Screenshot saved at {:?}", browser, screenshot_path);
                        }
                    }
                    Err(e) => error!("[{}] Failed to capture screenshots: {}", browser, e),
                }

                // セッションを終了
                if let Err(e) = driver.quit().await {
                    error!("[{}] Failed to quit session: {}", browser, e);
                }
            }
            Err(e) => error!("[{}] WebDriver creation failed: {}", browser, e),
        }
    }

    Ok(())
}
