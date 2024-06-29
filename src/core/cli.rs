use anyhow::{ Ok, Result };
use std::path::PathBuf;
use dialoguer::console::style;

use crate::utils::logger;
use super::build;
use super::frame::{ frame_selector, FrameWork };
use super::state::state_selector;
use super::lang::lang_selector;
use super::ui::ui_selector;
use super::css::css_selector;

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

pub fn create_project(
    project_name: String,
    fame_work: Option<FrameWork>,
) -> Result<()> {
    // 如果这个目录已经存在
    if PathBuf::from(&project_name).exists() {
        logger::error(&format!("创建失败: {:#?} 已经存在！", &project_name));
        return Ok(());
    }

    let current_version = env!("CARGO_PKG_VERSION");
    logger::info(
        &format!(
            "{}{}",
            style("elza-cli v").color256(14).bold(),
            style(current_version).color256(14).bold()
        )
    );

    logger::info("开始预设项目...");

    let frame = frame_selector(fame_work)?;
    let state = state_selector()?;
    let lang = lang_selector()?;
    let ui = ui_selector()?;
    let css = css_selector()?;

    build::start(project_name.as_str(), build::InlineConfig { ui, css, frame, lang, state })?;
    Ok(())
}

