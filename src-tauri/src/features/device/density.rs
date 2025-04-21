use log::{debug, error, info};
use std::process::Command;

use super::constants::{DEVICE_DENSITY, IDEVICE_PRODUCT_TYPE};
use crate::features::api::device_specs::fetch_device_specs;
use crate::utils::retry::retry_async;

const MDPI_BASE_DENSITY: f64 = 160.0; // Androidの基準密度（mdpi）

/// OSを指定してdensityを取得する関数
pub async fn get_physical_density(os: &str) {
    debug!("get_physical_density(OS: {}) called!!!", os);
    let density = match os {
        "Android" => get_android_density().map_err(|_| "Failed to get Android density"),
        "iOS" => get_ios_density()
            .await
            .map_err(|_| "Failed to get iOS density"),
        _ => {
            error!("Unsupported OS({}): Failed to get density", os);
            Err("Unsupported OS")
        }
    };

    let mut density_cache = DEVICE_DENSITY.lock().unwrap();
    if let Ok(val) = density {
        *density_cache = density.ok();
        info!("{} density: {:.1}", os, val);
    } else {
        *density_cache = Some(2.0);
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
async fn get_ios_density() -> Result<f64, f64> {
    debug!("get_ios_density called!!!");
    const RETRY: u8 = 3;
    const DELAY_MS: u64 = 1000;

    let product_type = {
        let lock = IDEVICE_PRODUCT_TYPE.lock().unwrap();
        match lock.as_deref() {
            Some(pt) => pt.to_string(),
            None => return Err(2.0),
        }
    };

    let result = retry_async(|| fetch_device_specs(Some(&product_type)), RETRY, DELAY_MS).await;

    match result {
        Ok(specs) => specs
            .get(&product_type)
            .map(|spec| spec.display.scale_factor)
            .ok_or(2.0),
        Err(_) => Err(2.0),
    }
}
