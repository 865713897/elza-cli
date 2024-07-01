mod utils;
mod core;
use anyhow::{ Ok, Result };
use clap::{ builder::{ EnumValueParser, ValueHint }, CommandFactory, Parser, Subcommand };
use crate::utils::logger;
use crate::core::cli::{ create_project, FrameWork };

#[derive(Parser, Debug)]
#[command(name = "elza-cli", author, version, about, args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // 创建一个新项目
    Create {
        #[arg(help = "项目名称", value_hint = ValueHint::DirPath)]
        name: String,

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

fn main() -> Result<()> {
    let _cli = Cli::parse();
    match _cli.command {
        // 如果匹配到了字段
        Some(command) => {
            match command {
                Commands::Create { name, frame_work } => {
                    // 执行创建项目的逻辑
                    create_project(name, frame_work)?;
                }
            }
        }
        None => {
            Cli::command().print_help()?;
            std::process::exit(0); // 显示帮助信息后退出
        }
    }
    Ok(())
}
