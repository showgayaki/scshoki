use log::{error, info};
use reqwest::blocking::Client;
use serde_json::Value;
use std::path::PathBuf;

use crate::config::constants::paths::BINARY_DIR;
use crate::config::env::{HOST_ARCH, HOST_OS};
use crate::infrastructure::archive::extract;
use crate::infrastructure::fs::{remove_file, set_executable};
use crate::infrastructure::network::download_file;

/// `chromedriver` があるか確認し、なければダウンロード
pub fn check_or_install() -> Result<PathBuf, String> {
    let chromedriver_path = BINARY_DIR.join("chromedriver");

    if chromedriver_path.exists() {
        info!("ChromeDriver is already installed: {:?}", chromedriver_path);
        return Ok(chromedriver_path);
    }

    let url = download_url()?;
    let dest_path = BINARY_DIR.join(url.split('/').last().unwrap());
    info!("Downloading ChromeDriver from {:?}", url);

    match download_file(&url, &dest_path) {
        Ok(archive_path) => {
            info!("Successfully downloaded ChromeDriver to {:?}", archive_path);

            if let Err(e) = extract(&archive_path, &BINARY_DIR) {
                return Err(format!("Failed to extract ChromeDriver: {}", e));
            } else {
                // macOS の場合は chmod +x
                if HOST_OS != "windows" {
                    let exec_path = BINARY_DIR.join("chromedriver");
                    set_executable(&exec_path).expect("Failed to set executable permissions");
                }
                info!("ChromeDriver installed at {:?}", chromedriver_path);
                // アーカイブ削除
                match remove_file(&archive_path) {
                    Ok(()) => info!("Removed: {:?}", archive_path),
                    Err(ref e) => error!("Failed to remove {:?}: {}", archive_path, e),
                }
            }
        }
        Err(e) => return Err(format!("Failed to download ChromeDriver: {}", e)),
    }

    Ok(chromedriver_path)
}

/// ChromeDriverの最新バージョン取得
fn get_latest_version() -> Result<String, String> {
    const CHROMEDRIVER_VERSION_URL: &str = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";

    let client = Client::new();
    let response = client
        .get(CHROMEDRIVER_VERSION_URL)
        .send()
        .map_err(|e| format!("Failed to fetch ChromeDriver version: {}", e))?;

    let json: Value = response
        .json()
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    let latest_version = json["channels"]["Stable"]["version"]
        .as_str()
        .ok_or("Failed to extract ChromeDriver version")?
        .to_string();

    Ok(latest_version)
}

/// Doanload URLを取得
fn download_url() -> Result<String, String> {
    let latest_version = get_latest_version()?;
    // プラットフォームの取得
    let platform = match (HOST_OS, HOST_ARCH) {
        ("windows", "x86_64") => "win64",
        ("macos", "x86_64") => "mac-x64",
        ("macos", "aarch64") => "mac-arm64",
        _ => panic!("Unsupported platform"),
    };

    Ok(format!(
        "https://storage.googleapis.com/chrome-for-testing-public/{}/{}/chromedriver-{}.zip",
        latest_version, platform, platform
    ))
}
