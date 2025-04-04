use log::info;
use rusb::{Context, UsbContext};

pub fn detect_device_os() -> Result<String, String> {
    let context = Context::new().unwrap();
    let mut os = "Unknown".to_string();
    let mut product_name = "Unknown".to_string();
    let mut manufacturer = "Unknown".to_string();

    if let Some(device) = context.devices().unwrap().iter().next() {
        let device_desc = device.device_descriptor().unwrap();

        // デバイスタイプの判別
        match device_desc.vendor_id() {
            0x18D1 => {
                os = "Android".to_string();
            }
            0x05AC => {
                os = "iOS".to_string();
            }
            _ => {
                os = "Unknown".to_string();
            }
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
        return Ok(os);
    }

    Ok(os)
}

pub fn ios_version() -> Result<String, String> {
    let output = std::process::Command::new("ideviceinfo")
        .arg("-k")
        .arg("ProductVersion")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let version = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to convert output to string: {}", e))?
        .trim()
        .to_string();

    info!("iOS version: {}", version);
    Ok(version.trim().to_string())
}
