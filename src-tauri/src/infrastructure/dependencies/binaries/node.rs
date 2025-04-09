use log::{error, info};

use crate::config::constants::paths::{BINARY_DIR, NODE_DIR};
use crate::config::env::{HOST_ARCH, HOST_OS};
use crate::infrastructure::archive::extract;
use crate::infrastructure::fs::{remove_file, set_executable};
use crate::infrastructure::network::download_file;

const NODE_VER: &str = "v22.14.0";

/// Node.js のバイナリをダウンロードして展開
pub async fn install() -> Result<(), String> {
    let url = download_url()?;
    info!("Downloading and installing Node.js from {}", url);
    let dest_path = BINARY_DIR.join(url.split('/').last().unwrap());

    match download_file(&url, &dest_path).await {
        Ok(archive_path) => {
            info!("Successfully downloaded Node.js to {:?}", archive_path);
            if let Err(e) = extract(&archive_path, &BINARY_DIR) {
                return Err(format!("Failed to extract Node.js: {}", e));
            } else {
                info!("Node.js installed at {:?}", archive_path);
                // アーカイブ削除
                match remove_file(&archive_path) {
                    Ok(()) => info!("Removed: {:?}", archive_path),
                    Err(ref e) => error!("Failed to remove {:?}: {}", archive_path, e),
                }
            }
        }
        Err(e) => return Err(format!("Failed to download Node.js: {}", e)),
    }

    // macOS の場合は `bin/node` を chmod +x
    if HOST_OS != "windows" {
        let node_exec = NODE_DIR.join("bin/node");
        set_executable(&node_exec).expect("Failed to set executable permissions");
    }

    Ok(())
}

fn download_url() -> Result<String, String> {
    let (os, arch, ext) = match (HOST_OS, HOST_ARCH) {
        ("windows", "x86_64") => ("win", "x64", "zip"),
        ("macos", "x86_64") => ("darwin", "x64", "tar.gz"),
        ("macos", "aarch64") => ("darwin", "arm64", "tar.gz"),
        _ => return Err("Unsupported platform".to_string()),
    };

    Ok(format!(
        "https://nodejs.org/dist/{}/node-{}-{}-{}.{}",
        NODE_VER, NODE_VER, os, arch, ext
    ))
}
