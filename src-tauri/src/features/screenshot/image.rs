use image::GenericImageView;
use log::debug;

use crate::features::device::constants::{DEVICE_DENSITY, DEVICE_OS, IDEVICE_STATUSBAR_HEIGHT};

/// 画像のいらない部分をカットする関数
/// - iOS
///   - Safari: ステータスバー(画面上部の時計とかWi-Fiとかのバー)からinner_height分を切り取る
///   - Safari以外： ステータスバーの下にあるURLバーの下からinner_height分を切り取る
///                 URLバーの高さは取得できない(Identifierの設定がない)が、ブラウザ下部のボタンの高さは取得できるので、
///                 (画像のheight - ステータスバーのheight - ボタン(ナビゲーションバー)のheight)
///                 で切り取る
///                 あと、ナビゲーションバーの境界の線が入っちゃうので少し調整する
pub fn trim_extra_space(
    image_data: &[u8],
    browser: &str,
    inner_height: f64,
    navigationbar_height: f64,
) -> Result<Vec<u8>, String> {
    debug!("trim_extra_space");

    let image =
        image::load_from_memory(image_data).map_err(|e| format!("Failed to load image: {}", e))?;
    let (width, height) = image.dimensions();

    if height <= inner_height as u32 {
        return Err("Image height is smaller than navigation bar height, cannot crop.".to_string());
    }

    // 必要な値をアレソレ
    let device_os = DEVICE_OS.lock().unwrap().clone();
    let physical_density = *DEVICE_DENSITY.lock().unwrap();
    let idevice_statusbar_height =
        (*IDEVICE_STATUSBAR_HEIGHT.lock().unwrap() * physical_density).round() as u32;
    let crop_height = (inner_height * physical_density).round() as u32;
    let navigationbar_height = (navigationbar_height * physical_density).round() as u32;

    let (start_y, crop_height) = match device_os.as_str() {
        "iOS" => match browser {
            "Chrome" => (
                height - (crop_height + navigationbar_height) + 1,
                crop_height - 2,
            ),
            "Firefox" => (
                height - (crop_height + navigationbar_height),
                crop_height - 1,
            ),
            _ => (idevice_statusbar_height, crop_height), // Safari
        },
        _ => (0, crop_height),
    };

    debug!(
        "[{}] start_y: {}, navigationbar_height: {}, crop_height: {}",
        browser, start_y, navigationbar_height, crop_height
    );

    let cropped_image = image.view(0, start_y, width, crop_height).to_image();

    let mut output = std::io::Cursor::new(Vec::new());
    cropped_image
        .write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(output.into_inner())
}

// 最後のスクロールで、被った部分をカットする関数
pub fn cut_scroll_overlap(
    image_data: &[u8],
    scroll_overlap_height: f64,
) -> Result<Vec<u8>, String> {
    debug!("cut_scroll_overlap");
    debug!("scroll_overlap_height: {} px", scroll_overlap_height);

    let image =
        image::load_from_memory(image_data).map_err(|e| format!("Failed to load image: {}", e))?;
    let (width, height) = image.dimensions();

    let physical_density = *DEVICE_DENSITY.lock().unwrap();

    let scroll_overlap_height = (scroll_overlap_height * physical_density) as u32;
    if height <= scroll_overlap_height {
        return Err("Image height is smaller than crop height, cannot crop.".to_string());
    }

    let new_height = height - scroll_overlap_height;
    let cropped_image = image
        .view(0, scroll_overlap_height, width, new_height)
        .to_image();

    let mut output = std::io::Cursor::new(Vec::new());
    cropped_image
        .write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(output.into_inner())
}
