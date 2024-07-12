use std::fs::File;
use std::io::{ BufReader, BufRead };

use super::logger;

pub fn get_user_npm_registry() -> String {
    // 获取home目录
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            logger::warning("未找到home目录");
            return "".to_string();
        }
    };

    // npm 配置文件路径
    let npmrc_path = home_dir.join(".npmrc");

    // 打开配置文件
    let file = match File::open(&npmrc_path) {
        Ok(file) => file,
        Err(_) => {
            logger::warning("打开.npmrc配置文件失败");
            return "".to_string();
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

    "https://registry.npmjs.org/".to_string()
}
