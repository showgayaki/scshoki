use std::collections::HashMap;
use tauri::command;

use super::infrastructure::appium;
use super::infrastructure::chromedriver;
use super::infrastructure::geckodriver;
use super::infrastructure::ios;
use super::infrastructure::node;
use super::services;

#[command]
pub fn check_installed_binaries() -> Vec<String> {
    return services::check_installed_binaries();
}
#[command]
pub async fn install_nodejs() -> Result<(), String> {
    return node::install().await;
}
#[command]
pub async fn install_appium() -> Result<(), String> {
    return appium::install().await;
}
#[command]
pub async fn install_chromedriver() -> Result<(), String> {
    return chromedriver::install().await;
}
#[command]
pub async fn install_geckodriver() -> Result<(), String> {
    return geckodriver::install().await;
}
#[command]
pub fn is_ios_dependencies_installed() -> Vec<HashMap<String, bool>> {
    ios::is_dependencies_installed()
}
