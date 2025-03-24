use log::{error, info};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

use crate::config::constants::NODE_DIR;

pub struct AppiumState {
    pub(crate) process: Arc<Mutex<Option<Child>>>,
}

impl AppiumState {
    pub async fn start_appium(&self) -> Result<(), String> {
        let mut lock = self.process.lock().unwrap();
        if lock.is_some() {
            error!("Appium is already running.");
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
        info!("Appium server started.");
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
