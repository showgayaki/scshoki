use log::info;
use std::process::Command;

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
