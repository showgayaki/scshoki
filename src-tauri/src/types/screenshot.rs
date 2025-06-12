use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

pub struct WebdriverParams<'a> {
    pub appium_server_url: &'a str,
    pub device_os: &'a str,
    pub device_os_version: &'a str,
    pub device_udid: &'a str,
    pub browser: &'a str,
    pub base_url: &'a str,
    pub token: &'a CancellationToken,
}

// スクリーンショットのパラメータを定義
// React用にエクスポートされるので、パラメーターの更新があったら、ここだけ変えればOK
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../../src/generated/ScreenshotParams.ts")]
pub struct ScreenshotParams {
    pub base_url: String,
    pub target_page_paths: Vec<String>,
    pub hidden_elements: Vec<String>,
    pub selected_browsers: Vec<String>,
    pub basic_auth_username: Option<String>,
    pub basic_auth_password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../../src/generated/ScreenshotResponse.ts")]
pub struct ScreenshotResponse {
    pub success: bool,
    pub cancelled: bool,
    pub path: String,
    pub error: Option<String>,
}

pub struct ScreenshotContext<'a> {
    pub driver: &'a WebDriver,
    pub hidden_elements: &'a [String],
    pub datetime_now: &'a str,
    pub device_os: &'a str,
    pub browser: &'a str,
    pub page_path: &'a str,
    pub navigationbar_height: f64,
    pub token: &'a CancellationToken,
}
