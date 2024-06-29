use clap::ValueEnum;
use anyhow::Result;
use std::process::exit;

use super::cli::{ Dependency, DependenciesMod };
use super::select_from_list::create_list;
use crate::utils::logger;

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

pub fn lang_selector() -> Result<CodeLanguage> {
    logger::select_msg("请选择语言");
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
