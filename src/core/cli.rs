use anyhow::{ Ok, Result };
use std::{ process::exit, path::PathBuf, fmt };
use tokio::{ spawn, join };
use console::style;
use clap::ValueEnum;

use crate::utils::{ logger, utils };
use super::build;
use super::select::create_list;
use super::pack;

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
        return Ok(());
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
        // let ui = ui_selector()?;
        // let state = state_selector()?;
        let css = css_selector()?;
        build
            ::start(project_name.as_str(), build::InlineConfig {
                frame,
                pack_tool,
                lang,
                loader,
                // ui,
                // state,
                css,
            })
            .map_err(|e| anyhow::anyhow!(e))
    });

    let (latest_version_result, _) = join!(latest_version_future, config_future);
    let latest_version = latest_version_result??;
    utils::compare_versions(current_version, &latest_version);
    logger::ready("项目初始化完成");
    Ok(())
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
                    name: "@vitejs/plugin-react",
                    version: "^4.3.1",
                    mod_type: DependenciesMod::Dev,
                }],
            (CodeLanguage::Ts, pack::PackTool::Vite, FrameWork::React) =>
                vec![
                    Dependency {
                        name: "@vitejs/plugin-react",
                        version: "^4.3.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "typescript",
                        version: "^5.5.2",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            (CodeLanguage::Js, pack::PackTool::Rspack, FrameWork::React) =>
                vec![Dependency {
                    name: "@rsbuild/plugin-react",
                    version: "1.0.1-beta.1",
                    mod_type: DependenciesMod::Dev,
                }],
            (CodeLanguage::Ts, pack::PackTool::Rspack, FrameWork::React) =>
                vec![
                    Dependency {
                        name: "@rsbuild/plugin-react",
                        version: "1.0.1-beta.1",
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
        0 => Ok(JsLoader::Babel),
        1 => Ok(JsLoader::Swc),
        _ => {
            logger::error(&format!("暂不支持{}", &items[selection]));
            exit(1);
        }
    }
}

// // UI框架选择
// #[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
// pub enum UIDesign {
//     Antd,
//     ElementPlus,
//     None,
// }

// impl UIDesign {
//     pub fn get_dependencies(&self) -> Vec<Dependency> {
//         match self {
//             UIDesign::Antd =>
//                 vec![Dependency {
//                     name: "antd",
//                     version: "^5.3.0",
//                     mod_type: DependenciesMod::Prod,
//                 }],
//             UIDesign::ElementPlus =>
//                 vec![Dependency {
//                     name: "element-plus",
//                     version: "^2.7.5",
//                     mod_type: DependenciesMod::Prod,
//                 }],
//             UIDesign::None => vec![],
//         }
//     }
// }

// pub fn ui_selector() -> Result<UIDesign> {
//     logger::pick("请选择UI库");
//     let items = vec!["antd", "element-plus", "跳过"];
//     let selection = create_list(&items, 0)?;
//     match selection {
//         0 => Ok(UIDesign::Antd),
//         1 => Ok(UIDesign::ElementPlus),
//         2 => Ok(UIDesign::None),
//         _ => {
//             logger::error("暂不支持");
//             exit(1)
//         }
//     }
// }

// // 状态选择
// #[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
// pub enum StateManagement {
//     Zustand,
//     None,
// }

// impl StateManagement {
//     pub fn get_dependencies(&self) -> Vec<Dependency> {
//         match self {
//             StateManagement::Zustand =>
//                 vec![Dependency {
//                     name: "zustand",
//                     version: "^4.5.4",
//                     mod_type: DependenciesMod::Prod,
//                 }],
//             StateManagement::None => vec![],
//         }
//     }
// }

// fn state_selector() -> Result<StateManagement> {
//     logger::pick("请选择状态管理器");
//     let items = vec!["zustand", "跳过"];
//     let selection = create_list(&items, 0)?;
//     match selection {
//         0 => Ok(StateManagement::Zustand),
//         1 => Ok(StateManagement::None),
//         _ => {
//             logger::error("暂不支持");
//             exit(1);
//         }
//     }
// }

// css预处理
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum CssPreset {
    Sass,
    Less,
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
            (CssPreset::Sass, pack::PackTool::Rspack) =>
                vec![Dependency {
                    name: "@rsbuild/plugin-sass",
                    version: "1.0.1-beta.1",
                    mod_type: DependenciesMod::Dev,
                }],
            (CssPreset::Less, pack::PackTool::Rspack) =>
                vec![Dependency {
                    name: "@rsbuild/plugin-less",
                    version: "1.0.1-beta.1",
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
                    version: "^1.9.0",
                    mod_type: DependenciesMod::Dev,
                }],
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
