use std::process::Command;

pub fn detect_device() -> Result<String, String> {
    // Android デバイスの判定
    let adb_output = Command::new("adb")
        .arg("devices")
        .output()
        .map_err(|_| "Failed to execute adb command")?;

    let adb_output_str = String::from_utf8_lossy(&adb_output.stdout);
    if adb_output_str.lines().any(|line| line.ends_with("device")) {
        return Ok("Android".to_string());
    }

    // iOS デバイスの判定
    let idevice_output = Command::new("idevice_id")
        .arg("-l")
        .output()
        .map_err(|_| "Failed to execute idevice_id command")?;

    let idevice_output_str = String::from_utf8_lossy(&idevice_output.stdout);
    if !idevice_output_str.trim().is_empty() {
        return Ok("iOS".to_string());
    }

    Err("No device detected".to_string())
}
