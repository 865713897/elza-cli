use std::process::Command;
use std::str;
use serde_json::Value;
use anyhow::{ Ok, Result };

use super::error::handle_result;
use super::logger;

// 获取用户当前包的版本
pub fn get_current_version() -> Result<String> {
    let output = handle_result(
        Command::new("npm")
            .arg("ls")
            .arg("-g")
            .arg("elza-cli")
            .arg("--depth=0")
            .arg("--json")
            .output(),
        "Failed to execute npm ls command"
    );

    if !output.status.success() {
        logger::warn("获取当前版本失败");
    }

    let output_str = str::from_utf8(&output.stdout).unwrap();
    let json: Value = serde_json::from_str(&output_str).unwrap();
    let version = json["dependencies"]["elza-cli"]["version"].as_str().unwrap();
    
    Ok(version.to_string())
}

