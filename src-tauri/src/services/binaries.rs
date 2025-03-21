use log::{error, info};
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::config::constants::{
    APPIUM_URL, BINARY_DIR, CHROMEDRIVER_VERSION_URL, GECKODRIVER_LATEST_RELEASE_URL,
};
use crate::utils::file::{download_and_extract, download_file};

// バイナリディレクトリ取得
fn get_binary_path(binary: &str) -> Result<(PathBuf, PathBuf), String> {
    let binaries_dir = BINARY_DIR.clone();
    let binary_path = binaries_dir.join(binary);

    Ok((binaries_dir, binary_path))
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

/// ChromeDriverのURL取得
fn get_chromedriver_url() -> Result<String, String> {
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

/// GeckoDriver の URL を取得
fn get_geckodriver_url() -> Result<String, String> {
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

/// `~/.local/scshoki/bin/` の存在確認＆なければ作成
pub fn init_binaries() -> Result<(), String> {
    let binaries_dir = BINARY_DIR.clone();

    if !BINARY_DIR.exists() {
        fs::create_dir_all(&binaries_dir)
            .map_err(|e| format!("Failed to create binaries dir: {}", e))?;
        info!("Created binaries directory at {:?}", binaries_dir);
    }

    Ok(())
}

/// `~/.local/scshoki/bin/` に `Appium` があるか確認し、なければダウンロード
pub fn ensure_appium() -> Result<PathBuf, String> {
    let (_, appium_path) = get_binary_path("appium")?;

    if appium_path.exists() {
        info!("Appium is already installed: {:?}", appium_path);
        return Ok(appium_path);
    }

    info!("Downloading Appium...");
    download_file(APPIUM_URL, &appium_path)?;
    info!("Appium installed at {:?}", appium_path);

    // 実行権限を付与（UNIX系のみ）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&appium_path, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("Failed to set execute permissions for Appium: {}", e))?;
    }

    Ok(appium_path)
}

/// `chromedriver` があるか確認し、なければダウンロード
pub fn ensure_chromedriver() -> Result<PathBuf, String> {
    let (binaries_dir, chromedriver_path) = get_binary_path("chromedriver")?;

    if chromedriver_path.exists() {
        info!("ChromeDriver is already installed: {:?}", chromedriver_path);
        return Ok(chromedriver_path);
    }

    let url = get_chromedriver_url()?;
    info!("Downloading ChromeDriver from {:?}", url);

    if let Err(e) = download_and_extract(&url, &binaries_dir) {
        error!("{}", e);
        error!("Failed to install ChromeDriver");
    } else {
        info!("ChromeDriver installed at {:?}", chromedriver_path);
    }

    Ok(chromedriver_path)
}

/// `geckodriver` があるか確認し、なければダウンロード
pub fn ensure_geckodriver() -> Result<PathBuf, String> {
    let (binaries_dir, geckodriver_path) = get_binary_path("geckodriver")?;

    if geckodriver_path.exists() {
        info!("GeckoDriver is already installed: {:?}", geckodriver_path);
        return Ok(geckodriver_path);
    }

    let url = get_geckodriver_url()?;
    info!("Downloading GeckoDriver from {:?}", url);

    if let Err(e) = download_and_extract(&url, &binaries_dir) {
        error!("{}", e);
        error!("Failed to install GeckoDriver");
    } else {
        info!("GeckoDriver installed at {:?}", geckodriver_path);
    }

    Ok(geckodriver_path)
}
