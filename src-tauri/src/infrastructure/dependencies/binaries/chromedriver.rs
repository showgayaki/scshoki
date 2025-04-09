use log::{debug, error, info};
use reqwest::Client;
use serde_json::Value;

use crate::config::constants::paths::{BINARY_DIR, CHROME_DRIVER_PATH};
use crate::config::env::{HOST_ARCH, HOST_OS};
use crate::infrastructure::archive::extract;
use crate::infrastructure::fs::{remove_file, set_executable};
use crate::infrastructure::network::download_file;

/// `chromedriver` があるか確認し、なければダウンロード
pub async fn install() -> Result<(), String> {
    let url = download_url().await?;
    info!("Downloading ChromeDriver from {:?}", url);
    let dest_path = BINARY_DIR.join(url.split('/').last().unwrap());

    match download_file(&url, &dest_path).await {
        Ok(archive_path) => {
            info!("Successfully downloaded ChromeDriver to {:?}", archive_path);

            if let Err(e) = extract(&archive_path, &BINARY_DIR) {
                let error = format!("Failed to extract ChromeDriver: {}", e);
                return Err(error);
            } else {
                // macOS の場合は chmod +x
                if HOST_OS != "windows" {
                    let exec_path = BINARY_DIR.join("chromedriver");
                    set_executable(&exec_path).expect("Failed to set executable permissions");
                }
                info!("ChromeDriver installed at {:?}", &CHROME_DRIVER_PATH);
                // アーカイブ削除
                match remove_file(&archive_path) {
                    Ok(()) => info!("Removed: {:?}", archive_path),
                    Err(ref e) => error!("Failed to remove {:?}: {}", archive_path, e),
                }
            }
        }
        Err(e) => return Err(format!("Failed to download ChromeDriver: {}", e)),
    }

    Ok(())
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

    info!("Download URL: {}", download_url);

    Ok(download_url)
}
