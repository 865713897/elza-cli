use std::path::PathBuf;
use std::fs;
use std::process::{ Command, Stdio, exit };
use anyhow::Result;
use rust_embed::RustEmbed;

use crate::utils::logger;
use crate::utils::error::{ handle_option, handle_result };

use super::package_json::{ PackageJson, PackageBasicInfo };
use super::pack::BuildTool;
use super::comprehensive::ComprehensiveType;
use super::cli::{
    Dependency,
    FrameWork,
    CodeLanguage,
    JsLoader,
    UIDesign,
    StateManagement,
    CssPreset,
    BuildToolWithCssPreset
};

#[derive(Clone, Copy)]
pub struct InlineConfig {
    pub frame: FrameWork,
    pub build_tool: BuildTool,
    pub lang: CodeLanguage,
    pub comprehensive_type: ComprehensiveType,
    pub loader: JsLoader,
    pub ui: UIDesign,
    pub state: StateManagement,
    pub css: CssPreset,
    pub build_tool_with_css: BuildToolWithCssPreset,
}

#[derive(RustEmbed)]
#[folder = "common/"]
struct Common;

#[derive(RustEmbed)]
#[folder = "react/webpack/template-js"]
struct ReactWebpackJsTemplate;

#[derive(RustEmbed)]
#[folder = "react/webpack/template-ts"]
struct ReactWebpackTsTemplate;

#[derive(RustEmbed)]
#[folder = "react/vite/template-js"]
struct ReactViteJsTemplate;

#[derive(RustEmbed)]
#[folder = "react/vite/template-ts"]
struct ReactViteTsTemplate;

#[derive(Clone)]
enum TemplateType {
    ReactWebpackJs,
    ReactWebpackTs,
    ReactViteJs,
    ReactViteTs,
    Common,
}

// 项目初始化
pub fn start(project_name: &str, config: InlineConfig) -> Result<()> {
    // 初始化项目路径
    let project_dir = PathBuf::from(project_name);
    create_project_dir(&project_dir)?;

    let template_type = get_template_type(config.comprehensive_type);

    // 复制特定模板文件
    copy_template_files(&project_dir, template_type, config.clone())?;

    // 复制公共文件
    copy_template_files(&project_dir, TemplateType::Common, config.clone())?;

    logger::info("文件创建完成");
    // 根据ui选项更新webpack rules
    if config.build_tool == BuildTool::Webpack {
        update_webpack_rules(&project_dir, config)?;
    }
    let mut pj = PackageJson::new(&project_dir)?;
    // 更新package.json基本信息
    pj.update_basic(PackageBasicInfo {
        name: project_name.to_string(),
        comprehensive_type: config.comprehensive_type,
    })?;
    // 更新package.json依赖项
    let deps = vec![
        config.frame.get_dependencies(),
        config.build_tool.get_dependencies(),
        config.lang.get_dependencies(),
        config.comprehensive_type.get_dependencies(),
        config.loader.get_dependencies(),
        config.state.get_dependencies(),
        config.ui.get_dependencies(),
        config.css.get_dependencies(),
        config.build_tool_with_css.get_dependencies(),
    ];
    let flatten_deps: Vec<Dependency> = deps.into_iter().flatten().collect();
    for dep in flatten_deps {
        pj.update_dependencies(&dep.name, &dep.version, dep.mod_type)?;
    }
    // 对依赖项排序
    pj.sort();
    // 写入
    pj.write()?;
    logger::info("预设依赖项添加完成");
    git_init(&project_dir)?;
    if config.build_tool == BuildTool::Webpack {
        logger::full_info(
            "[你知道吗？] webpack模板内置自动生成路由插件，依赖安装完成启动项目即可生成路由文件，详见 https://github.com/865713897/auto-route-plugin#readme"
        );
    }
    logger::ready("项目初始化完成");
    Ok(())
}

// 获取模板类型
fn get_template_type(comprehensive_type: ComprehensiveType) -> TemplateType {
    match comprehensive_type {
        ComprehensiveType::WebpackReactJs => TemplateType::ReactWebpackJs,
        ComprehensiveType::WebpackReactTs => TemplateType::ReactWebpackTs,
        ComprehensiveType::ViteReactJs => TemplateType::ReactViteJs,
        ComprehensiveType::ViteReactTs => TemplateType::ReactViteTs,
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
fn copy_template_files(
    project_dir: &PathBuf,
    template_type: TemplateType,
    config: InlineConfig
) -> Result<()> {
    let template_iter: Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>> = match
        template_type
    {
        TemplateType::ReactWebpackJs => Box::new(ReactWebpackJsTemplate::iter()),
        TemplateType::ReactWebpackTs => Box::new(ReactWebpackTsTemplate::iter()),
        TemplateType::ReactViteJs => Box::new(ReactViteJsTemplate::iter()),
        TemplateType::ReactViteTs => Box::new(ReactViteTsTemplate::iter()),
        TemplateType::Common => Box::new(Common::iter()),
    };
    for filename in template_iter {
        if should_skip_file(&filename, config.loader) {
            continue;
        }
        copy_template_file(project_dir, filename.as_ref(), template_type.clone())?;
    }
    Ok(())
}

// 判断是否应该忽略某个文件
fn should_skip_file(filename: &str, loader: JsLoader) -> bool {
    (loader == JsLoader::Babel && filename.contains(".swcrc")) ||
        (loader == JsLoader::Swc && filename.contains("babel.config.js"))
}

// 遍历template内部文件，并写入
fn copy_template_file(
    project_dir: &PathBuf,
    filename: &str,
    template_type: TemplateType
) -> Result<()> {
    let file_content = handle_option(
        match template_type {
            TemplateType::ReactWebpackJs => ReactWebpackJsTemplate::get(filename),
            TemplateType::ReactWebpackTs => ReactWebpackTsTemplate::get(filename),
            TemplateType::ReactViteJs => ReactViteJsTemplate::get(filename),
            TemplateType::ReactViteTs => ReactViteTsTemplate::get(filename),
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
    let mut path = project_dir.clone();
    match config.lang {
        CodeLanguage::Ts => path.push("scripts/webpack.common.ts"),
        CodeLanguage::Js => path.push("scripts/webpack.common.js"),
    }

    let mut content = fs::read_to_string(&path).unwrap();

    if config.css == CssPreset::Less {
        content = content.replace("sass-loader", "less-loader").replace("scss", "less");
    }

    if config.loader == JsLoader::Swc {
        content = content.replace("babel-loader", "swc-loader");
    }

    fs::write(&path, content).unwrap();

    Ok(())
}
