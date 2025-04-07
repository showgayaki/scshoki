use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

pub const BASE_DIR: &str = ".scshoki";
// HOMEディレクトリのパスをキャッシュ
pub static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| panic!("Failed to get HOME directory"))
});
pub static BINARY_DIR: LazyLock<PathBuf> = LazyLock::new(|| HOME_DIR.join(BASE_DIR).join("bin"));

// `canonicalize()` を使用して相対パスを絶対パスに変換
pub static SCREENSHOT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::fs::canonicalize(HOME_DIR.join(BASE_DIR).join("screenshots"))
        .unwrap_or_else(|_| HOME_DIR.join(BASE_DIR).join("screenshots"))
});

pub static NODE_DIR: LazyLock<PathBuf> = LazyLock::new(|| BINARY_DIR.join("node"));
