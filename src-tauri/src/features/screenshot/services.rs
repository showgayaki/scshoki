use image::{DynamicImage, GenericImageView, ImageBuffer};
use log::{debug, info};
use std::fs;
use thirtyfour::prelude::*;
use tokio_util::sync::CancellationToken;

use crate::constants::SCREENSHOT_DIR;
use crate::utils::cancel::check_cancellation;
use crate::utils::wait::wait_for_elements_hidden;

use super::infrastructure::dom::{
    get_page_metrics, get_scroll_position, hide_elements, scroll_by, show_elements,
};
use super::infrastructure::image::{cut_scroll_overlap, get_image_size, trim_extra_space};

pub async fn screenshot_full_page(
    driver: &WebDriver,
    hidden_elements: &[String],
    datetime_now: &str,
    device_os: &str,
    browser: &str,
    navigationbar_height: f64,
    token: &CancellationToken,
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

    // 既存タブを使うかもしれないので、0の位置に戻しておく
    let mut y_offset = get_scroll_position(driver)
        .await
        .map_err(|e| format!("Failed to get scroll position: {}", e))?;

    if y_offset != 0.0 {
        info!("Scrolling to top...");
        // スクロール位置を0に戻す
        scroll_by(driver, -y_offset)
            .await
            .map_err(|e| format!("Failed to scroll: {}", e))?;
    }

    // スクロールしながらスクリーンショット
    let mut screenshots = vec![];
    let mut y_before_last_scroll = 0.0;

    for index in 1..=scroll_steps {
        // キャンセルチェック
        check_cancellation(token, Some(driver)).await?;

        debug!("Screenshot count: {}", index);
        // スクリーンショットを撮る
        let screenshot: Vec<u8> = driver
            .screenshot_as_png()
            .await
            .map_err(|e| format!("Failed to take screenshot: {}", e))?;

        // 最後のスクロール時は、被った部分をカット
        let cropped_screenshot = if index == scroll_steps {
            // 残りの部分を計算
            // `前回のスクロール位置 - 今回のスクロール位置`が残りの部分の高さ
            let remaining_height = y_offset - y_before_last_scroll;
            debug!("Remaining height: {} px", remaining_height);
            // 余白をカットしてから被った部分をカット
            let tmp = trim_extra_space(&screenshot, browser, inner_height, navigationbar_height)?;
            cut_scroll_overlap(&tmp, remaining_height)?
        } else {
            trim_extra_space(&screenshot, browser, inner_height, navigationbar_height)?
        };

        screenshots.push(cropped_screenshot.clone());

        let filename = format!("{}_{}_{}_{}.png", datetime_now, device_os, browser, index);
        fs::write(SCREENSHOT_DIR.join(&filename), &cropped_screenshot)
            .map_err(|e| format!("Failed to save {}: {}", &filename, e))?;
        info!("Saved {}", &filename);

        // 最後のスクロールならあとの計算はもういらない
        if index == scroll_steps {
            break;
        }

        // スクショした画像の高さ分をスクロール
        let (_, croped_height) = get_image_size(&cropped_screenshot);
        // スクロール実行
        scroll_by(driver, croped_height)
            .await
            .map_err(|e| format!("Failed to scroll: {}", e))?;

        // 最後から2番目のyの位置を保存しておく（残りの高さ計算用）
        if index == scroll_steps - 1 {
            y_before_last_scroll = y_offset;
        }

        // 新しいスクロール位置を取得
        y_offset = get_scroll_position(driver)
            .await
            .map_err(|e| format!("Failed to get scroll position: {}", e))?;
        info!("Scrolled to: {} px", y_offset);

        // 最初のスクロール直後に指定した要素を非表示にする
        if index == 1 && !hidden_elements.is_empty() {
            info!("Hiding specified elements...");
            for element in hidden_elements {
                hide_elements(driver, element)
                    .await
                    .map_err(|e| format!("Failed to hide elements: {}", e))?;
                wait_for_elements_hidden(driver, element).await?; // 非表示完了を待つ
            }
        }
    }

    if !hidden_elements.is_empty() {
        info!("Showing hidden elements...");
        for element in hidden_elements {
            show_elements(driver, element)
                .await
                .map_err(|e| format!("Failed to restore elements: {}", e))?;
        }
    }

    Ok(screenshots)
}

// スクリーンショットを結合する関数
pub fn combine_screenshots(screenshots: &[Vec<u8>]) -> Result<Vec<u8>, String> {
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
        debug!("image height: {}", height);

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
