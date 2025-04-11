use log::{error, info};
use std::fs;

use crate::constants::{BINARY_DIR, NODE_DIR};

pub fn check_installed_binaries() -> Vec<String> {
    // バイナリ用ディレクトリのチェック
    if !BINARY_DIR.exists() {
        if let Err(e) = fs::create_dir_all(&*BINARY_DIR) {
            error!("Failed to create binaries dir: {}", e);
            std::process::exit(1);
        }
        info!("Created binaries directory at {:?}", BINARY_DIR);
    }

    let mut installed = Vec::new();

    if NODE_DIR.join("bin/node").exists() {
        info!("Node.js is already installed");
        installed.push("Node.js".to_string());
    } else {
        info!("Node.js is NOT installed");
    }

    if NODE_DIR.join("node_modules/appium").is_dir() {
        info!("Appium is already installed");
        installed.push("Appium".to_string());
    } else {
        info!("Appium is NOT installed");
    }

    if BINARY_DIR.join("chromedriver").exists() {
        info!("ChromeDriver is already installed");
        installed.push("ChromeDriver".to_string());
    } else {
        info!("ChromeDriver is NOT installed");
    }

    if BINARY_DIR.join("geckodriver").exists() {
        info!("GeckoDriver is already installed");
        installed.push("GeckoDriver".to_string());
    } else {
        info!("GeckoDriver is NOT installed");
    }

    installed
}
