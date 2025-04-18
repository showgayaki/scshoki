use log::debug;
use rusb::{Device, UsbContext};

use super::constants::{DEFAULT_DEVICE_VALUE, USB_CONTEXT};

pub fn detect_device_info<T: UsbContext>(
    device: &Device<T>,
) -> Result<(String, String, String), String> {
    let os: String;
    let mut product_name = DEFAULT_DEVICE_VALUE.to_string();
    let mut manufacturer = DEFAULT_DEVICE_VALUE.to_string();

    if let Ok(desc) = device.device_descriptor() {
        let vendor_id = desc.vendor_id();
        if let Some(context) = USB_CONTEXT.get() {
            // デバイスタイプの判別
            match vendor_id {
                0x18D1 => os = "Android".to_string(),
                0x05AC => os = "iOS".to_string(),
                _ => return Err("Unsupported device detected".to_string()),
            }
            if let Some(handle) =
                context.open_device_with_vid_pid(desc.vendor_id(), desc.product_id())
            {
                if let Some(product_index) = desc.product_string_index() {
                    if let Ok(product_string) = handle.read_string_descriptor_ascii(product_index) {
                        product_name = product_string;
                    }
                }
                if let Ok(manufacturer_string) = handle.read_manufacturer_string_ascii(&desc) {
                    manufacturer = manufacturer_string;
                }

                return Ok((os, product_name, manufacturer));
            } else {
                debug!("Could not open device with vid/pid using USB_CONTEXT");
            }
        }
    }

    Err("Failed to get device info".to_string())
}
