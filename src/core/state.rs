use clap::ValueEnum;
use anyhow::Result;
use std::process::exit;

use super::cli::{ Dependency, DependenciesMod };
use super::select_from_list::create_list;
use crate::utils::logger;

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

pub fn state_selector() -> Result<StateManagement> {
    logger::select_msg("请选择状态管理器");
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
