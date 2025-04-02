use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Duration;

pub const HOST_OS: &str = env::consts::OS;
pub const HOST_ARCH: &str = env::consts::ARCH;

pub const IDENTIFIER: &str = "world.kanke.scshoki";
pub static DEVELOPMENT_TEAM: LazyLock<String> =
    LazyLock::new(|| env::var("DEVELOPMENT_TEAM").unwrap_or_else(|_| "Unknown".to_string()));

// HOMEディレクトリのパスをキャッシュ
pub static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| panic!("Failed to get HOME directory"))
});

pub const BASE_DIR: &str = ".scshoki";

// `canonicalize()` を使用して相対パスを絶対パスに変換
pub static SCREENSHOT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::fs::canonicalize(HOME_DIR.join(BASE_DIR).join("screenshots"))
        .unwrap_or_else(|_| HOME_DIR.join(BASE_DIR).join("screenshots"))
});

pub static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::fs::canonicalize(HOME_DIR.join(BASE_DIR).join("log"))
        .unwrap_or_else(|_| HOME_DIR.join(BASE_DIR).join("log"))
});

pub const LOG_FILE_NAME: &str = "scshoki.log";
pub const LOG_ROTATE_BASE: u32 = 1;
pub const LOG_ROTATE_COUNT: u32 = 3;
pub const MB: u64 = 1024 * 1024;
pub const LOG_ROTATE_SIZE_MB: u64 = 3;
pub const LOG_ROTATE_SIZE: u64 = LOG_ROTATE_SIZE_MB * MB;

pub const APPIUM_PORT: &str = "4723";
pub static APPIUM_SERVER_URL: LazyLock<String> =
    LazyLock::new(|| format!("http://127.0.0.1:{APPIUM_PORT}"));
pub const APPIUM_TIMEOUT: Duration = Duration::from_secs(10);

pub static BINARY_DIR: LazyLock<PathBuf> = LazyLock::new(|| HOME_DIR.join(BASE_DIR).join("bin"));
pub static NODE_DIR: LazyLock<PathBuf> = LazyLock::new(|| BINARY_DIR.join("node"));
pub const NODE_VER: &str = "v22.14.0";
pub const APPIUM_VER: &str = "2.17.1";
pub const DRIVER_LIST: [&str; 4] = [
    "uiautomator2@4.1.5",
    "gecko@1.4.3",
    "xcuitest@9.1.2",
    "safari@3.5.23",
];

pub const CHROMEDRIVER_VERSION_URL: &str = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";
pub const GECKODRIVER_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/mozilla/geckodriver/releases/latest";

pub static DEVICE_OS: Mutex<Option<String>> = Mutex::new(None);
pub static DEVICE_DENSITY: Mutex<Option<f64>> = Mutex::new(None);
pub static DEVICE_UDID: Mutex<Option<String>> = Mutex::new(None);
pub static IOS_VERSION: Mutex<Option<String>> = Mutex::new(None);
