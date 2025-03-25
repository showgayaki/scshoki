use std::env;
use std::path::Path;

use log::info;

/// 指定されたディレクトリを `PATH` に追加する
pub fn add_to_path(dir: &Path) {
    if let Some(dir_str) = dir.to_str() {
        let old_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", dir_str, old_path);

        env::set_var("PATH", new_path);
        info!("Add to PATH: {:?}", dir);
    }
}
