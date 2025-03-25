use log::info;
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::fs;

use crate::config::constants::{
    BINARY_DIR, CHROMEDRIVER_VERSION_URL, GECKODRIVER_LATEST_RELEASE_URL,
};

/// `~/.scshoki/bin/` の存在確認＆なければ作成
pub fn init_binaries() -> Result<(), String> {
    if !BINARY_DIR.exists() {
        fs::create_dir_all(&*BINARY_DIR)
            .map_err(|e| format!("Failed to create binaries dir: {}", e))?;
        info!("Created binaries directory at {:?}", BINARY_DIR);
    }

    Ok(())
}

/// ChromeDriverの最新バージョン取得
fn get_latest_chromedriver_version() -> Result<String, String> {
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

/// GeckoDriver の最新バージョンを取得
fn get_latest_geckodriver_version() -> Result<String, String> {
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

/// ChromeDriverのURL取得
pub fn get_chromedriver_url() -> Result<String, String> {
    let latest_version = get_latest_chromedriver_version()?;
    // プラットフォームの取得
    let platform = match (env::consts::OS, env::consts::ARCH) {
        ("windows", "x86") => "win32",
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

/// GeckoDriver の URL を取得
pub fn get_geckodriver_url() -> Result<String, String> {
    let latest_version = get_latest_geckodriver_version()?;
    let (platform, ext) = match (env::consts::OS, env::consts::ARCH) {
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
