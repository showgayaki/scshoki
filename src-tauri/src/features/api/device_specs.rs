use log::info;

use super::client::fetch_data;
use super::types::DeviceSpecs;

const BASE_URL: &str = "https://idevice-specs.pages.dev/api/device-specs";

pub async fn fetch_device_specs(product_type: Option<&str>) -> Result<DeviceSpecs, String> {
    let url = match product_type {
        Some(pt) => format!("{}?productType={}", BASE_URL, pt),
        None => BASE_URL.to_string(),
    };
    info!("Fetch idevice specs from: {}", url);

    fetch_data(&url).await.map_err(|e| e.to_string())
}
