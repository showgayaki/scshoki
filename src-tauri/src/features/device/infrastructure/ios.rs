use log::info;
use std::process::Command;

pub fn product_type() -> Result<String, String> {
    let output = std::process::Command::new("ideviceinfo")
        .arg("-k")
        .arg("ProductType")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let version = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to convert output to string: {}", e))?
        .trim()
        .to_string();

    info!("ProductType: {}", version);
    Ok(version.trim().to_string())
}

pub fn ios_version() -> Result<String, String> {
    let output = std::process::Command::new("ideviceinfo")
        .arg("-k")
        .arg("ProductVersion")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let version = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to convert output to string: {}", e))?
        .trim()
        .to_string();

    info!("iOS version: {}", version);
    Ok(version.trim().to_string())
}

pub fn get_udid() -> Result<String, String> {
    // iOSデバイスのUDIDを取得するためのコマンド
    let output = Command::new("idevice_id")
        .arg("-l")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // コマンドの出力をUTF-8文字列に変換
    let output_str = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to convert output to string: {}", e))?;

    // 出力からUDIDを取得
    let udid = output_str.trim().to_string();

    if udid.is_empty() {
        Err("No UDID found".to_string())
    } else {
        info!("Device UDID: {}", udid);
        Ok(udid)
    }
}
