use clap::ValueEnum;
use anyhow::Result;
use std::process::exit;

use super::cli::{ Dependency, DependenciesMod };
use super::select_from_list::create_list;
use crate::utils::logger;

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

pub fn frame_selector(frame_work: Option<FrameWork>) -> Result<FrameWork> {
    match frame_work {
        Some(frame) => Ok(frame),
        None => {
            logger::info("请选择项目框架");
            let items = vec!["React", "Vue"];
            let selection = create_list(&items, 0)?;
            match selection {
                0 => Ok(FrameWork::React),
                _ => {
                    logger::error("暂不支持");
                    exit(1);
                }
            }
        }
    }
}
