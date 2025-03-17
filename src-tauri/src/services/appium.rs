use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use log::{info, error};


// Appium のプロセスを管理するための型
pub struct AppiumState {
    pub(crate) process: Arc<Mutex<Option<Child>>>,
}

impl AppiumState {
    // Appium サーバーを起動
    pub async fn start_appium(&self) -> Result<(), String> {
        let mut lock = self.process.lock().unwrap();
        if lock.is_some() {
            error!("Appium is already running.");
            return Err("Appium is already running.".to_string());
        }

        let process = Command::new("pnpm")
            .arg("exec")
            .arg("appium")
            .arg("--allow-insecure")
            .arg("chromedriver_autodownload")
            .arg("--session-override")
            .spawn()
            .map_err(|e| format!("Failed to start Appium: {}", e))?;

        *lock = Some(process);
        info!("Appium server started.");
        Ok(())
    }

    // Appium サーバーを停止
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
