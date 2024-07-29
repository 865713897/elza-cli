use anyhow::{ Ok, Result };
use std::process::exit;
use std::fmt;
use clap::ValueEnum;

use crate::utils::logger;
use super::select::create_list;
use super::cli::{ Dependency, DependenciesMod };

// 打包工具
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum PackTool {
    Webpack,
    Vite,
    Rspack,
    Farm,
}

impl fmt::Display for PackTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackTool::Webpack => write!(f, "webpack"),
            PackTool::Vite => write!(f, "vite"),
            PackTool::Rspack => write!(f, "rspack"),
            PackTool::Farm => write!(f, "farm"),
        }
    }
}

impl PackTool {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            PackTool::Webpack =>
                vec![
                    Dependency {
                        name: "webpack-plugin-auto-routes",
                        version: "^1.0.3",
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
                        name: "cross-env",
                        version: "^7.0.3",
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
            PackTool::Vite =>
                vec![Dependency {
                    name: "vite",
                    version: "^5.3.1",
                    mod_type: DependenciesMod::Dev,
                }],
            PackTool::Rspack =>
                vec![
                    Dependency {
                        name: "@rsbuild/core",
                        version: "1.0.1-beta.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "webpack-plugin-auto-routes",
                        version: "1.0.3",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            PackTool::Farm =>
                vec![
                    Dependency {
                        name: "@farmfe/cli",
                        version: "^1.0.2",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@farmfe/core",
                        version: "^1.3.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@farmfe/plugin-react",
                        version: "^1.2.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "core-js",
                        version: "^3.36.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "react-refresh",
                        version: "^0.14.0",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
        }
    }
}

pub fn pack_tool_selector(template: Option<PackTool>) -> Result<PackTool> {
    match template {
        Some(t) => Ok(t),
        None => {
            logger::pick("请选择打包工具");
            let items = vec!["webpack", "vite", "rspack", "farm"];
            let selection = create_list(&items, 0)?;
            match selection {
                0 => Ok(PackTool::Webpack),
                1 => Ok(PackTool::Vite),
                2 => Ok(PackTool::Rspack),
                3 => Ok(PackTool::Farm),
                _ => {
                    logger::error(&format!("暂不支持: {}", &items[selection]));
                    exit(1);
                }
            }
        }
    }
}
