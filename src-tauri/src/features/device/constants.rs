use rusb::Context;
use std::sync::OnceLock;
use std::sync::{LazyLock, Mutex};

pub const DEFAULT_DEVICE_VALUE: &str = "NoDetected";
pub static DEVICE_OS: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub static DEVICE_PRODUCT_NAME: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub static DEVICE_MANUFACTURE: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub static DEVICE_DENSITY: Mutex<f64> = Mutex::new(1.0);
pub static IDEVICE_UDID: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub static IDEVICE_OS_VERSION: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub static IDEVICE_PRODUCT_TYPE: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub static IDEVICE_STATUSBAR_HEIGHT: LazyLock<Mutex<f64>> = LazyLock::new(|| Mutex::new(0.0));

pub static USB_CONTEXT: OnceLock<Context> = OnceLock::new();
// pub const SMARTPHONE_VENDOR_IDS: [u16; 5] = [0x18D1, 0x05AC, 0x2717, 0x22D9, 0x04E8];
// pub const IGNORED_INTERFACE_CLASSES: [u8; 4] = [0x03, 0x08, 0x09, 0x11];
