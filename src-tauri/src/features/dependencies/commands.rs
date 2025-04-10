use log::{debug, error, info};
use std::collections::HashMap;
use std::fs;
use tauri::command;
use tokio::task;

use super::infrastructure::appium;
use super::infrastructure::chromedriver;
use super::infrastructure::geckodriver;
use super::infrastructure::ios;
use super::infrastructure::node;
use crate::constants::{BINARY_DIR, NODE_DIR};

#[command]
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

#[command]
pub async fn setup_node() -> Result<(), String> {
    let result = task::spawn_blocking(node::install)
        .await
        .map_err(|e| format!("Task failed: {:?}", e))?;

    result.await
}
#[command]
pub async fn setup_appium() -> Result<(), String> {
    return task::spawn_blocking(appium::install)
        .await
        .map_err(|e| format!("Task failed: {:?}", e))?;
}
#[command]
pub async fn setup_chromedriver() -> Result<(), String> {
    let result = task::spawn_blocking(chromedriver::install)
        .await
        .map_err(|e| format!("Task failed: {:?}", e))?;

    result.await
}
#[command]
pub async fn setup_geckodriver() -> Result<(), String> {
    let result = task::spawn_blocking(geckodriver::install)
        .await
        .map_err(|e| format!("Task failed: {:?}", e))?;

    result.await
}
#[command]
pub fn is_ios_dependencies_installed() -> Vec<HashMap<String, bool>> {
    debug!("commands: is_ios_dependencies_installed");
    ios::is_dependencies_installed()
}
