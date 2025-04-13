use rusb::Context;
use std::sync::Mutex;
use std::sync::OnceLock;

pub static DEVICE_OS: Mutex<Option<String>> = Mutex::new(None);
pub static DEVICE_PRODUCT_NAME: Mutex<Option<String>> = Mutex::new(None);
pub static DEVICE_MANUFACTURE: Mutex<Option<String>> = Mutex::new(None);
pub static DEVICE_DENSITY: Mutex<Option<f64>> = Mutex::new(None);
pub static DEVICE_UDID: Mutex<Option<String>> = Mutex::new(None);
pub static IOS_VERSION: Mutex<Option<String>> = Mutex::new(None);

pub static USB_CONTEXT: OnceLock<Context> = OnceLock::new();
