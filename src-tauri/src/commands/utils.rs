use appium_client::Client;
use appium_client::capabilities::android::AndroidCapabilities;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::fs;

use crate::config;

pub async fn capture_full_page(client: &Client<AndroidCapabilities>, hidden_elements: &str) -> Result<Vec<Vec<u8>>, String> {
    let script = r#"
        return {
            height: Math.max(
                document.body.scrollHeight, document.body.offsetHeight,
                document.documentElement.clientHeight, document.documentElement.scrollHeight, document.documentElement.offsetHeight
            ),
            screenHeight: screen.height,
            visualViewportHeight: window.visualViewport.height,
            availHeight: screen.availHeight,
            clientHeight: document.documentElement.clientHeight,
            viewportHeight: window.innerHeight,
            scrollHeight: document.documentElement.scrollHeight,
        };
    "#;

    let result = client.execute(script, vec![]).await.map_err(|e| format!("Failed to execute script: {}", e))?;
    let height = result.get("height").and_then(|h| h.as_f64()).unwrap_or(0.0);

    let screen_height = result.get("screenHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let viual_viewport_height = result.get("visualViewportHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let avail_height = result.get("availHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let client_height = result.get("clientHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let viewport_height = result.get("viewportHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let scroll_height = result.get("scrollHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let navigation_bar_height = avail_height - viewport_height + 72.0; // とりあえず72pxは力技

    if height <= 0.0 {
        return Err("Failed to retrieve page height.".to_string());
    }

    println!("Debug info:");
    println!("  screenHeight: {} px", screen_height);
    println!("  visualViewportHeight: {} px", viual_viewport_height);
    println!("  availHeight: {} px", avail_height);
    println!("  clientHeight: {} px", client_height);
    println!("  height: {} px", height);
    println!("  viewport_height: {} px", viewport_height);
    println!("  scroll_height: {} px", scroll_height);
    println!("  navigation_bar_height: {} px", navigation_bar_height);

    // 保存先ディレクトリを作成
    if !config::SCREENSHOT_DIR.exists() {
        fs::create_dir(&*config::SCREENSHOT_DIR).map_err(|e| format!("Failed to create screenshots directory: {}", e))?;
    }

    // 1. 最初のスクリーンショット（ヘッダーあり）を撮影
    let mut screenshots = vec![];
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await; // 読み込み待ち
    let first_screenshot = client.screenshot().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;

    let cropped_first_screenshot = crop_image(&first_screenshot, navigation_bar_height)?;
    screenshots.push(cropped_first_screenshot.clone());

    // 個別保存
    fs::write(config::SCREENSHOT_DIR.join("screenshot_0.png"), &first_screenshot)
        .map_err(|e| format!("Failed to save screenshot_0.png: {}", e))?;

    // 2. 指定した要素を非表示にする
    if !hidden_elements.trim().is_empty() {
        let hide_script = format!(
            r#"
            (function() {{
                let elements = document.querySelectorAll("{}");
                elements.forEach(e => {{
                    e.dataset.originalPosition = e.style.position;
                    e.style.position = 'static';
                }});
            }})();
            "#,
            hidden_elements
        );
        client.execute(&hide_script, vec![]).await.map_err(|e| format!("Failed to set position: static: {}", e))?;
    }

    // 3. スクロールしながらスクリーンショット
    let get_scroll_position = r#"return window.scrollY;"#;
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
        let scroll_script = format!(r#"window.scrollBy(0, {});"#, scroll_amount);
        client.execute(&scroll_script, vec![]).await.map_err(|e| format!("Failed to scroll: {}", e))?;
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await; // スクロール完了を待つ

        // 新しいスクロール位置を取得
        let new_y_offset = client.execute(get_scroll_position, vec![]).await
        .map_err(|e| format!("Failed to get scroll position: {}", e))?
        .as_f64().unwrap_or(y_offset);

        println!("Scrolled to: {}", new_y_offset);

        // `y_offset` が変わらない（スクロールの余地なし）ならループを抜ける
        if new_y_offset == y_offset {
            println!("No further scrolling detected, stopping.");
            break;
        }

        y_offset = new_y_offset;

        // スクリーンショットを撮る
        let screenshot: Vec<u8> = client.screenshot().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;
        // 最後のスクロール時は、被った部分をカット
        let cropped_screenshot = if y_offset + viewport_height > height {
             // 被った部分をカットしてから余白をカット
            let temp = crop_bottom(&screenshot, remaining_scroll)?;
            crop_image(&temp, navigation_bar_height)?
        } else {
            crop_image(&screenshot, navigation_bar_height)?
        };

        screenshots.push(cropped_screenshot.clone());

        let filename = format!("screenshot_{}.png", index);
        fs::write(config::SCREENSHOT_DIR.join(filename), &cropped_screenshot)
            .map_err(|e| format!("Failed to save screenshot_{}: {}", index, e))?;

        index += 1;
    }

    // 4. 非表示にした要素を元に戻す
    if !hidden_elements.trim().is_empty() {
        let show_script = format!(
            r#"
            (function() {{
                let elements = document.querySelectorAll("{}");
                elements.forEach(e => {{
                    e.style.position = e.dataset.originalPosition || '';
                    delete e.dataset.originalPosition;
                }});
            }})();
            "#,
            hidden_elements
        );
        client.execute(&show_script, vec![]).await.map_err(|e| format!("Failed to restore position: {}", e))?;
    }

    Ok(screenshots)
}


// 画像の下を navigation_bar_height 分カットする関数
fn crop_image(image_data: &[u8], navigation_bar_height: f64) -> Result<Vec<u8>, String> {
    println!("crop_image");
    println!("navigation_bar_height: {}", navigation_bar_height);
    let image = image::load_from_memory(image_data).map_err(|e| format!("Failed to load image: {}", e))?;
    let (width, height) = image.dimensions();

    let width = width as f64;
    let height = height as f64;

    if height <= navigation_bar_height {
        return Err("Image height is smaller than navigation bar height, cannot crop.".to_string());
    }

    let cropped_height = (height - navigation_bar_height) as u32;
    let cropped_image = image.view(0, 0, width as u32, cropped_height).to_image();

    let mut output = std::io::Cursor::new(Vec::new());
    cropped_image.write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(output.into_inner())
}


// 最後のスクロールで、被った部分をカットする関数
fn crop_bottom(image_data: &[u8], crop_height: f64) -> Result<Vec<u8>, String> {
    let image = image::load_from_memory(image_data).map_err(|e| format!("Failed to load image: {}", e))?;
    let (width, height) = image.dimensions();
    let height = height as f64;

    if height <= crop_height {
        return Err("Image height is smaller than crop height, cannot crop.".to_string());
    }

    let new_height = (height - crop_height) as u32;
    let cropped_image = image.view(0, crop_height as u32, width, new_height).to_image();

    let mut output = std::io::Cursor::new(Vec::new());
    cropped_image.write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(output.into_inner())
}


// スクリーンショットを結合する関数
pub fn combine_screenshots(screenshots: Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
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
    println!("Combining {} images, total height: {} px", screenshots.len(), total_height);

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
