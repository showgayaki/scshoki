use log::{error, info};

use crate::config::constants::{DEVICE_DENSITY, DEVICE_OS, DEVICE_UDID, IOS_VERSION};
use crate::services::device::density::get_physical_density;
use crate::services::device::os::{detect_device_os, ios_version};
use crate::services::device::udid::get_udid;

pub fn detect_device() {
    // USBで接続されたデバイスを取得
    match detect_device_os() {
        Ok(os) => {
            // デバイスOSを更新
            let mut device_os_lock = DEVICE_OS.lock().unwrap();
            *device_os_lock = Some(os.clone());

            // DPIスケールを取得
            let mut density_cache = DEVICE_DENSITY.lock().unwrap();
            match get_physical_density(&os) {
                Ok(density) => {
                    // デバイスの物理密度を取得
                    *density_cache = Some(density);
                }
                Err(ref e) => error!("Failed to get density: {}", e),
            }

            if os == "iOS" {
                // iOSのバージョンを取得
                match ios_version() {
                    Ok(version) => {
                        // iOSのバージョンを更新
                        let mut ios_version_lock = IOS_VERSION.lock().unwrap();
                        *ios_version_lock = Some(version.clone());
                    }
                    Err(ref e) => error!("Failed to get iOS version: {}", e),
                }
                // UDIDを取得
                match get_udid() {
                    Ok(udid) => {
                        let mut udid_cache = DEVICE_UDID.lock().unwrap();
                        *udid_cache = Some(udid.clone());
                    }
                    Err(ref e) => error!("Failed to get UDID: {}", e),
                }
            }
        }
        Err(_) => {
            info!("No new device detected.");
        }
    }
}
