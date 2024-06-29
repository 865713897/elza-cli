use clap::ValueEnum;
use anyhow::Result;
use std::process::exit;

use super::cli::{ Dependency, DependenciesMod };
use super::select_from_list::create_list;
use crate::utils::logger;

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
    logger::select_msg("请选择UI库");
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
