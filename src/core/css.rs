use clap::ValueEnum;
use anyhow::Result;
use std::process::exit;

use super::cli::{ Dependency, DependenciesMod };
use super::select_from_list::create_list;
use crate::utils::logger;

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
                    Dependency { name: "sass-loader", version: "^14.2.1", mod_type: DependenciesMod::Dev },
                    Dependency { name: "sass", version: "^1.77.6", mod_type: DependenciesMod::Dev }
                ],
            CssPreset::Less =>
                vec![
                    Dependency { name: "less", version: "^4.1.3", mod_type: DependenciesMod::Dev},
                    Dependency { name: "less-loader", version: "^11.1.0", mod_type: DependenciesMod::Dev }
                ],
        }
    }
}

pub fn css_selector() -> Result<CssPreset> {
    logger::select_msg("请选择CSS预处理器");
    let items = vec!["sass", "less", "styled-components"];
    let selection = create_list(&items, 0)?;
    match selection {
        0 => Ok(CssPreset::Sass),
        1 => Ok(CssPreset::Less),
        _ => {
            logger::error("暂不支持");
            exit(1);
        }
    }
}
