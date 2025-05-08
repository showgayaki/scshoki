use log::{debug, error};
use thirtyfour::prelude::*;

use crate::utils::wait::wait_ms;

use super::super::constants::NAVIGATION_ELEMTNT_FOR_HEIGHT;

pub async fn get_navigationbar_height(driver: &WebDriver, browser: &str) -> f64 {
    const RETRY_COUNT: u32 = 3;

    if let Some(element) = NAVIGATION_ELEMTNT_FOR_HEIGHT.get(browser) {
        for _ in 0..RETRY_COUNT {
            if let Ok(_source) = driver.source().await {
                // debug!("Page Source:\n{}", source);
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
