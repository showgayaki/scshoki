use std::collections::HashMap;
use std::sync::LazyLock;

pub static BROWSER_SCHEMES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut schemes = HashMap::new();
    schemes.insert("chrome", "googlechrome://");
    schemes.insert("firefox", "firefox://open-url?url=");
    schemes.insert("edge", "microsoft-edge://");
    schemes
});
