use std::fs::File;
use std::io::{ BufReader, BufRead };
use reqwest::Client;
use console::style;
use anyhow::{ Context, Result };

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
    let response = client
        .get(format!("{}{}", npm_registry, name))
        .send().await
        .context(format!("error sending request url ({}{})", npm_registry, name))?;
    // 检查请求是否成功
    if response.status().is_success() {
        let body = response.text().await?;
        // 解析 JSON 响应，获取版本信息等
        let package_info: serde_json::Value = serde_json::from_str(&body)?;
        let latest_version = package_info["dist-tags"]["latest"].as_str().unwrap_or("");
        return anyhow::Ok(latest_version.to_string());
    } else {
        return anyhow::Ok("".to_string());
    }
}

// 比较版本号
pub fn compare_versions(current_version: &str, latest_version: &str) {
    let current_parts: Vec<i32> = current_version
        .split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    let latest_parts: Vec<i32> = latest_version
        .split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    for (current, latest) in current_parts.iter().zip(latest_parts.iter()) {
        if current < latest {
            print_version(current_version, latest_version);
            return;
        }
    }
}

// 输出版本号
fn print_version(current_version: &str, latest_version: &str) {
    let borders = Borders {
        tl: &format!("{}", style("┌").yellow()),
        tr: &format!("{}", style("┐").yellow()),
        bl: &format!("{}", style("└").yellow()),
        br: &format!("{}", style("┘").yellow()),
        h: &format!("{}", style("─").yellow()),
        v: &format!("{}", style("│").yellow()),
    };
    let header = format!("{}", style("  发现新版本!  ").green());
    let footer = format!("  请使用 `{}` 更新  ", style("npm install -g elza-cli").magenta());
    let current_version_text = format!(
        "   > 当前版本: {}  ",
        style(format!("v{}", current_version)).red()
    );
    let latest_version_text = format!(
        "   > 最新版本: {}  ",
        style(format!("v{}", latest_version)).green().bold()
    );
    let header_len = get_string_length(&header) - 9;
    let footer_len = get_string_length(&footer) - 9;
    let current_len = get_string_length(&current_version_text) - 9;
    let latest_len = get_string_length(&latest_version_text) - 13;
    let max_len = header_len.max(footer_len).max(current_len).max(latest_len);
    let before_lines = vec![
        format!("{}{}{}", borders.tl, borders.h.repeat(max_len), borders.tr),
        format!("{}{}{}{}", borders.v, header, " ".repeat(max_len - header_len), borders.v),
        format!(
            "{}{}{}{}",
            borders.v,
            current_version_text,
            " ".repeat(max_len - current_len),
            borders.v
        )
    ];
    let main = format!(
        "{}{}{}{}",
        borders.v,
        latest_version_text,
        " ".repeat(max_len - latest_len),
        borders.v
    );
    let after_lines = vec![
        format!("{}{}{}", borders.v, " ".repeat(max_len), borders.v),
        format!("{}{}{}{}", borders.v, footer, " ".repeat(max_len - footer_len), borders.v),
        format!("{}{}{}", borders.bl, borders.h.repeat(max_len), borders.br)
    ];
    let mut before = vec![];
    for line in before_lines {
        before.push(format!("{}{}", " ".repeat(8), line));
    }
    let mut after = vec![];
    for line in after_lines {
        after.push(format!("{}{}", " ".repeat(8), line));
    }
    let before = before.join("\n");
    let after = after.join("\n");
    println!("{}", before);
    logger::info(&main);
    println!("{}", after);
}

// 获取字符串长度
fn get_string_length(s: &str) -> usize {
    let s = String::from(s);
    let vec: Vec<char> = s.chars().collect();
    let mut len = 0;
    for c in vec {
        if is_chinese(c) {
            len += 2;
        } else {
            len += 1;
        }
    }
    len
}

// 是否为中文字符
fn is_chinese(c: char) -> bool {
    return c >= '\u{4e00}' && c <= '\u{9fa5}';
}
