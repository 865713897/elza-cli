use std::process::exit;
use anyhow::Result;
use crate::utils::logger;

// 通用错误处理函数
pub fn handle_option<T>(option: Option<T>, error_msg: &str) -> T {
    match option {
        Some(value) => value,
        None => {
            logger::error(error_msg);
            exit(1);
        }
    }
}

// 错误处理结果
pub fn handle_result<T, E: std::fmt::Debug>(result: Result<T, E>, error_msg: &str) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            logger::error(&format!("{}: {:?}", error_msg, err));
            exit(1);
        }
    }
}
