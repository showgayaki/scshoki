use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// 指定URLのファイルをダウンロード
pub fn download_file(url: &str, dest_path: &Path) -> Result<PathBuf, String> {
    let response =
        reqwest::blocking::get(url).map_err(|e| format!("Failed to download {}: {}", url, e))?;
    let mut file = File::create(dest_path)
        .map_err(|e| format!("Failed to create file {}: {}", dest_path.display(), e))?;
    file.write_all(
        &response
            .bytes()
            .map_err(|e| format!("Failed to read response: {}", e))?,
    )
    .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(dest_path.to_path_buf()) // ダウンロードしたファイルのパスを返す
}
