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
