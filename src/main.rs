mod utils;
mod core;

use std::path::PathBuf;
use std::process::{ exit, Command };
use anyhow::{ Ok, Result };
use clap::{ Parser, Subcommand, ValueEnum, builder::{ ValueHint, EnumValueParser } };
use dialoguer::{ console::{ Term, style }, theme::ColorfulTheme, Select };
use console::Style;
use crate::utils::logger;
use crate::core::{ project, update_pkg };

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
            value_parser = EnumValueParser::<UIDesign>::new(),
            ignore_case = true
        )]
        ui_design: Option<UIDesign>,

        #[arg(
            help = "项目css预处理器",
            short = 's',
            long = "style",
            value_name = "css预处理器",
            value_parser = EnumValueParser::<CssPreset>::new(),
            ignore_case = true
        )]
        css_preset: Option<CssPreset>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum UIDesign {
    Antd,
    ElementPlus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum CssPreset {
    Sass,
    Less,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum PackageManger {
    Pnpm,
    Yarn,
    Npm,
    Cnpm,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    match _cli.command {
        // 如果匹配到了字段
        Some(command) => {
            match command {
                Commands::Create { cli_mode, name, ui_design, css_preset } => {
                    // 执行创建项目的逻辑
                    create_project(cli_mode, name, ui_design, css_preset)?;
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

fn create_project(
    cli_mode: bool,
    project_name: String,
    ui_design: Option<UIDesign>,
    css_preset: Option<CssPreset>
) -> Result<()> {
    // 如果这个目录已经存在
    if PathBuf::from(&project_name).exists() {
        logger::error(&format!("创建失败: {:#?} 已经存在！", &project_name));
        return Ok(());
    }
    logger::info("开始预设项目...");
    // 选择包管理器
    logger::select_msg("请选择包管理工具");
    let package_manager_items = vec!["pnpm", "yarn", "npm", "cnpm"];
    let package_manager_selection = Select::with_theme(
        &(ColorfulTheme {
            active_item_prefix: style("❯".to_string()).for_stderr().color256(33),
            active_item_style: Style::new().for_stderr().color256(33), // 设置选中项的样式
            ..ColorfulTheme::default()
        })
    )
        .items(&package_manager_items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    let pkg_mgr = match package_manager_selection {
        Some(0) => PackageManger::Pnpm,
        Some(1) => PackageManger::Yarn,
        Some(2) => PackageManger::Npm,
        Some(3) => PackageManger::Cnpm,
        _ => {
            logger::error("未指定的包管理工具");
            exit(1)
        }
    };
    // 检测用户包管理器是否有效
    match pkg_mgr {
        PackageManger::Pnpm => {
            if !Command::new("pnpm").output().is_ok() {
                logger::error("请先安装pnpm");
                exit(1);
            }
        }
        PackageManger::Yarn => {
            if !Command::new("yarn").output().is_ok() {
                logger::error("请先安装yarn");
                exit(1);
            }
        }
        PackageManger::Npm => {
            if !Command::new("npm").output().is_ok() {
                logger::error("请先安装npm");
                exit(1);
            }
        }
        PackageManger::Cnpm => {
            if !Command::new("cnpm").output().is_ok() {
                logger::error("请先安装cnpm");
                exit(1);
            }
        }
    }
    // 获取UI
    let ui: UIDesign = match ui_design {
        Some(ui_design) => ui_design,
        None => {
            if cli_mode {
                logger::error("请指定UI库");
                exit(1);
            }
            logger::select_msg("请选择UI库");
            let items = vec!["antd", "element-plus", "not-found"];
            let selection = Select::with_theme(
                &(ColorfulTheme {
                    active_item_prefix: style("❯".to_string()).for_stderr().color256(33),
                    active_item_style: Style::new().for_stderr().color256(33), // 设置选中项的样式
                    ..ColorfulTheme::default()
                })
            )
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr())?;

            match selection {
                Some(0) => UIDesign::Antd,
                Some(1) => UIDesign::ElementPlus,
                _ => {
                    logger::error("未指定的UI库");
                    exit(1);
                }
            }
        }
    };
    // 获取CSS预设
    let css: CssPreset = match css_preset {
        Some(css_preset) => css_preset,
        None => {
            if cli_mode {
                logger::error("请指定CSS预设");
                exit(1);
            }
            logger::select_msg("请选择CSS预设");
            let items = vec!["sass", "less", "not-found"];
            let selection = Select::with_theme(
                &(ColorfulTheme {
                    active_item_prefix: style("❯".to_string()).for_stderr().color256(33),
                    active_item_style: Style::new().for_stderr().color256(33), // 设置选中项的样式
                    ..ColorfulTheme::default()
                })
            )
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr())?;

            match selection {
                Some(0) => CssPreset::Sass,
                Some(1) => CssPreset::Less,
                _ => {
                    logger::error("未指定的CSS预设");
                    exit(1);
                }
            }
        }
    };
    // 创建项目
    let _ = project::init(project_name.as_str(), project::InlineConfig { ui, css, pkg_mgr });
    Ok(())
}
