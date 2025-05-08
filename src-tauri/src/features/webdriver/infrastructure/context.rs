use log::{debug, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

/// Appium経由で現在のcontextsを取得（例: ["NATIVE_APP", "WEBVIEW_com.apple.mobilesafari"]）
pub async fn get_contexts(
    session_id: &str,
    appium_url: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    #[derive(Debug, Deserialize)]
    struct ContextsResponse {
        value: Vec<String>,
    }

    let endpoint = format!(
        "{}/session/{}/contexts",
        appium_url.trim_end_matches('/'),
        session_id
    );
    let client = Client::new();
    let response = client.get(&endpoint).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to get contexts: {}", response.status()).into());
    }

    let body: ContextsResponse = response.json().await?;

    info!("Available contexts: {:?}", body.value);
    Ok(body.value)
}

/// Appiumでcontext（例: "WEBVIEW_660.4"）を切り替える
pub async fn set_context(
    session_id: &str,
    appium_url: &str,
    context_name: &str,
) -> Result<(), Box<dyn Error>> {
    #[derive(Serialize)]
    struct SetContextRequest {
        name: String,
    }

    let endpoint = format!(
        "{}/session/{}/context",
        appium_url.trim_end_matches('/'),
        session_id
    );
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    let body = SetContextRequest {
        name: context_name.to_string(),
    };
    let response = client.post(&endpoint).json(&body).send().await?;

    if response.status().is_success() {
        debug!("Context set to: {}", context_name);
    } else {
        return Err(format!("Failed to set context: {}", response.status()).into());
    }

    Ok(())
}
