use std::path::PathBuf;
use std::{ fs, vec };
use std::process::{ Command, Stdio, exit };
use anyhow::{ Result, Ok };
use rust_embed::RustEmbed;

use crate::utils::logger;
use crate::utils::error::{ handle_option, handle_result };

use super::package_json::{ PackageJson, PackageBasicInfo };
use super::pack::PackTool;
use super::cli::{
    Dependency,
    FrameWork,
    CodeLanguage,
    JsLoader,
    CssPreset,
};

#[derive(Clone, Copy)]
pub struct InlineConfig {
    pub frame: FrameWork,
    pub pack_tool: PackTool,
    pub lang: CodeLanguage,
    pub loader: JsLoader,
    // pub ui: UIDesign,
    // pub state: StateManagement,
    pub css: CssPreset,
}

#[derive(RustEmbed)]
#[folder = "common/"]
struct Common;

#[derive(RustEmbed)]
#[folder = "react/webpack/template-js"]
struct WebpackReactJsTemplate;

#[derive(RustEmbed)]
#[folder = "react/webpack/template-ts"]
struct WebpackReactTsTemplate;

#[derive(RustEmbed)]
#[folder = "react/vite/template-js"]
struct ViteReactJsTemplate;

#[derive(RustEmbed)]
#[folder = "react/vite/template-ts"]
struct ViteReactTsTemplate;

#[derive(RustEmbed)]
#[folder = "react/rspack/template-js"]
struct RspackReactJsTemplate;

#[derive(RustEmbed)]
#[folder = "react/rspack/template-ts"]
struct RspackReactTsTemplate;

#[derive(RustEmbed)]
#[folder = "react/farm"]
struct FarmReactTemplate;

#[derive(Clone)]
enum TemplateType {
    WebpackReactJsDir,
    WebpackReactTsDir,
    ViteReactJsDir,
    ViteReactTsDir,
    RspackReactJsDir,
    RspackReactTsDir,
    FarmReactDir,
    CommonDir,
}

#[derive(Clone, Debug)]
pub enum ProjectType {
    WebpackReactJs,
    WebpackReactTs,
    ViteReactJs,
    ViteReactTs,
    RspackReactJs,
    RspackReactTs,
    FarmReact,
}

