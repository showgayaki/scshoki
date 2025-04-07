use std::sync::LazyLock;
use std::time::Duration;

pub const APPIUM_PORT: &str = "4723";
pub static APPIUM_SERVER_URL: LazyLock<String> =
    LazyLock::new(|| format!("http://127.0.0.1:{APPIUM_PORT}"));
pub const APPIUM_TIMEOUT: Duration = Duration::from_secs(10);

pub const APPIUM_VER: &str = "2.17.1";
pub const DRIVER_LIST: [&str; 4] = [
    "uiautomator2@4.1.5",
    "gecko@1.4.3",
    "xcuitest@9.1.2",
    "safari@3.5.23",
];
pub const WDA_IDENTIFIER: &str = "com.facebook.WebDriverAgentRunner";
