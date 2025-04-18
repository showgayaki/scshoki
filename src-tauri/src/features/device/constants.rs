use rusb::Context;
use std::sync::OnceLock;
use std::sync::{LazyLock, Mutex};

pub const DEFAULT_DEVICE_VALUE: &str = "NoDetected";
pub static DEVICE_OS: LazyLock<Mutex<Option<String>>> =
    LazyLock::new(|| Mutex::new(Some(DEFAULT_DEVICE_VALUE.to_string())));
pub static DEVICE_PRODUCT_NAME: LazyLock<Mutex<Option<String>>> =
    LazyLock::new(|| Mutex::new(Some(DEFAULT_DEVICE_VALUE.to_string())));
pub static DEVICE_MANUFACTURE: LazyLock<Mutex<Option<String>>> =
    LazyLock::new(|| Mutex::new(Some(DEFAULT_DEVICE_VALUE.to_string())));
pub static DEVICE_DENSITY: Mutex<Option<f64>> = Mutex::new(None);
pub static DEVICE_UDID: Mutex<Option<String>> = Mutex::new(None);
pub static IOS_VERSION: Mutex<Option<String>> = Mutex::new(None);

pub static USB_CONTEXT: OnceLock<Context> = OnceLock::new();
// pub const SMARTPHONE_VENDOR_IDS: [u16; 5] = [0x18D1, 0x05AC, 0x2717, 0x22D9, 0x04E8];
// pub const IGNORED_INTERFACE_CLASSES: [u8; 4] = [0x03, 0x08, 0x09, 0x11];
