use log::debug;
use reqwest::Client;
use serde_json::Value;
use tokio::task::spawn_blocking;

use crate::constants::{HOST_ARCH, HOST_OS};

use super::super::infrastructure::install_binary;

pub async fn install() -> Result<(), String> {
    let binary_name = "chromedriver";
    let url = download_url().await?;

    return spawn_blocking(move || install_binary(binary_name.to_string(), url))
        .await
        .map_err(|e| format!("Task failed: {:?}", e))?
        .await;
}

/// Doanload URLを取得
async fn download_url() -> Result<String, String> {
    debug!("Fetching ChromeDriver download URL from JSON");
    const CHROMEDRIVER_VERSION_URL: &str = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";

    debug!("Creating HTTP client...");
    let client = Client::new();
    debug!("Fetching data from {}", CHROMEDRIVER_VERSION_URL);

    let response = client
        .get(CHROMEDRIVER_VERSION_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch ChromeDriver JSON: {}", e))?
        .error_for_status()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let json: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse ChromeDriver JSON: {}", e))?;

    // プラットフォームのマッピング
    let platform = match (HOST_OS, HOST_ARCH) {
        ("windows", "x86_64") => "win64",
        ("macos", "x86_64") => "mac-x64",
        ("macos", "aarch64") => "mac-arm64",
        _ => return Err("Unsupported platform".to_string()),
    };

    debug!("Searching for download URL for platform: {}", platform);

    // `channels.Stable.downloads` 配列から `platform` に一致する URL を取得
    let downloads = json["channels"]["Stable"]["downloads"]["chromedriver"]
        .as_array()
        .ok_or_else(|| {
            "Missing 'channels.Stable.downloads.chromedriver' array in JSON".to_string()
        })?;

    let download_url = downloads
        .iter()
        .find_map(|entry| {
            let entry_platform = entry["platform"].as_str()?;
            let url = entry["url"].as_str()?;
            if entry_platform == platform {
                Some(url.to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| format!("No matching download URL found for platform: {}", platform))?;

    Ok(download_url)
}
