use anyhow::{ Ok, Result, Context };
use std::path::PathBuf;
use std::process::exit;
use console::Style;
use clap::ValueEnum;
use dialoguer::{ console::{ style, Term }, theme::ColorfulTheme, Select };

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
pub enum FrameWork {
    React,
    // Vue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum CodeLanguage {
    Js,
    Ts,
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
    project_name: String,
    fame_work: Option<FrameWork>,
    ui_design: Option<UIDesign>,
    css_preset: Option<CssPreset>,
    language: Option<CodeLanguage>
) -> Result<()> {
    // 如果这个目录已经存在
    if PathBuf::from(&project_name).exists() {
        logger::error(&format!("创建失败: {:#?} 已经存在！", &project_name));
        return Ok(());
    }
    logger::info("开始预设项目...");

    let frame = select_frame_work(fame_work)?;
    let lang = select_language(language)?;
    let ui = select_ui_library(ui_design)?;
    let css = select_css_preset(css_preset)?;

    build::start(project_name.as_str(), build::InlineConfig { ui, css, frame, lang })?;
    Ok(())
}

// 框架选择器
fn select_frame_work(fame_work: Option<FrameWork>) -> Result<FrameWork> {
    match fame_work {
        Some(fame) => Ok(fame),
        None => {
            // if cli_mode {
            //     logger::error("请指定框架");
            //     exit(1);
            // }
            logger::select_msg("请选择项目框架");

            let items = vec!["react", "vue"];
            let selection = select_from_list(&items, 0).context("选择项目框架失败")?;
            match selection {
                0 => Ok(FrameWork::React),
                // 1 => Ok(FrameWork::Vue),
                _ => {
                    logger::error("未指定的框架");
                    exit(1)
                }
            }
        }
    }
}

// 语言选择器
fn select_language(language: Option<CodeLanguage>) -> Result<CodeLanguage> {
    match language {
        Some(lang) => Ok(lang),
        None => {
            // if cli_mode {
            //     logger::error("请指定语言");
            //     exit(1);
            // }
            logger::select_msg("请选择语言");

            let items = vec!["typescript", "javascript"];
            let selection = select_from_list(&items, 0).context("选择语言失败")?;
            match selection {
                0 => Ok(CodeLanguage::Ts),
                1 => Ok(CodeLanguage::Js),
                _ => {
                    logger::error("未指定的语言");
                    exit(1);
                }
            }
        }
    }
}

// UI选择器
fn select_ui_library(ui_design: Option<UIDesign>) -> Result<UIDesign> {
    match ui_design {
        Some(ui) => Ok(ui),
        None => {
            // if cli_mode {
            //     logger::error("请指定UI库");
            //     exit(1);
            // }
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
fn select_css_preset(css_preset: Option<CssPreset>) -> Result<CssPreset> {
    match css_preset {
        Some(css) => Ok(css),
        None => {
            // if cli_mode {
            //     logger::error("请指定CSS预设");
            //     exit(1);
            // }
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
