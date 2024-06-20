use anyhow::{ Ok, Result, Context };
use std::path::PathBuf;
use std::process::exit;
use console::Style;
use clap::ValueEnum;
use dialoguer::{ console::{ Term, style }, theme::ColorfulTheme, Select };

use crate::core::build;
use crate::utils::logger;

#[derive(Copy, Clone, Debug)]
pub enum DependenciesMod {
    Dev,
    Prod,
}

pub struct Dependency {
    pub name: &'static str,
    pub version: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum UIDesign {
    Antd,
    ElementPlus,
}

impl UIDesign {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            UIDesign::Antd => vec![Dependency { name: "antd", version: "^5.3.0" }],
            UIDesign::ElementPlus => vec![Dependency { name: "element-plus", version: "^2.7.5" }],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum CssPreset {
    Sass,
    Less,
}

impl CssPreset {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            CssPreset::Sass =>
                vec![
                    Dependency { name: "sass-loader", version: "^14.2.1" },
                    Dependency { name: "sass", version: "^1.77.6" }
                ],
            CssPreset::Less =>
                vec![
                    Dependency { name: "less", version: "^4.1.3" },
                    Dependency { name: "less-loader", version: "^11.1.0" }
                ],
        }
    }
}

pub fn create_project(
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

    let ui = select_ui_library(cli_mode, ui_design)?;
    let css = select_css_preset(cli_mode, css_preset)?;

    build::start(project_name.as_str(), build::InlineConfig { ui, css })?;
    Ok(())
}

// UI选择器
fn select_ui_library(cli_mode: bool, ui_design: Option<UIDesign>) -> Result<UIDesign> {
    match ui_design {
        Some(ui) => Ok(ui),
        None => {
            if cli_mode {
                logger::error("请指定UI库");
                exit(1);
            }
            logger::select_msg("请选择UI库");

            let items = vec!["antd", "element-plus"];
            let selection = select_from_list(&items, 0).context("选择UI库失败")?;
            match selection {
                0 => Ok(UIDesign::Antd),
                1 => Ok(UIDesign::ElementPlus),
                _ => {
                    logger::error("未指定的UI库");
                    exit(1)
                }
            }
        }
    }
}

// CSS选择器
fn select_css_preset(cli_mode: bool, css_preset: Option<CssPreset>) -> Result<CssPreset> {
    match css_preset {
        Some(css) => Ok(css),
        None => {
            if cli_mode {
                logger::error("请指定CSS预设");
                exit(1);
            }
            logger::select_msg("请选择CSS预设");

            let items: Vec<&str> = vec!["sass", "less"];
            let selection = select_from_list(&items, 0).context("选择CSS预设失败")?;
            match selection {
                0 => Ok(CssPreset::Sass),
                1 => Ok(CssPreset::Less),
                _ => {
                    logger::error("未指定的CSS预设");
                    exit(1)
                }
            }
        }
    }
}

// 生成选择项
fn select_from_list(items: &[&str], default: usize) -> Result<usize> {
    Select::with_theme(
        &(ColorfulTheme {
            active_item_prefix: style("❯".to_string()).for_stderr().color256(33),
            active_item_style: Style::new().for_stderr().color256(33),
            ..ColorfulTheme::default()
        })
    )
        .items(items)
        .default(default)
        .interact_on_opt(&Term::stderr())
        .context("选择项失败")?
        .ok_or_else(|| anyhow::anyhow!("未选择任何项"))
}
