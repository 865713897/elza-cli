use anyhow::{ Ok as AnyhowOk, Result };
use clap::ValueEnum;
use console::style;
use std::{ fmt, path::PathBuf, process::exit };
use tokio::{ join, spawn };

use super::build;
use super::pack;
use super::select::create_list;
use crate::utils::{ logger, utils };

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

pub async fn create_project(
    project_name: String,
    template: Option<pack::PackTool>,
    frame_work: Option<FrameWork>
) -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    logger::info(
        &format!("{}{}", style("elza-cli v").green().bold(), style(current_version).green().bold())
    );
    // 如果这个目录已经存在
    if PathBuf::from(&project_name).exists() {
        logger::error(&format!("创建失败: {:#?} 已经存在！", &project_name));
        return AnyhowOk(());
    }
    logger::info("开始预设项目...");

    // 获取最新版本
    let latest_version_future = spawn(async move {
        utils::get_latest_version(env!("CARGO_PKG_NAME")).await
    });

    let config_future = spawn(async move {
        let frame = frame_selector(frame_work)?;
        let pack_tool = pack::pack_tool_selector(template)?;
        let lang = match pack_tool {
            pack::PackTool::Farm => CodeLanguage::Ts,
            _ => lang_selector()?,
        };
        let loader = match pack_tool {
            pack::PackTool::Webpack => js_loader_selector()?,
            _ => JsLoader::None,
        };
        let css = match pack_tool {
            pack::PackTool::Elza => CssPreset::None,
            _ => css_selector()?,
        };
        build
            ::start(project_name.as_str(), build::InlineConfig {
                frame,
                pack_tool,
                lang,
                loader,
                css,
            })
            .map_err(|e| anyhow::anyhow!(e))
    });

    let (_, latest_version_result) = join!(config_future, latest_version_future);
    let latest_version = match latest_version_result {
        Ok(inner_result) =>
            match inner_result {
                Ok(version) => version, // 内层 Result 是 Ok，获取 version
                Err(e) => {
                    logger::warning(&format!("获取最新版本失败: {}", e));
                    "".to_string() // 内层 Result 是 Err，返回默认值
                }
            }
        Err(_) => {
            logger::warning(&format!("获取最新版本失败"));
            "".to_string() // 如果任务本身失败，返回默认值
        }
    };
    utils::compare_versions(current_version, &latest_version);
    logger::ready("项目初始化完成");
    AnyhowOk(())
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
                        name: "axios",
                        version: "^1.7.9",
                        mod_type: DependenciesMod::Prod,
                    },
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
        Some(frame) => AnyhowOk(frame),
        None => {
            logger::pick("请选择项目框架");
            let items = vec!["react", "vue"];
            let selection = create_list(&items, 0)?;
            match selection {
                0 => AnyhowOk(FrameWork::React),
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
    pub fn get_dependencies(
        &self,
        build_tool: pack::PackTool,
        frame: FrameWork
    ) -> Vec<Dependency> {
        let common = vec![
            Dependency {
                name: "@types/react",
                version: "^18.3.2",
                mod_type: DependenciesMod::Dev,
            },
            Dependency {
                name: "@types/react-dom",
                version: "^18.3.0",
                mod_type: DependenciesMod::Dev,
            }
        ];
        let mut dependencies = match (self, build_tool, frame) {
            (CodeLanguage::Js, pack::PackTool::Webpack, FrameWork::React) => vec![],
            (CodeLanguage::Ts, pack::PackTool::Webpack, FrameWork::React) =>
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
                        name: "@types/webpack",
                        version: "^5.28.5",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "ts-node",
                        version: "^10.9.2",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "typescript",
                        version: "^5.5.2",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            (CodeLanguage::Js, pack::PackTool::Vite, FrameWork::React) =>
                vec![Dependency {
                    name: "@vitejs/plugin-react-swc",
                    version: "^3.5.0",
                    mod_type: DependenciesMod::Dev,
                }],
            (CodeLanguage::Ts, pack::PackTool::Vite, FrameWork::React) =>
                vec![
                    Dependency {
                        name: "@vitejs/plugin-react-swc",
                        version: "^3.5.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "typescript",
                        version: "^5.5.2",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            (CodeLanguage::Js, pack::PackTool::Rsbuild, FrameWork::React) =>
                vec![Dependency {
                    name: "@rsbuild/plugin-react",
                    version: "^1.0.7",
                    mod_type: DependenciesMod::Dev,
                }],
            (CodeLanguage::Ts, pack::PackTool::Rsbuild, FrameWork::React) =>
                vec![
                    Dependency {
                        name: "@rsbuild/plugin-react",
                        version: "^1.0.7",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "typescript",
                        version: "^5.5.2",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            (_, pack::PackTool::Farm, FrameWork::React) =>
                vec![Dependency {
                    name: "@farmfe/plugin-react",
                    version: "^1.2.0",
                    mod_type: DependenciesMod::Dev,
                }],
            (_, pack::PackTool::Elza, FrameWork::React) => vec![],
        };

        if self == &CodeLanguage::Ts {
            dependencies.extend(common);
        }
        dependencies
    }
}

fn lang_selector() -> Result<CodeLanguage> {
    logger::pick("请选择语言");
    let items = vec!["typescript", "javascript"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => AnyhowOk(CodeLanguage::Ts),
        1 => AnyhowOk(CodeLanguage::Js),
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
                        version: "1.6.6",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "swc-loader",
                        version: "0.2.6",
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
        0 => AnyhowOk(JsLoader::Babel),
        1 => AnyhowOk(JsLoader::Swc),
        _ => {
            logger::error(&format!("暂不支持{}", &items[selection]));
            exit(1);
        }
    }
}

// css预处理
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum CssPreset {
    Sass,
    Less,
    None,
}

impl CssPreset {
    pub fn get_dependencies(&self, build_tool: pack::PackTool) -> Vec<Dependency> {
        match (self, build_tool) {
            (CssPreset::Sass, pack::PackTool::Webpack) =>
                vec![
                    Dependency {
                        name: "sass",
                        version: "^1.77.6",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "sass-loader",
                        version: "^14.2.1",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            (CssPreset::Less, pack::PackTool::Webpack) =>
                vec![
                    Dependency {
                        name: "less",
                        version: "^4.1.3",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "less-loader",
                        version: "^11.1.0",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            (CssPreset::Sass, pack::PackTool::Vite) =>
                vec![Dependency {
                    name: "sass",
                    version: "^1.77.6",
                    mod_type: DependenciesMod::Dev,
                }],
            (CssPreset::Less, pack::PackTool::Vite) =>
                vec![Dependency {
                    name: "less",
                    version: "^4.1.3",
                    mod_type: DependenciesMod::Dev,
                }],
            (CssPreset::Sass, pack::PackTool::Rsbuild) =>
                vec![Dependency {
                    name: "@rsbuild/plugin-sass",
                    version: "^1.1.1",
                    mod_type: DependenciesMod::Dev,
                }],
            (CssPreset::Less, pack::PackTool::Rsbuild) =>
                vec![Dependency {
                    name: "@rsbuild/plugin-less",
                    version: "^1.1.0",
                    mod_type: DependenciesMod::Dev,
                }],
            (CssPreset::Sass, pack::PackTool::Farm) =>
                vec![Dependency {
                    name: "@farmfe/plugin-sass",
                    version: "^1.1.0",
                    mod_type: DependenciesMod::Dev,
                }],
            (CssPreset::Less, pack::PackTool::Farm) =>
                vec![Dependency {
                    name: "@farmfe/js-plugin-less",
                    version: "^1.11.0",
                    mod_type: DependenciesMod::Dev,
                }],
            (_, _) => vec![],
        }
    }
}

pub fn css_selector() -> Result<CssPreset> {
    logger::pick("请选择CSS预处理器");
    let items = vec!["sass", "less", "styled-components"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => AnyhowOk(CssPreset::Sass),
        1 => AnyhowOk(CssPreset::Less),
        _ => {
            logger::error(&format!("暂不支持: {}", &items[selection]));
            exit(1);
        }
    }
}
