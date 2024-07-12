mod utils;
mod core;
use anyhow::{ Ok, Result };
use console::style;
use tokio::runtime::Runtime;
use lazy_static::lazy_static;
use clap::{
    builder::{ EnumValueParser, ValueHint },
    error::ErrorKind,
    Parser,
    Subcommand,
    ValueEnum,
};
use crate::utils::logger;
use crate::core::cli::{ create_project, FrameWork };

lazy_static! {
    static ref CUSTOM_HELP: String = format!(
        "{} {} {}\n\n{}\n    {}           项目名称\n\n{}\n    {}           创建一个新项目\n\n{}\n    {}      项目框架 [可选值: {}]\n    {}    版本信息\n    {}       输出帮助信息",
        style("Usage").yellow(),
        style("elza-cli create").cyan(),
        style("[Options] [Name]").blue(),
        style("Arguments:").yellow(),
        style("[Name]").cyan(),
        style("Command:").yellow(),
        style("create").cyan(),
        style("Options:").yellow(),
        style("-f, --frame").cyan(),
        get_possible_frame_values(),
        style("-V, --version").cyan(),
        style("-h, --help").cyan()
    );
}

#[derive(Parser, Debug)]
#[command(name = "elza-cli", author, version, about, args_conflicts_with_subcommands = true)]
#[command(override_help = CUSTOM_HELP.as_str())]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // 创建一个新项目
    #[command(override_help = CUSTOM_HELP.as_str(), version)] Create {
        #[arg(help = "项目名称", value_hint = ValueHint::DirPath, ignore_case = true)]
        name: Option<String>,

        #[arg(
            help = "项目框架",
            short = 'f',
            long = "frame",
            value_name = "框架",
            value_parser = EnumValueParser::<FrameWork>::new(),
            ignore_case = true
        )]
        frame_work: Option<FrameWork>,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let _cli = match Cli::try_parse_from(std::env::args()) {
        std::result::Result::Ok(cli) => cli,
        Err(e) => {
            // 检查错误类型
            handle_parse_error(e);
            return Ok(());
        }
    };

    match _cli.command {
        // 如果匹配到了字段
        Some(command) => {
            match command {
                Commands::Create { name, frame_work } => {
                    // 执行创建项目的逻辑
                    match name {
                        Some(project_name) => {
                            let rt: Runtime = Runtime::new()?;
                            rt.block_on(async {
                                create_project(project_name, frame_work).await?;
                                Ok(())
                            })?;
                        }
                        None => {
                            logger::error("Name为必填参数\n");
                            print_custom_help();
                        }
                    }
                }
            }
        }
        None => {
            print_custom_help();
        }
    }
    Ok(())
}

// 处理错误
fn handle_parse_error(e: clap::Error) {
    match e.kind() {
        ErrorKind::InvalidValue => {
            logger::error("提供的参数值无效\n");
            print_custom_help();
        }
        ErrorKind::UnknownArgument => {
            logger::error("提供的参数值未知\n");
            print_custom_help();
        }
        ErrorKind::MissingRequiredArgument => {
            logger::error("缺少必需参数\n");
            print_custom_help();
        }
        ErrorKind::DisplayHelp => {
            print_custom_help();
        }
        // 其他错误，显示默认错误信息
        _ => {
            e.print().unwrap();
        }
    }
    std::process::exit(1);
}

// 获取框架可能的值
fn get_possible_frame_values() -> String {
    let mut possible_values = Vec::new();
    for frame_work in FrameWork::value_variants() {
        possible_values.push(frame_work.to_string().to_lowercase());
    }
    let joined_strings = possible_values.join(",");
    joined_strings
}

// 输出自定义帮助信息
fn print_custom_help() {
    println!("{}", CUSTOM_HELP.as_str());
}
