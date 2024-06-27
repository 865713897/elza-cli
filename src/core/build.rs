use std::path::PathBuf;
use std::fs;
use std::process::{ Command, Stdio, exit };
use anyhow::Result;
use rust_embed::RustEmbed;

use crate::core::cli::{
    FrameWork,
    UIDesign,
    CssPreset,
    Dependency,
    DependenciesMod,
    CodeLanguage,
    StateManagement,
};
use crate::utils::logger;
use crate::utils::error::{ handle_option, handle_result };
use crate::core::package_json::{ update_pkg_basic, update_pkg_dependencies };

pub struct InlineConfig {
    pub frame: FrameWork,
    pub state: StateManagement,
    pub ui: UIDesign,
    pub css: CssPreset,
    pub lang: CodeLanguage,
}

#[derive(RustEmbed)]
#[folder = "common/"]
struct Common;

#[derive(RustEmbed)]
#[folder = "react-template/"]
struct ReactTemplate;

#[derive(RustEmbed)]
#[folder = "react-template-ts/"]
struct ReactTsTemplate;

#[derive(Clone)]
enum TemplateType {
    ReactWebpack,
    ReactTsWebpack,
    Common,
}

// 项目初始化
pub fn start(project_name: &str, config: InlineConfig) -> Result<()> {
    // 初始化项目路径
    let project_dir = PathBuf::from(project_name);
    create_project_dir(&project_dir)?;

    let template_type = get_template_type(config.frame, config.lang);

    // 复制特定模板文件
    copy_template_files(&project_dir, template_type)?;

    // 复制公共文件
    copy_template_files(&project_dir, TemplateType::Common)?;

    logger::info("文件创建完成");
    // 更新package.json基本信息
    update_pkg_basic(&project_dir, project_name.to_owned())?;

    // 更新package.json依赖项
    add_dependencies(&project_dir, config.ui.get_dependencies(), DependenciesMod::Prod)?;
    add_dependencies(&project_dir, config.state.get_dependencies(), DependenciesMod::Prod)?;
    add_dependencies(&project_dir, config.css.get_dependencies(), DependenciesMod::Dev)?;

    // 根据ui选项更新webpack rules
    handle_result(update_webpack_rules(&project_dir, config), "更新webpack rules失败");

    logger::info("预设依赖项添加完成");
    git_init(&project_dir)?;
    logger::info("webpack模板内置自动生成路由插件，依赖安装完成直接启动即可");
    logger::link_info("https://github.com/865713897/auto-route-plugin#readme");
    logger::ready("项目初始化完成");
    Ok(())
}

// 获取模板类型
fn get_template_type(frame: FrameWork, lang: CodeLanguage) -> TemplateType {
    match (frame, lang) {
        (FrameWork::React, CodeLanguage::Js) => TemplateType::ReactWebpack,
        (FrameWork::React, CodeLanguage::Ts) => TemplateType::ReactTsWebpack,
    }
}

// 创建项目目录
fn create_project_dir(project_dir: &PathBuf) -> Result<()> {
    logger::event("开始创建项目目录");
    handle_result(fs::create_dir(project_dir), "创建项目目录失败");
    logger::info("创建项目目录成功");
    Ok(())
}

// 复制模板文件
fn copy_template_files(project_dir: &PathBuf, template_type: TemplateType) -> Result<()> {
    let template_iter: Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>> = match
        template_type
    {
        TemplateType::ReactWebpack => Box::new(ReactTemplate::iter()),
        TemplateType::ReactTsWebpack => Box::new(ReactTsTemplate::iter()),
        TemplateType::Common => Box::new(Common::iter()),
    };
    for filename in template_iter {
        copy_template_file(project_dir, filename.as_ref(), template_type.clone())?;
    }
    Ok(())
}

// 遍历template内部文件，并写入
fn copy_template_file(
    project_dir: &PathBuf,
    filename: &str,
    template_type: TemplateType
) -> Result<()> {
    let file_content = handle_option(
        match template_type {
            TemplateType::ReactWebpack => ReactTemplate::get(filename),
            TemplateType::ReactTsWebpack => ReactTsTemplate::get(filename),
            TemplateType::Common => Common::get(filename),
        },
        &format!("获取模板文件内容失败: {}", filename)
    );

    let file_path = project_dir.join(filename);
    let directory_path = handle_option(file_path.parent(), "获取文件夹路径失败");

    logger::event(&format!("开始创建文件: {}", filename));
    handle_result(
        fs::create_dir_all(directory_path),
        &format!("创建目录失败: {:?}", directory_path)
    );
    handle_result(
        fs::write(&file_path, file_content.data),
        &format!("写入文件失败: {:?}", file_path)
    );
    Ok(())
}

// 添加依赖项目
fn add_dependencies(
    project_dir: &PathBuf,
    dependencies: Vec<Dependency>,
    dep_mod: DependenciesMod
) -> Result<()> {
    for dependency in dependencies {
        handle_result(
            update_pkg_dependencies(project_dir, dependency.name, dependency.version, dep_mod),
            &format!("添加依赖项失败: {} {}", dependency.name, dependency.version)
        );
    }
    Ok(())
}

// 初始化git仓库
fn git_init(project_dir: &PathBuf) -> Result<()> {
    logger::event("git 初始化");
    run_git_command(project_dir, &["init"], "git 初始化失败")?;
    logger::info("git 初始化完成");
    Ok(())
}

// 执行git命令
fn run_git_command(project_dir: &PathBuf, args: &[&str], error_msg: &str) -> Result<()> {
    let status = Command::new("git")
        .current_dir(project_dir)
        .args(args)
        .stdout(Stdio::null())
        .status()
        .expect(error_msg);
    if !status.success() {
        logger::error(error_msg);
        exit(1);
    }
    Ok(())
}

// 更新webpack rules
fn update_webpack_rules(project_dir: &PathBuf, config: InlineConfig) -> Result<()> {
    let InlineConfig { ui: _, lang, frame: _, css, state: _ } = config;

    let mut path = project_dir.clone();
    match lang {
        CodeLanguage::Ts => path.push("scripts/webpack.common.ts"),
        CodeLanguage::Js => path.push("scripts/webpack.common.js"),
    }

    let mut content = fs::read_to_string(&path).unwrap();

    if css == CssPreset::Less {
        content = content.replace("sass-loader", "less-loader").replace("scss", "less");
        fs::write(&path, content).unwrap();
    }

    Ok(())
}
