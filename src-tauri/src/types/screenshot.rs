use serde::{Deserialize, Serialize};
use ts_rs::TS;

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
