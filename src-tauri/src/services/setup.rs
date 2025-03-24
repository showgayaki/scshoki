use log::{error, info};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::config::constants::{APPIUM_VER, BINARY_DIR, DRIVER_LIST, NODE_DIR, NODE_VER};
use crate::services::binaries::{get_chromedriver_url, get_geckodriver_url};
use crate::services::network::download_file;
use crate::utils::archive::extract;

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

    let url = match (env::consts::OS, env::consts::ARCH) {
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

    info!("Download URL: {}", url);
    let dest_path = BINARY_DIR.join(url.split('/').last().unwrap());

    match download_file(&url, &dest_path) {
        Ok(archive_path) => {
            info!("Successfully downloaded Node.js to {:?}", archive_path);
            if let Err(e) = extract(&archive_path, &BINARY_DIR) {
                return Err(format!("Failed to extract Node.js: {}", e));
            } else {
                info!("Node.js installed at {:?}", archive_path);
                // アーカイブ削除
                fs::remove_file(&archive_path)
                    .map_err(|e| format!("Failed to delete archive: {}", e))?;
            }
        }
        Err(e) => return Err(format!("Failed to download Node.js: {}", e)),
    }

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

pub fn ensure_appium() -> Result<(), String> {
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

    install_appium_drivers()?;
    Ok(())
}

fn install_appium_drivers() -> Result<(), String> {
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
    let chromedriver_path = BINARY_DIR.join("chromedriver");

    if chromedriver_path.exists() {
        info!("ChromeDriver is already installed: {:?}", chromedriver_path);
        return Ok(chromedriver_path);
    }

    let url = get_chromedriver_url()?;
    let dest_path = BINARY_DIR.join(url.split('/').last().unwrap());
    info!("Downloading ChromeDriver from {:?}", url);

    match download_file(&url, &dest_path) {
        Ok(archive_path) => {
            info!("Successfully downloaded ChromeDriver to {:?}", archive_path);

            if let Err(e) = extract(&archive_path, &BINARY_DIR) {
                return Err(format!("Failed to extract ChromeDriver: {}", e));
            } else {
                // macOS の場合は chmod +x
                if env::consts::OS != "windows" {
                    let exec_path = BINARY_DIR.join("chromedriver");
                    Command::new("chmod")
                        .arg("+x")
                        .arg(&exec_path)
                        .output()
                        .map_err(|e| {
                            format!("Failed to set execute permission for Node.js: {}", e)
                        })?;
                }
                info!("ChromeDriver installed at {:?}", chromedriver_path);

                // アーカイブ削除
                fs::remove_file(&archive_path)
                    .map_err(|e| format!("Failed to delete archive: {}", e))?;
            }
        }
        Err(e) => return Err(format!("Failed to download ChromeDriver: {}", e)),
    }

    Ok(chromedriver_path)
}

/// `geckodriver` があるか確認し、なければダウンロード
pub fn ensure_geckodriver() -> Result<PathBuf, String> {
    let geckodriver_path = BINARY_DIR.join("geckodriver");

    if geckodriver_path.exists() {
        info!("GeckoDriver is already installed: {:?}", geckodriver_path);
        return Ok(geckodriver_path);
    }

    let url = get_geckodriver_url()?;
    let dest_path = BINARY_DIR.join(url.split('/').last().unwrap());
    info!("Downloading GeckoDriver from {:?}", url);

    match download_file(&url, &dest_path) {
        Ok(archive_path) => {
            info!("Successfully downloaded GeckoDriver to {:?}", archive_path);

            if let Err(e) = extract(&archive_path, &BINARY_DIR) {
                return Err(format!("Failed to extract GeckoDriver: {}", e));
            } else {
                // macOS の場合は chmod +x
                if env::consts::OS != "windows" {
                    let exec_path = BINARY_DIR.join("geckodriver");
                    Command::new("chmod")
                        .arg("+x")
                        .arg(&exec_path)
                        .output()
                        .map_err(|e| {
                            format!("Failed to set execute permission for Node.js: {}", e)
                        })?;
                }
                info!("GeckoDriver installed at {:?}", geckodriver_path);
                // アーカイブ削除
                fs::remove_file(&archive_path)
                    .map_err(|e| format!("Failed to delete archive: {}", e))?;
            }
        }
        Err(e) => return Err(format!("Failed to download GeckoDriver: {}", e)),
    }

    Ok(geckodriver_path)
}
