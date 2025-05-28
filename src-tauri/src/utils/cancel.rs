use log::info;
use thirtyfour::prelude::*;
use tokio_util::sync::CancellationToken;

/// チェックしてキャンセルされていれば早期リターンする共通関数
pub async fn check_cancellation(
    token: &CancellationToken,
    driver: Option<&WebDriver>,
) -> Result<(), String> {
    if token.is_cancelled() {
        info!("Cancellation token is cancelled.");
        if let Some(driver) = driver {
            info!("Cancelled. Quitting driver...");
            driver.clone().quit().await.ok(); // クリーンアップ
        }
        return Err("Screenshot cancelled".to_string());
    }
    Ok(())
}
