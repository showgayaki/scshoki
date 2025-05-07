use rusb::Context;
use std::env;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex, OnceLock};

pub(crate) const HOST_OS: &str = env::consts::OS;
pub(crate) const HOST_ARCH: &str = env::consts::ARCH;

pub(crate) const BASE_DIR: &str = ".scshoki";
// HOMEディレクトリのパスをキャッシュ
pub(crate) static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| panic!("Failed to get HOME directory"))
});
pub(crate) static BINARY_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| HOME_DIR.join(BASE_DIR).join("bin"));
pub(crate) static NODE_DIR: LazyLock<PathBuf> = LazyLock::new(|| BINARY_DIR.join("node"));

// `canonicalize()` を使用して相対パスを絶対パスに変換
pub(crate) static SCREENSHOT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::fs::canonicalize(HOME_DIR.join(BASE_DIR).join("screenshots"))
        .unwrap_or_else(|_| HOME_DIR.join(BASE_DIR).join("screenshots"))
});

pub(crate) const APPIUM_PORT: &str = "4723";
pub(crate) static APPIUM_SERVER_URL: LazyLock<String> =
    LazyLock::new(|| format!("http://127.0.0.1:{APPIUM_PORT}"));

pub(crate) static USB_CONTEXT: OnceLock<Context> = OnceLock::new();

pub(crate) const DEFAULT_DEVICE_VALUE: &str = "NoDetected";
pub(crate) static DEVICE_OS: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub(crate) static DEVICE_PRODUCT_NAME: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub(crate) static DEVICE_MANUFACTURE: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub(crate) static DEVICE_DENSITY: Mutex<f64> = Mutex::new(1.0);
pub(crate) static IDEVICE_UDID: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub(crate) static IDEVICE_OS_VERSION: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub(crate) static IDEVICE_PRODUCT_TYPE: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(DEFAULT_DEVICE_VALUE.to_string()));
pub(crate) static IDEVICE_STATUSBAR_HEIGHT: LazyLock<Mutex<f64>> =
    LazyLock::new(|| Mutex::new(0.0));

// pub(crate) const SMARTPHONE_VENDOR_IDS: [u16; 5] = [0x18D1, 0x05AC, 0x2717, 0x22D9, 0x04E8];
// pub(crate) const IGNORED_INTERFACE_CLASSES: [u8; 4] = [0x03, 0x08, 0x09, 0x11];
