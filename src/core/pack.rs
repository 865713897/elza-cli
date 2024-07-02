use anyhow::{ Ok, Result };
use std::process::exit;
use clap::ValueEnum;

use crate::utils::logger;
use super::select::create_list;
use super::cli::{ Dependency, DependenciesMod };

// 打包工具
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum BuildTool {
    Webpack,
    // Vite,
    // Rspack,
}

impl BuildTool {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            BuildTool::Webpack =>
                vec![
                    Dependency {
                        name: "@elzajs/auto-route-plugin",
                        version: "^0.1.6",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "autoprefixer",
                        version: "^10.4.19",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "copy-webpack-plugin",
                        version: "^12.0.2",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "css-loader",
                        version: "^7.1.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "css-minimizer-webpack-plugin",
                        version: "^7.0.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "html-webpack-plugin",
                        version: "^5.6.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "terser-webpack-plugin",
                        version: "^5.3.10",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "mini-css-extract-plugin",
                        version: "^2.9.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "postcss-loader",
                        version: "^8.1.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "style-loader",
                        version: "^4.0.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "webpack",
                        version: "^5.91.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "webpack-cli",
                        version: "^5.1.4",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "webpack-dev-server",
                        version: "^5.0.4",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "webpack-merge",
                        version: "^5.10.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "webpackbar",
                        version: "^6.0.1",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            // BuildTool::Vite => vec![],
            // BuildTool::Rspack => vec![],
        }
    }
}

pub fn build_tool_selector() -> Result<BuildTool> {
    logger::pick("请选择打包工具");
    let items = vec!["webpack", "vite", "rspack"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => Ok(BuildTool::Webpack),
        // 1 => Ok(BuildTool::Vite),
        // 2 => Ok(BuildTool::Rspack),
        _ => {
            logger::error(&format!("暂不支持: {}", &items[selection]));
            exit(1);
        }
    }
}
