use log::debug;
use tauri::command;

use crate::usecases::screenshot;

#[command]
pub async fn take_screenshot(
    url: String,
    hidden_elements: String,
    selected_browsers: Vec<String>,
) -> Result<(), String> {
    debug!(
        "take_screenshot command called with url: {}, hidden_elements: {}, selected_browsers: {:?}",
        url, hidden_elements, selected_browsers
    );
    screenshot::take_screenshot(url, hidden_elements, selected_browsers).await
}
