use appium_client::Client;
use appium_client::capabilities::android::AndroidCapabilities;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::fs;

use crate::config;

pub async fn capture_full_page(client: &Client<AndroidCapabilities>, hidden_elements: &str) -> Result<Vec<Vec<u8>>, String> {
    let script = r#"
        try {
            const body = document.body;
            const html = document.documentElement;
            return Math.max(body.scrollHeight || 0, body.offsetHeight || 0,
                            html.clientHeight || 0, html.scrollHeight || 0, html.offsetHeight || 0);
        } catch (e) {
            return 0;
        }
    "#;

    let height: i64 = client.execute(script, vec![]).await
        .map_err(|e| format!("Failed to execute script: {}", e))?
        .as_i64().unwrap_or(0);

    if height <= 0 {
        return Err("Failed to retrieve page height.".to_string());
    }

    let scroll_script = r#"window.scrollBy(0, 800);"#;
    let get_scroll_position = r#"return window.scrollY;"#;

    let mut y_offset = 0;
    let mut screenshots = vec![];

    // 保存先ディレクトリを作成
    if !config::SCREENSHOT_DIR.exists() {
        fs::create_dir(&*config::SCREENSHOT_DIR).map_err(|e| format!("Failed to create screenshots directory: {}", e))?;
    }

    // **1. 最初のスクリーンショット（ヘッダーあり）を撮影**
    let first_screenshot = client.screenshot().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;
    screenshots.push(first_screenshot.clone());

    // **個別保存**
    fs::write(config::SCREENSHOT_DIR.join("screenshot_0.png"), &first_screenshot)
        .map_err(|e| format!("Failed to save screenshot_0.png: {}", e))?;

    // **2. 指定した要素を非表示にする**
    if !hidden_elements.trim().is_empty() {
        let hide_script = format!(
            r#"
            (function() {{
                let elements = document.querySelectorAll("{}");
                elements.forEach(e => e.style.display = 'none');
            }})();
            "#,
            hidden_elements
        );
        client.execute(&hide_script, vec![]).await.map_err(|e| format!("Failed to hide elements: {}", e))?;
    }

    // **3. スクロールしながらスクリーンショット**
    let mut index = 1;
    while y_offset < height {
        // スクロール実行
        client.execute(&scroll_script, vec![]).await.map_err(|e| format!("Failed to scroll: {}", e))?;
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await; // **スクロール後の待機時間を増やす**

        // **現在のスクロール位置を取得**
        let new_y_offset: i64 = client.execute(get_scroll_position, vec![]).await
            .map_err(|e| format!("Failed to get scroll position: {}", e))?
            .as_i64().unwrap_or(y_offset);

        println!("Scrolled to: {}", new_y_offset);

        // **スクロール位置が変わらなかった場合、ループを終了**
        if new_y_offset == y_offset {
            println!("No further scrolling detected, stopping.");
            break;
        }

        y_offset = new_y_offset;

        // **スクリーンショットを撮影**
        let screenshot = client.screenshot().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;
        screenshots.push(screenshot.clone());

        // **個別保存**
        let filename = format!("screenshot_{}.png", index);
        fs::write(config::SCREENSHOT_DIR.join(filename), &screenshot)
            .map_err(|e| format!("Failed to save screenshot_{}: {}", index, e))?;

        index += 1;
    }

    // **4. 非表示にした要素を元に戻す**
    if !hidden_elements.trim().is_empty() {
        let show_script = format!(
            r#"
            (function() {{
                let elements = document.querySelectorAll("{}");
                elements.forEach(e => e.style.display = '');
            }})();
            "#,
            hidden_elements
        );
        client.execute(&show_script, vec![]).await.map_err(|e| format!("Failed to restore elements: {}", e))?;
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
