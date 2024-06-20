use std::path::PathBuf;
use std::fs;
use std::process::{ Command, Stdio, exit };
use anyhow::{ Context, Result };
use rust_embed::RustEmbed;

use crate::core::cli::{ UIDesign, CssPreset, Dependency, DependenciesMod };
use crate::utils::logger;
use crate::core::package_json::{ update_pkg_basic, update_pkg_dependencies };

pub struct InlineConfig {
    pub ui: UIDesign,
    pub css: CssPreset,
}

#[derive(RustEmbed)]
#[folder = "react-template/"]
struct Template;

// 项目初始化
pub fn start(project_name: &str, config: InlineConfig) -> Result<()> {
    // 初始化项目路径
    let project_dir = PathBuf::from(project_name);
    create_project_dir(&project_dir).context("创建项目目录失败")?;

    for filename in Template::iter() {
        copy_template_file(&project_dir, filename.as_ref())?;
    }
    logger::info("文件创建完成");
    // 更新package.json
    update_pkg_basic(&project_dir, project_name.to_owned())?;

    add_dependencies(&project_dir, config.ui.get_dependencies(), DependenciesMod::Prod)?;
    add_dependencies(&project_dir, config.css.get_dependencies(), DependenciesMod::Dev)?;

    logger::info("预设依赖项添加完成");
    git_init(&project_dir)?;
    logger::ready("项目初始化完成");
    Ok(())
}

// 创建项目目录
fn create_project_dir(project_dir: &PathBuf) -> Result<()> {
    logger::event("开始创建项目目录");
    fs::create_dir(project_dir).context("创建项目目录失败")?;
    logger::info("创建项目目录成功");
    Ok(())
}

// 遍历template内部文件，并写入
fn copy_template_file(project_dir: &PathBuf, filename: &str) -> Result<()> {
    let file_content = Template::get(filename).context(
        format!("获取模板文件内容失败: {}", filename)
    )?;
    let file_path = project_dir.join(filename);
    let directory_path = file_path.parent().context("获取文件夹路径失败")?;

    logger::event(&format!("开始创建文件: {}", filename));
    fs::create_dir_all(directory_path).context(format!("创建目录失败: {:?}", directory_path))?;
    fs::write(&file_path, file_content.data).context(format!("写入文件失败: {:?}", file_path))?;
    Ok(())
}

// 添加依赖项目
fn add_dependencies(
    project_dir: &PathBuf,
    dependencies: Vec<Dependency>,
    dep_mod: DependenciesMod
) -> Result<()> {
    for dependency in dependencies {
        update_pkg_dependencies(project_dir, dependency.name, dependency.version, dep_mod).context(
            format!("添加依赖项失败: {} {}", dependency.name, dependency.version)
        )?;
    }
    Ok(())
}

// 初始化git仓库
fn git_init(project_dir: &PathBuf) -> Result<()> {
    run_git_command(project_dir, &["init"], "git 初始化失败")?;
    logger::event("git init");
    run_git_command(project_dir, &["add", "-A"], "git 暂存失败")?;
    logger::event("git add -A");
    run_git_command(project_dir, &["commit", "-m", "initial commit"], "git 提交失败")?;
    logger::event("git commit -m 'initial commit'");
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
