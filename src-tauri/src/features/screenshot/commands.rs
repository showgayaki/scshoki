use log::{error, info};
use std::fs;
use tauri::command;

use super::constants::SCREENSHOT_DIR;
use super::services::{capture_full_page, combine_screenshots};
use crate::features::webdriver::services::create_webdriver;
use crate::utils::wait::wait_for_page_load;

#[command]
pub async fn take_screenshot(
    url: String,
    hidden_elements: String,
    selected_browsers: Vec<String>,
) -> Result<(), String> {
    info!(
        "Taking screenshot of {} for browsers: {:?}",
        url, selected_browsers
    );

    if selected_browsers.is_empty() {
        return Err("No browsers selected for screenshot".to_string());
    }

    for browser in selected_browsers {
        info!("Starting screenshot process for {}", browser);
        let browser_lowercased = browser.to_lowercase();

        match create_webdriver(&browser_lowercased, &url).await {
            Ok(driver) => {
                // ページの完全読み込みを待つ
                if let Err(e) = wait_for_page_load(&driver, &url).await {
                    error!("[{}] Page load error: {}", browser, e);
                    continue;
                }

                // スクロールしながらスクリーンショットを撮影
                match capture_full_page(&driver, &hidden_elements).await {
                    Ok(screenshots) => {
                        let final_screenshot = match combine_screenshots(&screenshots) {
                            Ok(img) => img,
                            Err(e) => {
                                error!("[{}] Failed to combine screenshots: {}", browser, e);
                                continue;
                            }
                        };

                        let screenshot_path =
                            SCREENSHOT_DIR.join(format!("screenshot_{}.png", browser_lowercased));
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
