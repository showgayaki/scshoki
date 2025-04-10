use std::path::PathBuf;
use std::sync::LazyLock;

use crate::constants::{BASE_DIR, HOME_DIR};

// `canonicalize()` を使用して相対パスを絶対パスに変換
pub static SCREENSHOT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::fs::canonicalize(HOME_DIR.join(BASE_DIR).join("screenshots"))
        .unwrap_or_else(|_| HOME_DIR.join(BASE_DIR).join("screenshots"))
});
