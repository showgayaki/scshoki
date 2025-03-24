use log::{error, info};
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::config::constants::{
    APPIUM_VER, BINARY_DIR, CHROMEDRIVER_VERSION_URL, DRIVER_LIST, GECKODRIVER_LATEST_RELEASE_URL,
    NODE_DIR, NODE_VER,
};
use crate::utils::file::{download_and_extract, download_file};

// バイナリディレクトリ取得
fn get_binary_path(binary: &str) -> Result<(PathBuf, PathBuf), String> {
    let binaries_dir = BINARY_DIR.clone();
    let binary_path = binaries_dir.join(binary);

    let old_path = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("{}:{}", binaries_dir.display(), old_path));

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

/// Node.js のバイナリをダウンロードして展開
pub fn ensure_node() -> Result<(), String> {
    let node_bin_path = NODE_DIR.join("bin/node");

    if node_bin_path.exists() {
        info!(
            "Node.js is already installed at: {}",
            node_bin_path.display()
        );
        return Ok(());
    }

    info!("Downloading and installing Node.js...");

    let node_url = match (env::consts::OS, env::consts::ARCH) {
        ("macos", "x86_64") => format!(
            "https://nodejs.org/dist/{}/node-{}-darwin-x64.tar.gz",
            NODE_VER, NODE_VER
        ),
        ("macos", "aarch64") => format!(
            "https://nodejs.org/dist/{}/node-{}-darwin-arm64.tar.gz",
            NODE_VER, NODE_VER
        ),
        ("windows", "x86_64") => format!(
            "https://nodejs.org/dist/{}/node-{}-win-x64.zip",
            NODE_VER, NODE_VER
        ),
        _ => return Err("Unsupported platform for Node.js".to_string()),
    };

    info!("Doanload URL: {}", node_url);

    download_and_extract(&node_url, &BINARY_DIR)?;

    // macOS の場合は `bin/node` を chmod +x
    if env::consts::OS != "windows" {
        let node_exec = NODE_DIR.join("bin/node");
        Command::new("chmod")
            .arg("+x")
            .arg(&node_exec)
            .output()
            .map_err(|e| format!("Failed to set execute permission for Node.js: {}", e))?;
    }

    Ok(())
}

pub fn install_appium() -> Result<(), String> {
    let npm_bin = NODE_DIR.join("bin/npm");

    if NODE_DIR.join("node_modules/appium").is_dir() {
        info!("Appium is already installed");
        return Ok(());
    }

    info!("Installing Appium using {:?}", npm_bin);

    let mut child = Command::new(npm_bin)
        .current_dir(&*NODE_DIR)
        .arg("install")
        .arg(format!("appium@{}", APPIUM_VER))
        .spawn()
        .map_err(|e| format!("Failed to start npm: {}", e))?;

    info!("Waiting for Appium installation to complete...");

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for Appium installation: {}", e))?;

    if !status.success() {
        return Err("Failed to install Appium".to_string());
    }

    info!("Appium installed successfully.");

    install_drivers()?;
    Ok(())
}

fn install_drivers() -> Result<(), String> {
    let npm_bin = NODE_DIR.join("bin/npm");

    for driver in DRIVER_LIST.iter() {
        info!("Installing Appium driver: {}", driver);

        let mut child = Command::new(&npm_bin)
            .current_dir(&*NODE_DIR)
            .arg("exec")
            .arg("appium")
            .arg("driver")
            .arg("install")
            .arg(driver)
            .spawn()
            .map_err(|e| format!("Failed to start Appium for {}: {}", driver, e))?;

        let status = child.wait().map_err(|e| {
            format!(
                "Failed to wait for Appium driver installation ({}): {}",
                driver, e
            )
        })?;

        if !status.success() {
            error!("Failed to install Appium driver: {}", driver);
            return Err(format!("Failed to install Appium driver: {}", driver));
        }

        info!("Successfully installed Appium driver: {}", driver);
    }

    Ok(())
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
