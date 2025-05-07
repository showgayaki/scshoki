use log::debug;
use reqwest::Client;
use serde_json::Value;
use tokio::task::spawn_blocking;

use crate::constants::{HOST_ARCH, HOST_OS};

use super::super::infrastructure::install_binary;

pub async fn install() -> Result<(), String> {
    let binary_name = "geckodriver";
    let url = download_url().await?;

    return spawn_blocking(move || install_binary(binary_name.to_string(), url))
        .await
        .map_err(|e| format!("Task failed: {:?}", e))?
        .await;
}

/// Doanload URLを取得
async fn download_url() -> Result<String, String> {
    let latest_version = get_latest_version().await?;
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

/// GeckoDriverの最新バージョンを取得
async fn get_latest_version() -> Result<String, String> {
    const GECKODRIVER_LATEST_RELEASE_URL: &str =
        "https://api.github.com/repos/mozilla/geckodriver/releases/latest";

    debug!("Creating HTTP client...");
    let client = Client::new();
    debug!("Fetching data from {}", GECKODRIVER_LATEST_RELEASE_URL);

    let response = client
        .get(GECKODRIVER_LATEST_RELEASE_URL)
        .header("User-Agent", "scshoki-app") // GitHub API には User-Agent が必須
        .send()
        .await
        .map_err(|e| format!("Failed to fetch GeckoDriver version: {}", e))?;

    let json: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    let latest_version = json["tag_name"]
        .as_str()
        .ok_or("Failed to extract GeckoDriver version")?
        .to_string();

    Ok(latest_version)
}
