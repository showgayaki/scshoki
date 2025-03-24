use flate2::read::GzDecoder;
use log::info;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use tar::Archive;
use zip::read::ZipArchive;

/// ZIPファイルを解凍
fn unzip_file(zip_path: &Path, dest_dir: &Path) -> Result<(), String> {
    info!("Unzipping ZIP file: {:?}", zip_path);

    let zip_file = File::open(zip_path).map_err(|e| format!("Failed to open ZIP file: {}", e))?;
    let mut archive = ZipArchive::new(BufReader::new(zip_file))
        .map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to access file in ZIP: {}", e))?;

        let file_name = file.name().to_string();
        let file_path = Path::new(&file_name);

        let output_path = if let Some(file_name) = file_path.file_name() {
            dest_dir.join(file_name) // **サブディレクトリを無視**
        } else {
            dest_dir.join(file_path) // **そのまま使用**
        };

        if file_name.ends_with('/') {
            fs::create_dir_all(&output_path)
                .map_err(|e| format!("Failed to create directory {:?}: {}", output_path, e))?;
        } else {
            let mut out_file = File::create(&output_path)
                .map_err(|e| format!("Failed to create extracted file {:?}: {}", output_path, e))?;
            std::io::copy(&mut file, &mut out_file)
                .map_err(|e| format!("Failed to extract file {:?}: {}", output_path, e))?;

            // 実行権限を付与（UNIX系のみ）
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&output_path, fs::Permissions::from_mode(0o755))
                    .map_err(|e| format!("Failed to set execute permissions: {}", e))?;
            }
        }
    }

    info!("Unzip completed: {:?}", dest_dir);
    Ok(())
}

/// TAR.GZファイルを解凍
fn extract_tar_gz(tar_gz_path: &Path, dest_dir: &Path) -> Result<(), String> {
    let tar_gz =
        fs::File::open(tar_gz_path).map_err(|e| format!("Failed to open TAR.GZ file: {}", e))?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    // 解凍
    archive
        .unpack(dest_dir)
        .map_err(|e| format!("Failed to extract TAR.GZ: {}", e))?;

    // 解凍後のディレクトリが存在するか確認
    let extracted_dirs: Vec<_> = fs::read_dir(dest_dir)
        .map_err(|e| format!("Failed to read extracted directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .collect();

    if extracted_dirs.is_empty() {
        return Err(
            "No directories found after extraction. Extraction might have failed.".to_string(),
        );
    }

    // `node-*` を `node` にリネーム
    rename_extracted_node(dest_dir)?;

    Ok(())
}

fn rename_extracted_node(dest_dir: &Path) -> Result<(), String> {
    for entry in fs::read_dir(dest_dir).map_err(|e| format!("Failed to read dir: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to access entry: {}", e))?;
        let path = entry.path();
        if path.is_dir()
            && path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .starts_with("node-")
        {
            let new_path = dest_dir.join("node");
            fs::rename(&path, &new_path)
                .map_err(|e| format!("Failed to rename {:?} to {:?}: {}", path, new_path, e))?;
            break;
        }
    }
    Ok(())
}

/// 指定URLのファイルをダウンロード
pub fn download_file(url: &str, dest_path: &Path) -> Result<(), String> {
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
    Ok(())
}

/// ダウンロード後に解凍処理を追加
pub fn download_and_extract(url: &str, dest_dir: &Path) -> Result<(), String> {
    let file_name = url
        .split('/')
        .last()
        .ok_or("Failed to extract file name from URL")?;
    let archive_path = dest_dir.join(file_name);

    // ファイルをダウンロード
    download_file(url, &archive_path)?;

    // 拡張子をチェックして解凍方法を自動選択
    if url.ends_with(".zip") {
        unzip_file(&archive_path, dest_dir)?;
    } else if url.ends_with(".tar.gz") {
        extract_tar_gz(&archive_path, dest_dir)?;
    } else {
        return Err("Unsupported archive format".to_string());
    }

    // アーカイブ削除
    fs::remove_file(&archive_path).map_err(|e| format!("Failed to delete archive: {}", e))?;

    Ok(())
}
