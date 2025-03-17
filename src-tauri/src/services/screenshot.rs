use std::fs;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use thirtyfour::prelude::*;
use log::info;

use crate::config::constants;
use crate::services::dom::{get_page_metrics, hide_elements, show_elements, get_scroll_position, scroll_by};
use crate::services::utils::{trim_extra_space, cut_scroll_overlap};


pub async fn capture_full_page(driver: &WebDriver, hidden_elements: &str) -> Result<Vec<Vec<u8>>, String> {
    info!("Capturing full page screenshot...");

    // ページの各種メトリクスを取得
    let metrics = get_page_metrics(driver).await
        .map_err(|e| format!("Failed to get page metrics: {}", e))?;

    let height = *metrics.get("height").unwrap_or(&0.0);
    let screen_height = *metrics.get("screenHeight").unwrap_or(&0.0);
    let visual_viewport_height = *metrics.get("visualViewportHeight").unwrap_or(&0.0);
    let avail_height = *metrics.get("availHeight").unwrap_or(&0.0);
    let client_height = *metrics.get("clientHeight").unwrap_or(&0.0);
    let viewport_height = *metrics.get("viewportHeight").unwrap_or(&0.0);
    let scroll_height = *metrics.get("scrollHeight").unwrap_or(&0.0);
    let navigation_bar_height = avail_height - viewport_height + 72.0; // とりあえず72pxは力技

    if height <= 0.0 {
        return Err("Failed to retrieve page height.".to_string());
    }

    info!("Debug info:");
    info!("screenHeight: {} px", screen_height);
    info!("visualViewportHeight: {} px", visual_viewport_height);
    info!("availHeight: {} px", avail_height);
    info!("clientHeight: {} px", client_height);
    info!("height: {} px", height);
    info!("viewport_height: {} px", viewport_height);
    info!("scroll_height: {} px", scroll_height);
    info!("navigation_bar_height: {} px", navigation_bar_height);

    // 保存先ディレクトリを作成
    if !constants::SCREENSHOT_DIR.exists() {
        info!("Creating screenshots directory...");
        fs::create_dir(&*constants::SCREENSHOT_DIR).map_err(|e| format!("Failed to create screenshots directory: {}", e))?;
    }

    // 1. 最初のスクリーンショット（ヘッダーあり）を撮影
    info!("Taking first screenshot...");
    let mut screenshots = vec![];
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await; // 読み込み待ち
    let first_screenshot = driver.screenshot_as_png().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;

    let cropped_first_screenshot = trim_extra_space(&first_screenshot, navigation_bar_height)?;
    screenshots.push(cropped_first_screenshot.clone());

    // 個別保存
    fs::write(constants::SCREENSHOT_DIR.join("screenshot_0.png"), &cropped_first_screenshot)
        .map_err(|e| format!("Failed to save screenshot_0.png: {}", e))?;
    info!("Saved screenshot_0.png");

    // 2. 指定した要素を非表示にする
    hide_elements(driver, hidden_elements).await
        .map_err(|e| format!("Failed to hide elements: {}", e))?;

    // 3. スクロールしながらスクリーンショット
    let mut y_offset = 0.0;
    let mut index = 1;

    while y_offset < height {
        // 残りのスクロール可能な高さを計算
        let remaining_scroll = height - y_offset;
        let scroll_amount = if remaining_scroll < viewport_height {
            remaining_scroll // 最後のスクロールは、余った高さだけスクロール
        } else {
            viewport_height
        };

        // スクロール実行
        scroll_by(driver, scroll_amount).await.map_err(|e| format!("Failed to scroll: {}", e))?;
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await; // スクロール完了を待つ

        // 新しいスクロール位置を取得
        let new_y_offset = get_scroll_position(driver).await
            .map_err(|e| format!("Failed to get scroll position: {}", e))?;

        info!("Scrolled to: {}", new_y_offset);

        // `y_offset` が変わらない（スクロールの余地なし）ならループを抜ける
        if new_y_offset == y_offset {
            info!("No further scrolling detected, stopping.");
            break;
        }

        y_offset = new_y_offset;

        // スクリーンショットを撮る
        let screenshot: Vec<u8> = driver.screenshot_as_png().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;
        // 最後のスクロール時は、被った部分をカット
        let cropped_screenshot = if y_offset + viewport_height > height {
             // 被った部分をカットしてから余白をカット
            let temp = cut_scroll_overlap(&screenshot, remaining_scroll)?;
            trim_extra_space(&temp, navigation_bar_height)?
        } else {
            trim_extra_space(&screenshot, navigation_bar_height)?
        };

        screenshots.push(cropped_screenshot.clone());

        let filename = format!("screenshot_{}.png", index);
        fs::write(constants::SCREENSHOT_DIR.join(filename), &cropped_screenshot)
            .map_err(|e| format!("Failed to save screenshot_{}: {}", index, e))?;

        index += 1;
    }

    // 4. 非表示にした要素を元に戻す
    show_elements(driver, hidden_elements).await
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
    info!("Combining {} images, total height: {} px", screenshots.len(), total_height);

    let mut combined_image = ImageBuffer::new(width, total_height);
    let mut y_offset = 0;

    for (_i, screenshot) in screenshots.iter().enumerate() {
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
