mod utils;
mod core;

use anyhow::{ Ok, Result };
use clap::{ Parser, Subcommand, builder::{ ValueHint, EnumValueParser } };
use crate::utils::logger;
use crate::core::cli;

#[derive(Parser, Debug)]
#[command(name = "elza-cli", author, version, about, args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(help = "可选参数，用于指定新项目的名称", value_hint = ValueHint::DirPath)]
    name: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // 创建一个新项目
    Create {
        #[arg(short = 'c', long = "cli", help = "通过命令行参数直接生成")]
        cli_mode: bool,

        #[arg(help = "项目名称", value_hint = ValueHint::DirPath)]
        name: String,

        #[arg(
            help = "项目UI库",
            short = 'u',
            long = "ui",
            value_name = "UI库",
            value_parser = EnumValueParser::<cli::UIDesign>::new(),
            ignore_case = true
        )]
        ui_design: Option<cli::UIDesign>,

        #[arg(
            help = "项目css预处理器",
            short = 's',
            long = "style",
            value_name = "css预处理器",
            value_parser = EnumValueParser::<cli::CssPreset>::new(),
            ignore_case = true
        )]
        css_preset: Option<cli::CssPreset>,
    },
}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    match _cli.command {
        // 如果匹配到了字段
        Some(command) => {
            match command {
                Commands::Create { cli_mode, name, ui_design, css_preset } => {
                    // 执行创建项目的逻辑
                    cli::create_project(cli_mode, name, ui_design, css_preset)?;
                }
            }
        }
        None => {
            match _cli.name {
                Some(name) => {
                    // 执行其他逻辑
                    println!("其他逻辑: {}", name);
                }
                None => logger::error("请提供项目名称！"),
            }
        }
    }
    Ok(())
}
