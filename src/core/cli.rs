use anyhow::{ Ok, Result };
use std::{ process::exit, path::PathBuf, fmt };
use tokio::{ spawn, join };
use console::style;
use clap::ValueEnum;

use crate::utils::{ logger, utils };
use super::build;
use super::select::create_list;
use super::pack;
use super::comprehensive;

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

pub async fn create_project(project_name: String, frame_work: Option<FrameWork>) -> Result<()> {
    // 如果这个目录已经存在
    if PathBuf::from(&project_name).exists() {
        logger::error(&format!("创建失败: {:#?} 已经存在！", &project_name));
        return Ok(());
    }
    let current_version = env!("CARGO_PKG_VERSION");

    logger::info(
        &format!("{}{}", style("elza-cli v").green().bold(), style(current_version).green().bold())
    );

    logger::info("开始预设项目...");

    // 获取最新版本
    let latest_version_future = spawn(async move {
        utils::get_latest_version(env!("CARGO_PKG_NAME")).await
    });

    let config_future = spawn(async move {
        let frame = frame_selector(frame_work)?;
        let build_tool = pack::build_tool_selector()?;
        let lang = lang_selector()?;
        // 获取综合类型
        let comprehensive_type = get_comprehensive_type(frame, build_tool, lang);
        let loader = match build_tool {
            pack::BuildTool::Webpack => js_loader_selector()?,
            pack::BuildTool::Vite => JsLoader::None,
        };
        let ui = ui_selector()?;
        let state = state_selector()?;
        let css = css_selector()?;
        let build_tool_with_css = get_build_tool_with_css(build_tool, css);
        build
            ::start(project_name.as_str(), build::InlineConfig {
                frame,
                build_tool,
                lang,
                comprehensive_type,
                loader,
                ui,
                state,
                css,
                build_tool_with_css,
            })
            .map_err(|e| anyhow::anyhow!(e))
    });

    let (latest_version_result, _) = join!(latest_version_future, config_future);
    let latest_version = latest_version_result??;
    utils::compare_versions(current_version, &latest_version);
    logger::ready("项目初始化完成");
    Ok(())
}

fn get_comprehensive_type(
    frame: FrameWork,
    build_tool: pack::BuildTool,
    lang: CodeLanguage
) -> comprehensive::ComprehensiveType {
    match (frame, build_tool, lang) {
        (FrameWork::React, pack::BuildTool::Webpack, CodeLanguage::Js) =>
            comprehensive::ComprehensiveType::WebpackReactJs,
        (FrameWork::React, pack::BuildTool::Webpack, CodeLanguage::Ts) =>
            comprehensive::ComprehensiveType::WebpackReactTs,
        (FrameWork::React, pack::BuildTool::Vite, CodeLanguage::Js) =>
            comprehensive::ComprehensiveType::ViteReactJs,
        (FrameWork::React, pack::BuildTool::Vite, CodeLanguage::Ts) =>
            comprehensive::ComprehensiveType::ViteReactTs,
    }
}

fn get_build_tool_with_css(build_tool: pack::BuildTool, css: CssPreset) -> BuildToolWithCssPreset {
    match (build_tool, css) {
        (pack::BuildTool::Webpack, CssPreset::Sass) => BuildToolWithCssPreset::WebpackSass,
        (pack::BuildTool::Webpack, CssPreset::Less) => BuildToolWithCssPreset::WebpackLess,
        (pack::BuildTool::Vite, CssPreset::Sass) => BuildToolWithCssPreset::ViteSass,
        (pack::BuildTool::Vite, CssPreset::Less) => BuildToolWithCssPreset::ViteLess,
    }
}

// 框架选择
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum FrameWork {
    React,
}

impl fmt::Display for FrameWork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameWork::React => write!(f, "React"),
        }
    }
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
                vec![Dependency {
                    name: "typescript",
                    version: "^5.5.2",
                    mod_type: DependenciesMod::Dev,
                }],
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

// loader
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum JsLoader {
    Babel,
    Swc,
    None,
}

impl JsLoader {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            JsLoader::Babel =>
                vec![
                    Dependency {
                        name: "@babel/core",
                        version: "^7.24.5",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@babel/plugin-transform-runtime",
                        version: "^7.24.7",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@babel/preset-env",
                        version: "^7.24.5",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@babel/preset-react",
                        version: "^7.24.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@babel/runtime",
                        version: "^7.24.7",
                        mod_type: DependenciesMod::Prod,
                    },
                    Dependency {
                        name: "babel-loader",
                        version: "^9.1.3",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "babel-plugin-auto-css-module",
                        version: "1.0.0",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            JsLoader::Swc =>
                vec![
                    Dependency {
                        name: "@swc/core",
                        version: "^1.6.6",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "swc-loader",
                        version: "^0.2.6",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "swc-plugin-auto-css-module",
                        version: "0.0.9",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            JsLoader::None => vec![],
        }
    }
}

fn js_loader_selector() -> Result<JsLoader> {
    logger::pick("请选择loader");
    let items = vec!["babel-loader", "swc-loader"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => Ok(JsLoader::Babel),
        1 => Ok(JsLoader::Swc),
        _ => {
            logger::error(&format!("暂不支持{}", &items[selection]));
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
                vec![Dependency {
                    name: "sass",
                    version: "^1.77.6",
                    mod_type: DependenciesMod::Dev,
                }],
            CssPreset::Less =>
                vec![Dependency {
                    name: "less",
                    version: "^4.1.3",
                    mod_type: DependenciesMod::Dev,
                }],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum BuildToolWithCssPreset {
    WebpackSass,
    WebpackLess,
    ViteSass,
    ViteLess,
}

impl BuildToolWithCssPreset {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            BuildToolWithCssPreset::WebpackSass =>
                vec![Dependency {
                    name: "sass-loader",
                    version: "^14.2.1",
                    mod_type: DependenciesMod::Dev,
                }],
            BuildToolWithCssPreset::WebpackLess =>
                vec![Dependency {
                    name: "less-loader",
                    version: "^11.1.0",
                    mod_type: DependenciesMod::Dev,
                }],
            _ => vec![],
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
