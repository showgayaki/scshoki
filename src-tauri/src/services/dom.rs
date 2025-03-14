use thirtyfour::prelude::*;
use std::collections::HashMap;
use std::error::Error;


pub async fn get_page_metrics(driver: &WebDriver) -> Result<HashMap<String, f64>, Box<dyn Error>> {
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
            scrollHeight: document.documentElement.scrollHeight
        };
    "#;

    let result = driver.execute(script, vec![]).await?;
    let result_map = result.json().as_object()
        .ok_or("Failed to convert script result to map")?
        .iter()
        .map(|(k, v)| (k.clone(), v.as_f64().unwrap_or(0.0)))
        .collect();

    Ok(result_map)
}


// 指定した要素を `display: none;` に設定し非表示にする
pub async fn hide_elements(driver: &WebDriver, selectors: &str) -> Result<(), Box<dyn Error>> {
    if selectors.trim().is_empty() {
        return Ok(());
    }

    let script = format!(
        r#"
        (function() {{
            let elements = document.querySelectorAll("{}");
            elements.forEach(e => e.style.display = 'none');
        }})();
        "#,
        selectors
    );

    driver.execute(&script, vec![]).await?;
    println!("Elements hidden: {}", selectors);
    Ok(())
}


// 指定した要素を元の状態に戻す（`display` プロパティをクリア）
pub async fn show_elements(driver: &WebDriver, selectors: &str) -> Result<(), Box<dyn Error>> {
    if selectors.trim().is_empty() {
        return Ok(());
    }

    let script = format!(
        r#"
        (function() {{
            let elements = document.querySelectorAll("{}");
            elements.forEach(e => e.style.display = '');
        }})();
        "#,
        selectors
    );

    driver.execute(&script, vec![]).await?;
    println!("Elements restored: {}", selectors);
    Ok(())
}


// 現在のスクロール位置を取得
pub async fn get_scroll_position(driver: &WebDriver) -> Result<f64, Box<dyn Error>> {
    let script = "return window.scrollY;";
    let result = driver.execute(script, vec![]).await?;
    Ok(result.json().as_f64().unwrap_or(0.0))
}


// 指定したピクセル分スクロールする
pub async fn scroll_by(driver: &WebDriver, pixels: f64) -> Result<(), Box<dyn Error>> {
    let script = format!("window.scrollBy(0, {});", pixels);
    driver.execute(&script, vec![]).await?;
    println!("Scrolled by {} pixels", pixels);
    Ok(())
}
