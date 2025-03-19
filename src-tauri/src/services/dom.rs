use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;
use thirtyfour::prelude::*;

pub async fn get_page_metrics(driver: &WebDriver) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    debug!("Getting page metrics...");

    let script = r#"
        const innerHeight = window.innerHeight;
        const totalScrollHeight = document.documentElement.scrollHeight
        return {
            innerHeight: window.innerHeight,
            totalScrollHeight: document.documentElement.scrollHeight,
            scrollSteps: Math.ceil(totalScrollHeight / innerHeight),
        };
    "#;

    let result = driver.execute(script, vec![]).await?;
    let result_map = result
        .json()
        .as_object()
        .ok_or("Failed to convert script result to map")?
        .iter()
        .map(|(k, v)| (k.clone(), v.as_f64().unwrap_or(0.0)))
        .collect();

    info!("Page metrics: {:?}", result_map);
    Ok(result_map)
}

// 指定した要素を `display: none;` に設定し非表示にする
pub async fn hide_elements(driver: &WebDriver, selectors: &str) -> Result<(), Box<dyn Error>> {
    debug!("Hiding elements: {}", selectors);

    if selectors.trim().is_empty() {
        info!("No elements to hide.");
        return Ok(());
    }

    let script = format!(
        r#"
        (function() {{
            let elements = document.querySelectorAll("{}");
            elements.forEach(e => e.style.visibility = 'hidden');
        }})();
        "#,
        selectors
    );

    driver.execute(&script, vec![]).await?;
    info!("Elements hidden: {}", selectors);
    Ok(())
}

// 指定した要素を元の状態に戻す（`display` プロパティをクリア）
pub async fn show_elements(driver: &WebDriver, selectors: &str) -> Result<(), Box<dyn Error>> {
    debug!("Restoring elements: {}", selectors);
    if selectors.trim().is_empty() {
        return Ok(());
    }

    let script = format!(
        r#"
        (function() {{
            let elements = document.querySelectorAll("{}");
            elements.forEach(e => e.style.visibility = '');
        }})();
        "#,
        selectors
    );

    driver.execute(&script, vec![]).await?;
    info!("Elements restored: {}", selectors);
    Ok(())
}

// 現在のスクロール位置を取得
pub async fn get_scroll_position(driver: &WebDriver) -> Result<f64, Box<dyn Error>> {
    debug!("get_scroll_position");
    let script = "return window.scrollY;";
    let result = driver.execute(script, vec![]).await?;
    Ok(result.json().as_f64().unwrap_or(0.0))
}

// 指定したピクセル分スクロールする
pub async fn scroll_by(driver: &WebDriver, pixels: f64) -> Result<(), Box<dyn Error>> {
    debug!("scroll_by");
    let script = format!("window.scrollBy(0, {});", pixels);
    driver.execute(&script, vec![]).await?;
    Ok(())
}
