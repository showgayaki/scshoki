use log::{error, info};
use std::fs;

use crate::config::constants::paths::BINARY_DIR;
use crate::infrastructure::dependencies::binaries::appium;
use crate::infrastructure::dependencies::binaries::chromedriver;
use crate::infrastructure::dependencies::binaries::geckodriver;
use crate::infrastructure::dependencies::binaries::node;

pub fn check_or_install() {
    // バイナリ用ディレクトリのチェック
    if !BINARY_DIR.exists() {
        if let Err(e) = fs::create_dir_all(&*BINARY_DIR) {
            error!("Failed to create binaries dir: {}", e);
            std::process::exit(1);
        }
        info!("Created binaries directory at {:?}", BINARY_DIR);
    }

    // Node.jsのチェック＆インストール
    if let Err(e) = node::check_or_install() {
        error!("Failed to setup Node.js: {}", e);
        std::process::exit(1);
    }

    // Appiumのチェック＆インストール
    if let Err(e) = appium::check_or_install() {
        error!("Failed to setup Appium: {}", e);
    }

    // ChromeDriverのチェック＆インストール
    if let Err(e) = chromedriver::check_or_install() {
        error!("Failed to setup ChromeDriver: {}", e);
    }

    //  GeckoDriverのチェック＆インストール
    if let Err(e) = geckodriver::check_or_install() {
        error!("Failed to setup GeckoDriver: {}", e);
    }
}
