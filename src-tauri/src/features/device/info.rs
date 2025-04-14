use log::info;
use rusb::UsbContext;

use super::constants::USB_CONTEXT;

pub fn detect_device_info() -> Result<(String, String, String), String> {
    let context = USB_CONTEXT.get().expect("USB context not initialized");
    let mut os = "Unknown".to_string();
    let mut product_name = "Unknown".to_string();
    let mut manufacturer = "Unknown".to_string();

    if let Some(device) = context.devices().unwrap().iter().next() {
        let device_desc = device.device_descriptor().unwrap();
        let vendor_id = device_desc.vendor_id();

        // デバイスタイプの判別
        match vendor_id {
            0x18D1 => os = "Android".to_string(),
            0x05AC => os = "iOS".to_string(),
            _ => return Err("Unsupported device detected".to_string()),
        }

        // 製品名を取得
        if let Ok(handle) = device.open() {
            if let Some(product_index) = device_desc.product_string_index() {
                if let Ok(product_string) = handle.read_string_descriptor_ascii(product_index) {
                    product_name = product_string;
                }
            }
            if let Ok(manufacturer_string) = handle.read_manufacturer_string_ascii(&device_desc) {
                manufacturer = manufacturer_string;
            }
        }

        info!("Device detected: {}({}) {}", product_name, os, manufacturer);
        return Ok((os, product_name, manufacturer));
    }

    Err("Failed to get device info".to_string())
}
