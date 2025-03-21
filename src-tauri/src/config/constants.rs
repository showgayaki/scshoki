use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::Duration;

// `canonicalize()` を使用して相対パスを絶対パスに変換
pub static SCREENSHOT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::fs::canonicalize(Path::new("../screenshots"))
        .unwrap_or_else(|_| Path::new("../screenshots").to_path_buf())
});

pub static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::fs::canonicalize(Path::new("../log")).unwrap_or_else(|_| Path::new("../log").to_path_buf())
});
pub const LOG_FILE_NAME: &str = "scshoki.log";
pub const LOG_ROTATE_BASE: u32 = 1; // インデックスを 1 から始める
pub const LOG_ROTATE_COUNT: u32 = 3; // ローテートするファイル数
pub const MB: u64 = 1024 * 1024; // 1MB のバイト数
pub const LOG_ROTATE_SIZE_MB: u64 = 3; // ローテートサイズ（MB）
pub const LOG_ROTATE_SIZE: u64 = LOG_ROTATE_SIZE_MB * MB; // 実際のバイト数

pub const APPIUM_SERVER_URL: &str = "http://127.0.0.1:4723";
pub const APPIUM_TIMEOUT: Duration = Duration::from_secs(10);

pub const BINARY_DIR: &str = ".local/scshoki/bin";
pub const APPIUM_URL: &str = "https://github.com/appium/appium/releases/latest/download/appium";
pub const CHROMEDRIVER_VERSION_URL: &str = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";
pub const GECKODRIVER_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/mozilla/geckodriver/releases/latest";
