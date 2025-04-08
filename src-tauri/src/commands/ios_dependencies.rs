use log::debug;
use std::collections::HashMap;
use tauri::command;

use crate::infrastructure::dependencies::binaries::ios;

#[command]
pub fn is_ios_dependencies_installed() -> Vec<HashMap<String, bool>> {
    debug!("commands: is_ios_dependencies_installed");
    ios::is_dependencies_installed()
}
