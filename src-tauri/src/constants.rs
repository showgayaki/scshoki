use std::env;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

pub const HOST_OS: &str = env::consts::OS;
pub const HOST_ARCH: &str = env::consts::ARCH;

pub static DEVELOPMENT_TEAM: LazyLock<String> =
    LazyLock::new(|| env::var("DEVELOPMENT_TEAM").unwrap_or_else(|_| "Unknown".to_string()));

pub const BASE_DIR: &str = ".scshoki";
// HOMEディレクトリのパスをキャッシュ
pub static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| panic!("Failed to get HOME directory"))
});
pub static BINARY_DIR: LazyLock<PathBuf> = LazyLock::new(|| HOME_DIR.join(BASE_DIR).join("bin"));
pub static NODE_DIR: LazyLock<PathBuf> = LazyLock::new(|| BINARY_DIR.join("node"));

pub const APPIUM_PORT: &str = "4723";
pub static APPIUM_SERVER_URL: LazyLock<String> =
    LazyLock::new(|| format!("http://127.0.0.1:{APPIUM_PORT}"));
