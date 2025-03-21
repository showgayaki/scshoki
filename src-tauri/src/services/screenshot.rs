use image::{DynamicImage, GenericImageView, ImageBuffer};
use log::{debug, info};
use std::fs;
use thirtyfour::prelude::*;

use crate::config::constants::SCREENSHOT_DIR;
use crate::services::dom::{
    get_page_metrics, get_scroll_position, hide_elements, scroll_by, show_elements,
};
use crate::utils::image::{cut_scroll_overlap, trim_extra_space};
use crate::utils::wait::{wait_for_elements_hidden, wait_for_scroll_complete};

pub async fn capture_full_page(
    driver: &WebDriver,
    hidden_elements: &str,
) -> Result<Vec<Vec<u8>>, String> {
    info!("Capturing full page screenshot...");

    // ページの各種メトリクスを取得
    let metrics = get_page_metrics(driver)
        .await
        .map_err(|e| format!("Failed to get page metrics: {}", e))?;

    let total_scroll_height = *metrics.get("totalScrollHeight").unwrap_or(&0.0);
    let inner_height = *metrics.get("innerHeight").unwrap_or(&0.0);
    let scroll_steps = *metrics.get("scrollSteps").unwrap_or(&0.0) as u32;

    if total_scroll_height <= 0.0 {
        return Err("Failed to retrieve page height.".to_string());
    }

    // 保存先ディレクトリを作成
    if !SCREENSHOT_DIR.exists() {
        info!("Creating screenshots directory...");
        fs::create_dir(&*SCREENSHOT_DIR)
            .map_err(|e| format!("Failed to create screenshots directory: {}", e))?;
    }

    // 最初のスクリーンショット（ヘッダーあり）を撮影
    info!("Taking first screenshot...");
    let mut screenshots = vec![];

    // スクロールしながらスクリーンショット
    for index in 1..=scroll_steps {
        debug!("Starting scroll and caputure.");

        // スクリーンショットを撮る
        let screenshot: Vec<u8> = driver
            .screenshot_as_png()
            .await
            .map_err(|e| format!("Failed to take screenshot: {}", e))?;

        // 最後のスクロール時は、被った部分をカット
        let cropped_screenshot = if index == scroll_steps {
            // 被った部分を計算
            let scroll_overlap_height = (inner_height * index as f64) - total_scroll_height;
            // 余白をカットしてから被った部分をカット
            let tmp = trim_extra_space(&screenshot, inner_height)?;
            cut_scroll_overlap(&tmp, scroll_overlap_height)?
        } else {
            trim_extra_space(&screenshot, inner_height)?
        };

        screenshots.push(cropped_screenshot.clone());

        let filename = format!("screenshot_{}.png", index);
        fs::write(SCREENSHOT_DIR.join(filename), &cropped_screenshot)
            .map_err(|e| format!("Failed to save screenshot_{}: {}", index, e))?;
        info!("Saved screenshot_{}.png", index);

        // スクロール実行
        scroll_by(driver, inner_height)
            .await
            .map_err(|e| format!("Failed to scroll: {}", e))?;
        wait_for_scroll_complete(driver).await?; // スクロール完了を待つ

        // 新しいスクロール位置を取得
        let y_offset = get_scroll_position(driver)
            .await
            .map_err(|e| format!("Failed to get scroll position: {}", e))?;
        info!("Scrolled to: {} px", y_offset);

        // 最初のスクロール直後に指定した要素を非表示にする
        if index == 1 {
            hide_elements(driver, hidden_elements)
                .await
                .map_err(|e| format!("Failed to hide elements: {}", e))?;
            wait_for_elements_hidden(driver, hidden_elements).await?; // 非表示完了を待つ
        }
    }

    // 非表示にした要素を元に戻す
    show_elements(driver, hidden_elements)
        .await
        .map_err(|e| format!("Failed to restore elements: {}", e))?;

    Ok(screenshots)
}

// スクリーンショットを結合する関数
pub fn combine_screenshots(screenshots: Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
    info!("Combining screenshots...");
    if screenshots.is_empty() {
        return Err("No screenshots to combine".to_string());
    }

    let first_image = image::load_from_memory(&screenshots[0]).map_err(|e| e.to_string())?;
    let (width, first_height) = first_image.dimensions();
    let mut total_height = first_height;

    // 各スクリーンショットの高さを取得し、合計高さを計算
    let mut image_heights = vec![first_height];
    for screenshot in screenshots.iter().skip(1) {
        let image = image::load_from_memory(screenshot).map_err(|e| e.to_string())?;
        let (_, height) = image.dimensions();
        total_height += height;
        image_heights.push(height);
    }

    // 最終画像の高さが合っているかログ出力
    info!(
        "Combining {} images, total height: {} px",
        screenshots.len(),
        total_height
    );

    let mut combined_image = ImageBuffer::new(width, total_height);
    let mut y_offset = 0;

    for screenshot in screenshots.iter() {
        let image = image::load_from_memory(screenshot).map_err(|e| e.to_string())?;
        let (_, height) = image.dimensions();

        for (x, y, pixel) in image.pixels() {
            combined_image.put_pixel(x, y + y_offset, pixel);
        }

        y_offset += height;
    }

    let mut output = std::io::Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(combined_image)
        .write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save combined image: {}", e))?;

    Ok(output.into_inner())
}