// 项目初始化
pub fn start(project_name: &str, config: InlineConfig) -> Result<()> {
    // 初始化项目路径
    let project_dir = PathBuf::from(project_name);
    create_project_dir(&project_dir)?;
    // 获取项目类型
    let project_type = get_project_type(config.pack_tool, config.frame, config.lang);
    let template_type = get_template_type(project_type.clone());

    // 复制特定模板文件
    copy_template_files(&project_dir, template_type, config.clone())?;

    // 复制公共文件
    copy_template_files(&project_dir, TemplateType::CommonDir, config.clone())?;

    logger::info("文件创建完成");
    match config.pack_tool {
        PackTool::Webpack => {
            update_webpack_rules(&project_dir, config)?;
        }
        PackTool::Rspack => {
            update_rsbuild_config(&project_dir, config)?;
        }
        PackTool::Vite => {}
        PackTool::Farm => {}
    }
    let mut pj = PackageJson::new(&project_dir)?;
    // 更新package.json基本信息
    pj.update_basic(PackageBasicInfo {
        name: project_name.to_string(),
        project_type,
    })?;
    // 更新package.json依赖项
    let deps = vec![
        config.frame.get_dependencies(),
        config.pack_tool.get_dependencies(),
        config.lang.get_dependencies(config.pack_tool, config.frame),
        config.loader.get_dependencies(),
        // config.state.get_dependencies(),
        // config.ui.get_dependencies(),
        config.css.get_dependencies(config.pack_tool)
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
    if config.pack_tool == PackTool::Webpack || config.pack_tool == PackTool::Rspack {
        logger::full_info(
            &format!(
                "[你知道吗？] {:?}模板内置约定式路由插件，依赖安装完成启动项目即可生成路由文件，详见 https://github.com/865713897/webpack-plugin-auto-routes",
                config.pack_tool
            )
        );
    }
    Ok(())
}

// 获取模板类型
fn get_template_type(project_type: ProjectType) -> TemplateType {
    match project_type {
        ProjectType::WebpackReactJs => TemplateType::WebpackReactJsDir,
        ProjectType::WebpackReactTs => TemplateType::WebpackReactTsDir,
        ProjectType::ViteReactJs => TemplateType::ViteReactJsDir,
        ProjectType::ViteReactTs => TemplateType::ViteReactTsDir,
        ProjectType::RspackReactJs => TemplateType::RspackReactJsDir,
        ProjectType::RspackReactTs => TemplateType::RspackReactTsDir,
        ProjectType::FarmReact => TemplateType::FarmReactDir,
    }
}

// 获取项目类型
fn get_project_type(pack_tool: PackTool, frame: FrameWork, lang: CodeLanguage) -> ProjectType {
    match (pack_tool, frame, lang) {
        (PackTool::Webpack, FrameWork::React, CodeLanguage::Js) => ProjectType::WebpackReactJs,
        (PackTool::Webpack, FrameWork::React, CodeLanguage::Ts) => ProjectType::WebpackReactTs,
        (PackTool::Vite, FrameWork::React, CodeLanguage::Js) => ProjectType::ViteReactJs,
        (PackTool::Vite, FrameWork::React, CodeLanguage::Ts) => ProjectType::ViteReactTs,
        (PackTool::Rspack, FrameWork::React, CodeLanguage::Js) => ProjectType::RspackReactJs,
        (PackTool::Rspack, FrameWork::React, CodeLanguage::Ts) => ProjectType::RspackReactTs,
        (PackTool::Farm, FrameWork::React, _) => ProjectType::FarmReact,
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
        TemplateType::WebpackReactJsDir => Box::new(WebpackReactJsTemplate::iter()),
        TemplateType::WebpackReactTsDir => Box::new(WebpackReactTsTemplate::iter()),
        TemplateType::ViteReactJsDir => Box::new(ViteReactJsTemplate::iter()),
        TemplateType::ViteReactTsDir => Box::new(ViteReactTsTemplate::iter()),
        TemplateType::RspackReactJsDir => Box::new(RspackReactJsTemplate::iter()),
        TemplateType::RspackReactTsDir => Box::new(RspackReactTsTemplate::iter()),
        TemplateType::FarmReactDir => Box::new(FarmReactTemplate::iter()),
        TemplateType::CommonDir => Box::new(Common::iter()),
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
            TemplateType::WebpackReactJsDir => WebpackReactJsTemplate::get(filename),
            TemplateType::WebpackReactTsDir => WebpackReactTsTemplate::get(filename),
            TemplateType::ViteReactJsDir => ViteReactJsTemplate::get(filename),
            TemplateType::ViteReactTsDir => ViteReactTsTemplate::get(filename),
            TemplateType::RspackReactJsDir => RspackReactJsTemplate::get(filename),
            TemplateType::RspackReactTsDir => RspackReactTsTemplate::get(filename),
            TemplateType::FarmReactDir => FarmReactTemplate::get(filename),
            TemplateType::CommonDir => Common::get(filename),
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
    let mut replace_vec = match config.css {
        CssPreset::Sass => vec!["sass-loader", "scss"],
        CssPreset::Less => vec!["less-loader", "less"],
    };
    match config.loader {
        JsLoader::Babel => replace_vec.push("babel-loader"),
        JsLoader::Swc => replace_vec.push("swc-loader"),
        _ => {}
    }
    for (i, item) in replace_vec.iter().enumerate() {
        content = content.replace(&format!("`placeholder:{i}`"), item);
    }
    handle_result(fs::write(&path, content), "更新webpack rules失败");
    Ok(())
}

// 更新rsbuild.config.js
fn update_rsbuild_config(project_dir: &PathBuf, config: InlineConfig) -> Result<()> {
    let mut path = project_dir.clone();
    match config.lang {
        CodeLanguage::Js => path.push("rsbuild.config.mjs"),
        CodeLanguage::Ts => path.push("rsbuild.config.ts"),
    }
    let mut content = fs::read_to_string(&path).unwrap();
    let replace_vec = match config.css {
        CssPreset::Sass =>
            vec!["import { pluginSass } from '@rsbuild/plugin-sass';", "pluginSass()"],
        CssPreset::Less =>
            vec!["import { pluginLess } from '@rsbuild/plugin-less';", "pluginLess()"],
    };
    for (i, item) in replace_vec.iter().enumerate() {
        content = content.replace(&format!("`placeholder:{i}`"), item);
    }
    handle_result(fs::write(&path, content), "更新rsbuild.config失败");
    Ok(())
}
