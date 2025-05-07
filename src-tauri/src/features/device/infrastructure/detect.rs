use log::{debug, info};
use rusb::{Device, UsbContext};
use tauri::{AppHandle, Emitter};

use crate::constants::{DEFAULT_DEVICE_VALUE, DEVICE_MANUFACTURE, DEVICE_OS, DEVICE_PRODUCT_NAME};

use super::info::{detect_device_info, get_idevice_info};

pub fn emit_device_event(app_handle: &AppHandle, event_type: &str) {
    debug!("emit_device_event called!!!");
    let os = DEVICE_OS.lock().unwrap();
    let product_name = DEVICE_PRODUCT_NAME.lock().unwrap();
    let manufacturer = DEVICE_MANUFACTURE.lock().unwrap();

    if *os == "iOS"
        && (*product_name == DEFAULT_DEVICE_VALUE || *manufacturer == DEFAULT_DEVICE_VALUE)
    {
        return;
    }

    let message = format!("{}({}: {}) {}", os, manufacturer, product_name, event_type);
    info!("{}", message);
    let _ = app_handle.emit(&format!("device_{}", event_type), message);
}

pub fn detect_device<T: UsbContext>(device: &Device<T>) -> Result<(), String> {
    match detect_device_info(device) {
        Ok((os, product_name, manufacturer)) => {
            let mut device_os_lock = DEVICE_OS.lock().unwrap();
            let mut device_product_name_lock = DEVICE_PRODUCT_NAME.lock().unwrap();
            let mut device_manufacturer_lock = DEVICE_MANUFACTURE.lock().unwrap();

            *device_os_lock = os.clone();
            *device_product_name_lock = product_name.clone();
            *device_manufacturer_lock = manufacturer.clone();

            if os == "iOS" {
                get_idevice_info();
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}
