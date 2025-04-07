use std::env;
use std::path::Path;
use std::sync::LazyLock;

use log::info;

pub const HOST_OS: &str = env::consts::OS;
pub const HOST_ARCH: &str = env::consts::ARCH;

pub static DEVELOPMENT_TEAM: LazyLock<String> =
    LazyLock::new(|| env::var("DEVELOPMENT_TEAM").unwrap_or_else(|_| "Unknown".to_string()));

/// 指定されたディレクトリを `PATH` に追加する
pub fn add_to_path(dir: &Path) {
    if let Some(dir_str) = dir.to_str() {
        let old_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", dir_str, old_path);

        env::set_var("PATH", new_path);
        info!("Add to PATH: {:?}", dir);
    }
}
