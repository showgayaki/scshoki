use std::path::Path;
use std::sync::LazyLock;


pub const SCREENSHOT_DIR: LazyLock<&Path> = LazyLock::new(|| Path::new("../screenshots"));

pub const LOG_DIR: LazyLock<&Path> = LazyLock::new(|| Path::new("../log"));
pub const LOG_FILE_NAME: &str = "scshoki.log";
