use log::{error, info};
use std::process::Command;
use tokio::task::spawn_blocking;

use crate::constants::NODE_DIR;

const APPIUM_VER: &str = "2.19.0";
const DRIVER_LIST: [&str; 3] = [
    "uiautomator2@4.2.3", // https://github.com/appium/appium-uiautomator2-driver/releases
    "gecko@1.4.3",        // https://github.com/appium/appium-geckodriver/releases
    "xcuitest@9.4.0",     // https://github.com/appium/appium-xcuitest-driver/releases
];

pub async fn install() -> Result<(), String> {
    return spawn_blocking(install_appium)
        .await
        .map_err(|e| format!("Task failed: {:?}", e))?;
}

fn install_appium() -> Result<(), String> {
    let npm_bin = NODE_DIR.join("bin/npm");
    info!("Installing Appium using {:?}", npm_bin);

    let mut child = Command::new(npm_bin)
        .current_dir(&*NODE_DIR)
        .arg("install")
        .arg(format!("appium@{}", APPIUM_VER))
        .spawn()
        .map_err(|e| format!("Failed to start npm: {}", e))?;

    info!("Waiting for Appium installation to complete...");

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for Appium installation: {}", e))?;

    if !status.success() {
        return Err("Failed to install Appium".to_string());
    }

    info!("Appium installed successfully.");

    install_appium_drivers()?;
    Ok(())
}

fn install_appium_drivers() -> Result<(), String> {
    let npm_bin = NODE_DIR.join("bin/npm");

    for driver in DRIVER_LIST.iter() {
        info!("Installing Appium driver: {}", driver);

        let mut child = Command::new(&npm_bin)
            .current_dir(&*NODE_DIR)
            .arg("exec")
            .arg("appium")
            .arg("driver")
            .arg("install")
            .arg(driver)
            .spawn()
            .map_err(|e| format!("Failed to start Appium for {}: {}", driver, e))?;

        let status = child.wait().map_err(|e| {
            format!(
                "Failed to wait for Appium driver installation ({}): {}",
                driver, e
            )
        })?;

        if !status.success() {
            error!("Failed to install Appium driver: {}", driver);
            return Err(format!("Failed to install Appium driver: {}", driver));
        }

        info!("Successfully installed Appium driver: {}", driver);
    }

    Ok(())
}
