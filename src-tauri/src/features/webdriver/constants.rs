use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::constants::BINARY_DIR;

pub static CHROME_DRIVER_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| BINARY_DIR.join("chromedriver"));

pub static BROWSER_SCHEMES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut schemes = HashMap::new();
    schemes.insert("chrome", "googlechrome://");
    schemes.insert("firefox", "firefox://open-url?url=");
    schemes.insert("edge", "microsoft-edge://");
    schemes
});

// iOSのSafari以外のブラウザ用
// ページ下部のナビゲーションバーの高さを取るためのエレメントのIdentifier
pub static NAVIGATION_ELEMTNT_FOR_HEIGHT: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut element_name = HashMap::new();
    element_name.insert("chrome", "kToolbarToolsMenuButtonIdentifier");
    element_name.insert("firefox", "TabToolbar.homeButton");
    element_name.insert("edge", "");
    element_name
});
