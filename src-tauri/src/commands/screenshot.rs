use appium_client::ClientBuilder;
use appium_client::capabilities::android::AndroidCapabilities;
use appium_client::commands::AppiumCommand;
use http::Method;
use std::fs;
use tauri::command;
use crate::commands::utils::{capture_full_page, combine_screenshots};

#[command]
pub async fn take_screenshot(url: String, hidden_elements: String) -> Result<(), String> {
    let mut capabilities = AndroidCapabilities::new_uiautomator();
    capabilities.insert("appium:platformName".to_string(), "Android".into());
    capabilities.insert("appium:deviceName".to_string(), "your_device_name".into());
    capabilities.insert("appium:browserName".to_string(), "Chrome".into());
    capabilities.insert("appium:newCommandTimeout".to_string(), 300.into());

    let client = ClientBuilder::native(capabilities)
        .connect("http://127.0.0.1:4723/")
        .await
        .map_err(|e| format!("Failed to connect to Appium: {}", e))?;

    let formatted_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("http://{}", url)
    };

    client.goto(&formatted_url).await.map_err(|e| format!("Failed to navigate to URL: {}", e))?;

    // **スクロールしながらスクリーンショットを撮影**
    let screenshots = capture_full_page(&client, &hidden_elements).await?;

    let final_screenshot = combine_screenshots(screenshots)?;
    fs::write("screenshot.png", final_screenshot).map_err(|e| format!("Failed to save screenshot: {}", e))?;

    println!("スクリーンショットを保存しました");

    // **セッションを終了**
    if let Err(e) = client.issue_cmd(AppiumCommand::Custom(
        Method::DELETE,
        "".to_string(),
        None
    )).await {
        eprintln!("Failed to quit session: {}", e);
    }

    Ok(())
}
