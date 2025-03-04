use appium_client::Client;
use appium_client::capabilities::android::AndroidCapabilities;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::fs;
use std::path::Path;

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

    // let scroll_script = r#"window.scrollBy(0, 800);"#;

    const OFFSET: i64 = 800;
    let scroll_script: String = format!(r#"window.scrollBy(0, {});"#, OFFSET);

    let mut y_offset: i64 = 0;
    let mut screenshots: Vec<Vec<u8>> = vec![];

    // 保存先ディレクトリを作成
    let screenshot_dir = Path::new("screenshots");
    if !screenshot_dir.exists() {
        fs::create_dir(screenshot_dir).map_err(|e| format!("Failed to create screenshots directory: {}", e))?;
    }

    // **1. 最初のスクリーンショット（ヘッダーあり）を撮影**
    let first_screenshot = client.screenshot().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;
    screenshots.push(first_screenshot.clone());

    // **個別保存**
    fs::write(screenshot_dir.join("screenshot_0.png"), &first_screenshot)
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
        client.execute(&scroll_script, vec![]).await.map_err(|e| format!("Failed to scroll: {}", e))?;
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let screenshot = client.screenshot().await.map_err(|e| format!("Failed to take screenshot: {}", e))?;
        screenshots.push(screenshot.clone());

        // **個別保存**
        let filename = format!("screenshot_{}.png", index);
        fs::write(screenshot_dir.join(filename), &screenshot)
            .map_err(|e| format!("Failed to save screenshot_{}: {}", index, e))?;

        y_offset += OFFSET;
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
