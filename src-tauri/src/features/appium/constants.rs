use std::time::Duration;

pub const APPIUM_TIMEOUT: Duration = Duration::from_secs(10);

pub const APPIUM_VER: &str = "2.17.1";
pub const DRIVER_LIST: [&str; 3] = ["uiautomator2@4.1.5", "gecko@1.4.3", "xcuitest@9.1.2"];
pub const WDA_IDENTIFIER: &str = "com.facebook.WebDriverAgentRunner";
