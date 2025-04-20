use reqwest::Client;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Unexpected status code: {0}")]
    StatusCode(u16),
}

pub async fn fetch_data<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, ApiError> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let resp = client.get(url).send().await?;

    if !resp.status().is_success() {
        return Err(ApiError::StatusCode(resp.status().as_u16()));
    }

    let json = resp.json::<T>().await?;
    Ok(json)
}
