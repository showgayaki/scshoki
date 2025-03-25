use std::process::Command;
use std::sync::LazyLock;

const MDPI_BASE_DENSITY: f64 = 160.0; // Androidの基準密度（mdpi）

/// `adb shell wm density` の結果から物理的なDPI値を取得し、mdpi(160dpi)基準のスケール値を返す。
/// 例: `Physical density: 480` → `3.0`
pub static PHYSICAL_DENSITY: LazyLock<Result<f64, f64>> = LazyLock::new(|| {
    let output = Command::new("adb")
        .arg("shell")
        .arg("wm")
        .arg("density")
        .output();

    if let Ok(output) = output {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            if let Some(density_value) = output_str
                .lines()
                .find(|line| line.contains("Physical density:"))
                .and_then(|line| line.split_whitespace().last())
                .and_then(|num| num.parse::<f64>().ok())
            {
                return Ok(density_value / MDPI_BASE_DENSITY);
            }
        }
    }

    // エラー時のデフォルト値（xxhdpi 480dpi）
    Err(480.0 / MDPI_BASE_DENSITY)
});
