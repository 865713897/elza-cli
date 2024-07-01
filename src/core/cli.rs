use anyhow::{ Ok, Result, Context };
use std::path::PathBuf;
use std::process::exit;
use console::Style;
use dialoguer::{ console::{ style, Term }, theme::ColorfulTheme, Select };
use clap::ValueEnum;

use crate::utils::logger;
use super::build;

#[derive(Copy, Clone, Debug)]
pub enum DependenciesMod {
    Dev,
    Prod,
}

#[derive(Copy, Clone, Debug)]
pub struct Dependency {
    pub name: &'static str,
    pub version: &'static str,
    pub mod_type: DependenciesMod,
}

pub fn create_project(project_name: String, fame_work: Option<FrameWork>) -> Result<()> {
    // 如果这个目录已经存在
    if PathBuf::from(&project_name).exists() {
        logger::error(&format!("创建失败: {:#?} 已经存在！", &project_name));
        return Ok(());
    }

    let current_version = env!("CARGO_PKG_VERSION");
    logger::info(
        &format!(
            "{}{}",
            style("elza-cli v").color256(14).bold(),
            style(current_version).color256(14).bold()
        )
    );

    logger::info("开始预设项目...");

    let frame = frame_selector(fame_work)?;
    let state = state_selector()?;
    let lang = lang_selector()?;
    let ui = ui_selector()?;
    let css = css_selector()?;

    build::start(project_name.as_str(), build::InlineConfig { ui, css, frame, lang, state })?;
    Ok(())
}

// 框架选择
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum FrameWork {
    React,
}

impl FrameWork {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            FrameWork::React =>
                vec![
                    Dependency {
                        name: "react",
                        version: "^18.2.0",
                        mod_type: DependenciesMod::Prod,
                    },
                    Dependency {
                        name: "react-dom",
                        version: "^18.2.0",
                        mod_type: DependenciesMod::Prod,
                    },
                    Dependency {
                        name: "react-router-dom",
                        version: "^6.23.1",
                        mod_type: DependenciesMod::Prod,
                    }
                ],
        }
    }
}

fn frame_selector(frame_work: Option<FrameWork>) -> Result<FrameWork> {
    match frame_work {
        Some(frame) => Ok(frame),
        None => {
            logger::pick("请选择项目框架");
            let items = vec!["react", "vue"];
            let selection = create_list(&items, 0)?;
            match selection {
                0 => Ok(FrameWork::React),
                _ => {
                    logger::error(&format!("暂不支持: {}", &items[selection]));
                    exit(1);
                }
            }
        }
    }
}

// 状态选择
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum StateManagement {
    Zustand,
    None,
}

impl StateManagement {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            StateManagement::Zustand =>
                vec![Dependency {
                    name: "zustand",
                    version: "^4.5.4",
                    mod_type: DependenciesMod::Prod,
                }],
            StateManagement::None => vec![],
        }
    }
}

fn state_selector() -> Result<StateManagement> {
    logger::pick("请选择状态管理器");
    let items = vec!["zustand", "跳过"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => Ok(StateManagement::Zustand),
        1 => Ok(StateManagement::None),
        _ => {
            logger::error("暂不支持");
            exit(1);
        }
    }
}

// 语言选择
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum CodeLanguage {
    Js,
    Ts,
}

impl CodeLanguage {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            CodeLanguage::Js => vec![],
            CodeLanguage::Ts =>
                vec![
                    Dependency {
                        name: "@babel/preset-typescript",
                        version: "^7.24.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/node",
                        version: "^20.12.12",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/react",
                        version: "^18.3.2",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/react-dom",
                        version: "^18.3.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/webpack",
                        version: "^5.28.5",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "ts-node",
                        version: "^10.9.2",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
        }
    }
}

fn lang_selector() -> Result<CodeLanguage> {
    logger::pick("请选择语言");
    let items = vec!["typescript", "javascript"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => Ok(CodeLanguage::Ts),
        1 => Ok(CodeLanguage::Js),
        _ => {
            logger::error("暂不支持");
            exit(1);
        }
    }
}

// UI框架选择
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum UIDesign {
    Antd,
    ElementPlus,
}

impl UIDesign {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            UIDesign::Antd =>
                vec![Dependency {
                    name: "antd",
                    version: "^5.3.0",
                    mod_type: DependenciesMod::Prod,
                }],
            UIDesign::ElementPlus =>
                vec![Dependency {
                    name: "element-plus",
                    version: "^2.7.5",
                    mod_type: DependenciesMod::Prod,
                }],
        }
    }
}

pub fn ui_selector() -> Result<UIDesign> {
    logger::pick("请选择UI库");
    let items = vec!["antd", "element-plus"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => Ok(UIDesign::Antd),
        1 => Ok(UIDesign::ElementPlus),
        _ => {
            logger::error("暂不支持");
            exit(1)
        }
    }
}

// css预处理
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
                    Dependency {
                        name: "sass-loader",
                        version: "^14.2.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency { name: "sass", version: "^1.77.6", mod_type: DependenciesMod::Dev }
                ],
            CssPreset::Less =>
                vec![
                    Dependency { name: "less", version: "^4.1.3", mod_type: DependenciesMod::Dev },
                    Dependency {
                        name: "less-loader",
                        version: "^11.1.0",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
        }
    }
}

pub fn css_selector() -> Result<CssPreset> {
    logger::pick("请选择CSS预处理器");
    let items = vec!["sass", "less", "styled-components"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => Ok(CssPreset::Sass),
        1 => Ok(CssPreset::Less),
        _ => {
            logger::error(&format!("暂不支持: {}", &items[selection]));
            exit(1);
        }
    }
}

pub fn create_list(items: &[&str], default: usize) -> Result<usize> {
    Select::with_theme(
        &(ColorfulTheme {
            active_item_prefix: style("❯".to_string()).for_stderr().color256(69),
            active_item_style: Style::new().for_stderr().color256(69),
            ..ColorfulTheme::default()
        })
    )
        .items(&items)
        .default(default)
        .interact_on_opt(&Term::stderr())
        .context("选择项失败")?
        .ok_or_else(|| anyhow::anyhow!("未选择任何项"))
}
