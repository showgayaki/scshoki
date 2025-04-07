use log::{error, info};
use reqwest::blocking::Client;
use serde_json::Value;
use std::path::PathBuf;

use crate::config::constants::paths::BINARY_DIR;
use crate::config::env::{HOST_ARCH, HOST_OS};
use crate::infrastructure::archive::extract;
use crate::infrastructure::fs::{remove_file, set_executable};
use crate::infrastructure::network::download_file;

/// `geckodriver` があるか確認し、なければダウンロード
pub fn check_or_install() -> Result<PathBuf, String> {
    let geckodriver_path = BINARY_DIR.join("geckodriver");

    if geckodriver_path.exists() {
        info!("GeckoDriver is already installed: {:?}", geckodriver_path);
        return Ok(geckodriver_path);
    }

    let url = download_url()?;
    let dest_path = BINARY_DIR.join(url.split('/').last().unwrap());
    info!("Downloading GeckoDriver from {:?}", url);

    match download_file(&url, &dest_path) {
        Ok(archive_path) => {
            info!("Successfully downloaded GeckoDriver to {:?}", archive_path);

            if let Err(e) = extract(&archive_path, &BINARY_DIR) {
                return Err(format!("Failed to extract GeckoDriver: {}", e));
            } else {
                // macOS の場合は chmod +x
                if HOST_OS != "windows" {
                    let exec_path = BINARY_DIR.join("geckodriver");
                    set_executable(&exec_path).expect("Failed to set executable permissions");
                }
                info!("GeckoDriver installed at {:?}", geckodriver_path);
                // アーカイブ削除
                match remove_file(&archive_path) {
                    Ok(()) => info!("Removed: {:?}", archive_path),
                    Err(ref e) => error!("Failed to remove {:?}: {}", archive_path, e),
                }
            }
        }
        Err(e) => return Err(format!("Failed to download GeckoDriver: {}", e)),
    }

    Ok(geckodriver_path)
}

/// GeckoDriverの最新バージョンを取得
fn get_latest_version() -> Result<String, String> {
    const GECKODRIVER_LATEST_RELEASE_URL: &str =
        "https://api.github.com/repos/mozilla/geckodriver/releases/latest";

    let client = Client::new();
    let response = client
        .get(GECKODRIVER_LATEST_RELEASE_URL)
        .header("User-Agent", "scshoki-app") // GitHub API には User-Agent が必須
        .send()
        .map_err(|e| format!("Failed to fetch GeckoDriver version: {}", e))?;

    let json: Value = response
        .json()
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    let latest_version = json["tag_name"]
        .as_str()
        .ok_or("Failed to extract GeckoDriver version")?
        .to_string();

    Ok(latest_version)
}

/// Doanload URLを取得
fn download_url() -> Result<String, String> {
    let latest_version = get_latest_version()?;
    let (platform, ext) = match (HOST_OS, HOST_ARCH) {
        ("windows", "x86_64") => ("win64", "zip"),
        ("macos", "x86_64") => ("macos", "tar.gz"),
        ("macos", "aarch64") => ("macos-aarch64", "tar.gz"),
        _ => return Err("Unsupported platform".to_string()),
    };

    Ok(format!(
        "https://github.com/mozilla/geckodriver/releases/download/{}/geckodriver-{}-{}.{}",
        latest_version, latest_version, platform, ext
    ))
}
