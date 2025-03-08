use std::path::Path;
use std::sync::LazyLock;

pub const SCREENSHOT_DIR: LazyLock<&Path> = LazyLock::new(|| Path::new("../screenshots"));
