use std::collections::HashMap;
use std::process::Command;
use std::sync::LazyLock;

use log::info;

struct Dependency {
    name: String,
    command: String,
}

static IOS_DEPENDENCIES: LazyLock<[Dependency; 3]> = LazyLock::new(|| {
    [
        Dependency {
            name: String::from("libimobiledevice"),
            command: String::from("idevice_id"),
        },
        Dependency {
            name: String::from("ideviceinstaller"),
            command: String::from("ideviceinstaller"),
        },
        Dependency {
            name: String::from("ios-webkit-debug-proxy"),
            command: String::from("ios_webkit_debug_proxy"),
        },
    ]
});

fn is_installed(binary_name: &str) -> bool {
    Command::new(binary_name)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn is_dependencies_installed() -> Vec<HashMap<String, bool>> {
    let mut results = Vec::new();

    for binary in IOS_DEPENDENCIES.iter() {
        let mut command_result = HashMap::new();
        command_result.insert(binary.name.to_string(), is_installed(&binary.command));

        results.push(command_result);
    }

    info!("iOS dependencies installed: {:?}", results);
    results
}
