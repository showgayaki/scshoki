use log::debug;
use reqwest::Client;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Unexpected status code: {0}")]
    StatusCode(u16),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub async fn fetch_data<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, ApiError> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let resp = client.get(url).send().await?;

    let status = resp.status(); // 先に status を保存しておく
    let raw_text = resp.text().await?;
    debug!("Raw JSON: {}", raw_text);

    if !status.is_success() {
        return Err(ApiError::StatusCode(status.as_u16()));
    }

    let json = serde_json::from_str::<T>(&raw_text)?;
    Ok(json)
}
