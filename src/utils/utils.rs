use std::fs::File;
use std::io::{ BufReader, BufRead };
use reqwest::Client;
use console::style;
use anyhow::Result;

use super::logger;

const DEFAULT_NPM_REGISTRY: &str = "https://registry.npmjs.org/";

struct Borders<'a> {
    tl: &'a str,
    tr: &'a str,
    bl: &'a str,
    br: &'a str,
    h: &'a str,
    v: &'a str,
}

pub fn get_user_npm_registry() -> String {
    // 获取home目录
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            logger::warning("未找到home目录");
            return DEFAULT_NPM_REGISTRY.to_string();
        }
    };

    // npm 配置文件路径
    let npmrc_path = home_dir.join(".npmrc");

    // 打开配置文件
    let file = match File::open(&npmrc_path) {
        Ok(file) => file,
        Err(_) => {
            logger::warning("打开.npmrc配置文件失败");
            return DEFAULT_NPM_REGISTRY.to_string();
        }
    };

    // 读取配置文件中的镜像设置
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("registry=") {
                let registry_url = line.trim_start_matches("registry=").trim();
                return registry_url.to_string();
            }
        }
    }

    DEFAULT_NPM_REGISTRY.to_string()
}

// 获取最新版本
pub async fn get_latest_version(name: &str) -> Result<String> {
    // 创建一个 reqwest 客户端
    let client = Client::new();
    let npm_registry = get_user_npm_registry();
    let response = client.get(format!("{}{}", npm_registry, name)).send().await?;
    // 检查请求是否成功
    if response.status().is_success() {
        let body = response.text().await?;
        // 解析 JSON 响应，获取版本信息等
        let package_info: serde_json::Value = serde_json::from_str(&body)?;
        let latest_version = package_info["dist-tags"]["latest"].as_str().unwrap_or("unknown");
        return anyhow::Ok(latest_version.to_string());
    } else {
        return anyhow::Ok("".to_string());
    }
}

// 比较版本号
pub fn compare_versions(current_version: &str, latest_version: &str) {
    let current_parts: Vec<&str> = current_version.split(".").collect();
    let latest_parts: Vec<&str> = latest_version.split(".").collect();
    for n in 1..current_parts.len() {
        if current_parts[n].parse::<i32>().unwrap() < latest_parts[n].parse::<i32>().unwrap() {
            print_version(current_version, latest_version);
            return;
        }
    }
}

// 输出版本号
fn print_version(current_version: &str, latest_version: &str) {
    let borders = Borders {
        tl: &format!("{}", style("╔").yellow()),
        tr: &format!("{}", style("╗").yellow()),
        bl: &format!("{}", style("╚").yellow()),
        br: &format!("{}", style("╝").yellow()),
        h: &format!("{}", style("═").yellow()),
        v: &format!("{}", style("║").yellow()),
    };
    let tip = "  发现新版本！  ";
    let current_version_text = format!("  当前版本: {}  ", style(current_version).red());
    let latest_version_text = format!("  最新版本: {}  ", style(latest_version).green().bold());
    let max_len = 20;

    let header_line = format!("{}{}{}", borders.tl, borders.h.repeat(max_len), borders.tr);
    let tip_line = format!(
        "{}{}{}{}",
        borders.v,
        style(tip).cyan(),
        " ".repeat(max_len - 16),
        borders.v
    );
    let current_version_line = format!("{}{}{}", borders.v, current_version_text, borders.v);
    let latest_version_line = format!("{}{}{}", borders.v, latest_version_text, borders.v);
    let footer_line = format!("{}{}{}", borders.bl, borders.h.repeat(max_len), borders.br);

    println!(
        "{}\n{}\n{}\n{}\n{}",
        header_line,
        tip_line,
        current_version_line,
        latest_version_line,
        footer_line
    );
}
