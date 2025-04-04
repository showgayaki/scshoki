use log::{debug, info};
use serde_json::json;
use thirtyfour::prelude::*;

use crate::config::constants::HOST_OS;
use crate::setup::ensure::ensure_chromedriver;

pub async fn capabilities(
    browser: &str,
    device_os: Option<String>,
) -> Result<Capabilities, String> {
    info!("Creating WebDriver Android capabilities for {}", browser);

    let mut caps = Capabilities::new();
    caps.insert("browserName".to_string(), json!(browser));

    match browser {
        "chrome" => {
            let chromedriver_path = ensure_chromedriver()?;
            let chromedriver_str = chromedriver_path
                .to_str()
                .ok_or("Invalid chromedriver path")?;

            caps.insert("platformName".to_string(), json!(device_os));
            caps.insert("appium:automationName".to_string(), json!("UiAutomator2"));
            caps.insert(
                "appium:chromedriverExecutable".to_string(),
                json!(chromedriver_str),
            );
        }
        "firefox" => {
            let host_os = match HOST_OS {
                "macos" => "mac",
                other => other,
            };
            caps.insert("platformName".to_string(), json!(host_os));
            caps.insert("appium:automationName".to_string(), json!("Gecko"));
            caps.insert(
                "moz:firefoxOptions".to_string(),
                json!({
                    "androidPackage": "org.mozilla.firefox",
                }),
            );
        }
        _ => return Err("Unsupported browser".to_string()),
    };

    debug!("WebDriver Android capabilities: {:?}", caps);
    Ok(caps)
}
