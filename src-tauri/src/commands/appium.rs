use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

pub async fn start_appium(state: Arc<Mutex<Option<Child>>>) -> Result<(), String> {
    let process = Command::new("pnpm")
        .arg("exec")
        .arg("appium")
        .arg("--allow-insecure")
        .arg("chromedriver_autodownload")
        .arg("--session-override")
        .spawn()
        .map_err(|e| format!("Failed to start Appium: {}", e))?;

    println!("Appium server started.");

    let mut lock = state.lock().unwrap();
    *lock = Some(process);

    Ok(())
}

pub fn stop_appium(state: Arc<Mutex<Option<Child>>>) {
    let mut lock = state.lock().unwrap();
    if let Some(mut process) = lock.take() {
        if let Err(e) = process.kill() {
            eprintln!("Failed to stop Appium: {}", e);
        } else {
            println!("Appium server stopped.");
        }
    }
}
