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

pub static WEBVIEW_BUNDLE_IDS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut ids = HashMap::new();
    ids.insert("safari", "com.apple.mobilesafari");
    ids.insert("chrome", "com.google.chrome.ios");
    ids.insert("firefox", "org.mozilla.ios.Firefox");
    ids.insert("edge", "");
    ids
});

// iOSのSafari以外のブラウザ用
// ページ下部のナビゲーションバーの高さを取るためのエレメントのIdentifier
pub struct NavigationElement {
    pub identifier: &'static str,
    pub default_height: f64,
}

pub static NAVIGATION_ELEMTNT_FOR_HEIGHT: LazyLock<HashMap<&str, NavigationElement>> =
    LazyLock::new(|| {
        let mut element_name = HashMap::new();
        element_name.insert(
            "chrome",
            NavigationElement {
                identifier: "kToolbarToolsMenuButtonIdentifier",
                default_height: 44.0,
            },
        );
        element_name.insert(
            "firefox",
            NavigationElement {
                identifier: "TabToolbar.homeButton",
                default_height: 47.0,
            },
        );
        element_name.insert(
            "edge",
            NavigationElement {
                identifier: "",
                default_height: 0.0,
            },
        );
        element_name
    });
