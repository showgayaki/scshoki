use log::{debug, error, info};
use std::process::Command;
use thirtyfour::prelude::*;

use super::constants::{DEVICE_DENSITY, IDEVICE_STATUSBAR_HEIGHT};

const MDPI_BASE_DENSITY: f64 = 160.0; // Androidの基準密度（mdpi）

/// OSを指定してdensityを取得する関数
pub async fn get_display_info(driver: &WebDriver, os: &str) {
    debug!("get_physical_density(OS: {}) called!!!", os);
    let density = match os {
        "Android" => get_android_density().map_err(|_| "Failed to get Android density"),
        "iOS" => get_ios_screen_info(driver)
            .await
            .map_err(|_| "Failed to get iOS density"),
        _ => {
            error!("Unsupported OS({}): Failed to get density", os);
            Err("Unsupported OS")
        }
    };

    let mut density_cache = DEVICE_DENSITY.lock().unwrap();
    if let Ok(val) = density {
        *density_cache = val;
        info!("{} density: {:.1}", os, val);
    } else {
        *density_cache = 2.0;
        error!("{} density: failed to retrieve", os);
    }
}

/// Androidのdensity取得
fn get_android_density() -> Result<f64, f64> {
    debug!("get_android_density called!!!");
    let output = Command::new("adb")
        .arg("shell")
        .arg("wm")
        .arg("density")
        .output();

    if let Ok(output) = output {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            if let Some(density_value) = output_str
                .lines()
                .find(|line| line.contains("Physical density:"))
                .and_then(|line| line.split_whitespace().last())
                .and_then(|num| num.parse::<f64>().ok())
            {
                let density = density_value / MDPI_BASE_DENSITY;
                return Ok(density);
            }
        }
    }

    Err(480.0 / MDPI_BASE_DENSITY) // エラー時のデフォルト値
}

/// iOSのdensity取得
async fn get_ios_screen_info(driver: &WebDriver) -> Result<f64, String> {
    debug!("get_ios_screen_info called!!!");

    let script = "mobile: deviceScreenInfo";
    let screen_info = driver
        .execute(script, vec![])
        .await
        .map_err(|e| format!("[{}] script error: {}", script, e))?;

    let json_value = screen_info.json();
    debug!("json_value: {:?}", json_value);

    // statusBarSize.height を取得
    let statusbar_height = json_value
        .get("statusBarSize")
        .and_then(|s| s.get("height"))
        .and_then(|h| h.as_f64())
        .ok_or_else(|| "Failed to extract 'statusBarSize.height'".to_string())?;
    let mut statusbar_height_lock = IDEVICE_STATUSBAR_HEIGHT.lock().unwrap();
    *statusbar_height_lock = statusbar_height;

    // scale を取得
    let scale = json_value
        .get("scale")
        .and_then(|s| s.as_f64())
        .ok_or_else(|| "Failed to extract 'scale'".to_string())?;
    let mut device_density_lock = DEVICE_DENSITY.lock().unwrap();
    *device_density_lock = scale;

    Ok(scale)
}
