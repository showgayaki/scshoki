use std::sync::Mutex;

pub static DEVICE_OS: Mutex<Option<String>> = Mutex::new(None);
pub static DEVICE_DENSITY: Mutex<Option<f64>> = Mutex::new(None);
pub static DEVICE_UDID: Mutex<Option<String>> = Mutex::new(None);
pub static IOS_VERSION: Mutex<Option<String>> = Mutex::new(None);
