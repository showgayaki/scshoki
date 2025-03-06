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
            availHeight: screen.availHeight,
            viewportHeight: window.innerHeight,
            scrollHeight: document.documentElement.scrollHeight,
        };
    "#;

    let result = client.execute(script, vec![]).await.map_err(|e| format!("Failed to execute script: {}", e))?;
    let height = result.get("height").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let avail_height = result.get("availHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let viewport_height = result.get("viewportHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);
    let scroll_height = result.get("scrollHeight").and_then(|h| h.as_f64()).unwrap_or(0.0);

    if height <= 0.0 {
        return Err("Failed to retrieve page height.".to_string());
    }

    println!("Debug info:");
    println!("  availHeight: {} px", avail_height);
    println!("  height: {} px", height);
    println!("  viewport_height: {} px", viewport_height);
    println!("  scroll_height: {} px", scroll_height);

    // 保存先ディレクトリを作成
    if !config::SCREENSHOT_DIR.exists() {
        fs::create_dir(&*config::SCREENSHOT_DIR).map_err(|e| format!("Failed to create screenshots directory: {}", e))?;
    }

    // 1. 最初のスクリーンショット（ヘッダーあり）を撮影
    let mut screenshots = vec![];
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await; // 読み込み待ち
    let first_screenshot = client.screenshot().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;
    screenshots.push(first_screenshot.clone());

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
    let scroll_script = format!(r#"window.scrollBy(0, {});"#, viewport_height);
    let get_scroll_position = r#"return window.scrollY;"#;
    let mut y_offset = 0.0;
    let mut index = 1;

    while y_offset < height {
        // スクロール実行
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

        // スクロール後スクリーンショットを撮る
        let screenshot: Vec<u8> = client.screenshot().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;
        // let cropped_screenshot = crop_image(&screenshot, navigation_bar_height)?;
        screenshots.push(screenshot.clone());

        let filename = format!("screenshot_{}.png", index);
        fs::write(config::SCREENSHOT_DIR.join(filename), &screenshot)
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


pub fn combine_screenshots(screenshots: Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
    if screenshots.is_empty() {
        return Err("No screenshots to combine".to_string());
    }

    let first_image = image::load_from_memory(&screenshots[0]).map_err(|e| e.to_string())?;
    let (width, height) = first_image.dimensions();
    let total_height = height * screenshots.len() as u32;

    let mut combined_image = ImageBuffer::new(width, total_height);

    for (i, screenshot) in screenshots.iter().enumerate() {
        let image = image::load_from_memory(screenshot).map_err(|e| e.to_string())?;
        let y_offset = i as u32 * height;
        for (x, y, pixel) in image.pixels() {
            combined_image.put_pixel(x, y + y_offset, pixel);
        }
    }

    let mut output = std::io::Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(combined_image)
        .write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(output.into_inner())
}
