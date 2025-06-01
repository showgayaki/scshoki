use log::{error, info};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::constants::NODE_DIR;

use super::infrastructure::wait::wait_for_appium_ready;

pub struct AppiumState {
    pub(crate) process: Arc<Mutex<Option<Child>>>,
}

impl AppiumState {
    const APPIUM_TIMEOUT: Duration = Duration::from_secs(10);

    pub async fn start_appium(&self, app_handle: AppHandle) -> Result<(), String> {
        let mut lock = self.process.lock().unwrap();
        if lock.is_some() {
            error!("Appium is already running.");
            let _ = app_handle.emit("appium_ready", ());
            return Err("Appium is already running.".to_string());
        }

        let npm_bin = NODE_DIR.join("bin/npm");
        info!("Starting Appium with Node.js: {:?}", npm_bin);

        let process = Command::new(npm_bin)
            .current_dir(&*NODE_DIR)
            .arg("exec")
            .arg("appium")
            .arg("--allow-insecure")
            .arg("--session-override")
            .spawn()
            .map_err(|e| format!("Failed to start Appium: {}", e))?;

        *lock = Some(process);

        // wait for Appium and emit event
        tokio::spawn(wait_for_appium_ready(app_handle, Self::APPIUM_TIMEOUT));

        Ok(())
    }

    pub fn stop_appium(&self) -> Result<(), String> {
        let mut lock = self.process.lock().unwrap();
        if let Some(mut process) = lock.take() {
            if let Err(e) = process.kill() {
                error!("Failed to stop Appium: {}", e);
                return Err(format!("Failed to stop Appium: {}", e));
            } else {
                info!("Appium server stopped.");
            }
        }
        Ok(())
    }
}
