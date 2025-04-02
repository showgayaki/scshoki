use std::process::Command;

const MDPI_BASE_DENSITY: f64 = 160.0; // Androidの基準密度（mdpi）

/// OSを指定してdensityを取得する関数
pub fn get_physical_density(os: &str) -> Result<f64, f64> {
    match os {
        "Android" => get_android_density(),
        "iOS" => get_ios_density(),
        _ => Err(1.0), // 不明なOSの場合はデフォルト値（mdpi相当）
    }
}

/// Androidのdensity取得
fn get_android_density() -> Result<f64, f64> {
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
                let density = density_value / MDPI_BASE_DENSITY;
                log::info!("Android Physical Density: {:.1}", density);
                return Ok(density);
            }
        }
    }

    Err(480.0 / MDPI_BASE_DENSITY) // エラー時のデフォルト値
}

/// iOSのdensity取得
fn get_ios_density() -> Result<f64, f64> {
    Ok(2.0)

    // Err(326.0 / MDPI_BASE_DENSITY) // エラー時のデフォルト値（Retinaディスプレイを想定）
}
