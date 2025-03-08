use std::process::{Child, Command};
use std::sync::{Arc, Mutex};


// Appium のプロセスを管理するための型
pub struct AppiumState {
    pub(crate) process: Arc<Mutex<Option<Child>>>,
}

impl AppiumState {
    // Appium サーバーを起動
    pub async fn start_appium(&self) -> Result<(), String> {
        let mut lock = self.process.lock().unwrap();
        if lock.is_some() {
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
        println!("Appium server started.");
        Ok(())
    }

    // Appium サーバーを停止
    pub fn stop_appium(&self) -> Result<(), String> {
        let mut lock = self.process.lock().unwrap();
        if let Some(mut process) = lock.take() {
            if let Err(e) = process.kill() {
                return Err(format!("Failed to stop Appium: {}", e));
            } else {
                println!("Appium server stopped.");
            }
        }
        Ok(())
    }
}
