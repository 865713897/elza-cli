use anyhow::{ Ok, Result };
use rust_embed::{ EmbeddedFile, RustEmbed };
use std::path::PathBuf;
use std::process::{ exit, Command, Stdio };
use std::{ fs, vec };

use crate::utils::error::{ handle_option, handle_result };
use crate::utils::logger;

use super::cli::{ CodeLanguage, CssPreset, Dependency, FrameWork, JsLoader };
use super::pack::PackTool;
use super::package_json::{ PackageBasicInfo, PackageJson };

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
#[folder = "common-react-js"]
struct CommonReactJs;

#[derive(RustEmbed)]
#[folder = "common-react-ts"]
struct CommonReactTs;

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

#[derive(RustEmbed)]
#[folder = "react/elza/template-js"]
struct ElzaReactJsTemplate;

#[derive(RustEmbed)]
#[folder = "react/elza/template-ts"]
struct ElzaReactTsTemplate;

#[derive(Clone)]
enum TemplateType {
    WebpackReactJsDir,
    WebpackReactTsDir,
    ViteReactJsDir,
    ViteReactTsDir,
    RspackReactJsDir,
    RspackReactTsDir,
    FarmReactDir,
    ElzaReactJsDir,
    ElzaReactTsDir,
    CommonDir,
    CommonReactJsDir,
    CommonReactTsDir,
}

impl TemplateType {
    fn get_file_content(&self, filename: &str) -> Option<EmbeddedFile> {
        match self {
            TemplateType::WebpackReactJsDir => WebpackReactJsTemplate::get(filename),
            TemplateType::WebpackReactTsDir => WebpackReactTsTemplate::get(filename),
            TemplateType::ViteReactJsDir => ViteReactJsTemplate::get(filename),
            TemplateType::ViteReactTsDir => ViteReactTsTemplate::get(filename),
            TemplateType::RspackReactJsDir => RspackReactJsTemplate::get(filename),
            TemplateType::RspackReactTsDir => RspackReactTsTemplate::get(filename),
            TemplateType::FarmReactDir => FarmReactTemplate::get(filename),
            TemplateType::ElzaReactJsDir => ElzaReactJsTemplate::get(filename),
            TemplateType::ElzaReactTsDir => ElzaReactTsTemplate::get(filename),
            TemplateType::CommonDir => Common::get(filename),
            TemplateType::CommonReactJsDir => CommonReactJs::get(filename),
            TemplateType::CommonReactTsDir => CommonReactTs::get(filename),
        }
    }
    fn iter_files(&self) -> Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>> {
        match self {
            TemplateType::WebpackReactJsDir => Box::new(WebpackReactJsTemplate::iter()),
            TemplateType::WebpackReactTsDir => Box::new(WebpackReactTsTemplate::iter()),
            TemplateType::ViteReactJsDir => Box::new(ViteReactJsTemplate::iter()),
            TemplateType::ViteReactTsDir => Box::new(ViteReactTsTemplate::iter()),
            TemplateType::RspackReactJsDir => Box::new(RspackReactJsTemplate::iter()),
            TemplateType::RspackReactTsDir => Box::new(RspackReactTsTemplate::iter()),
            TemplateType::FarmReactDir => Box::new(FarmReactTemplate::iter()),
            TemplateType::ElzaReactJsDir => Box::new(ElzaReactJsTemplate::iter()),
            TemplateType::ElzaReactTsDir => Box::new(ElzaReactTsTemplate::iter()),
            TemplateType::CommonDir => Box::new(Common::iter()),
            TemplateType::CommonReactJsDir => Box::new(CommonReactJs::iter()),
            TemplateType::CommonReactTsDir => Box::new(CommonReactTs::iter()),
        }
    }
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
    ElzaReactJs,
    ElzaReactTs,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CopyType {
    Template,
    Common,
}

// 项目初始化
pub fn start(project_name: &str, config: InlineConfig) -> Result<()> {
    // 初始化项目路径
    let project_dir = PathBuf::from(project_name);
    create_project_dir(&project_dir)?;
    // 获取项目类型
    let (project_type, template_type) = get_project_type(
        config.pack_tool,
        config.frame,
        config.lang
    );

    // 复制特定模板文件
    copy_template_files(&project_dir, template_type, config.clone(), CopyType::Template)?;

    // 复制公共文件
    copy_common_files(&project_dir, config, CopyType::Common)?;

    logger::info("文件创建完成");
    match config.pack_tool {
        PackTool::Webpack => {
            update_webpack_rules(&project_dir, config)?;
        }
        PackTool::Rsbuild => {
            update_rsbuild_config(&project_dir, config)?;
        }
        PackTool::Vite => {}
        PackTool::Farm => {
            update_farm_config(&project_dir, config)?;
        }
        PackTool::Elza => {}
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
    if config.pack_tool == PackTool::Webpack || config.pack_tool == PackTool::Rsbuild {
        logger::full_info(
            &format!(
                "[你知道吗？] {:?}模板内置约定式路由插件，依赖安装完成启动项目即可生成路由文件，详见 https://github.com/865713897/webpack-plugin-auto-routes",
                config.pack_tool
            )
        );
    }
    Ok(())
}

// 获取项目类型
fn get_project_type(
    pack_tool: PackTool,
    frame: FrameWork,
    lang: CodeLanguage
) -> (ProjectType, TemplateType) {
    match (pack_tool, frame, lang) {
        (PackTool::Webpack, FrameWork::React, CodeLanguage::Js) =>
            (ProjectType::WebpackReactJs, TemplateType::WebpackReactJsDir),
        (PackTool::Webpack, FrameWork::React, CodeLanguage::Ts) =>
            (ProjectType::WebpackReactTs, TemplateType::WebpackReactTsDir),
        (PackTool::Vite, FrameWork::React, CodeLanguage::Js) =>
            (ProjectType::ViteReactJs, TemplateType::ViteReactJsDir),
        (PackTool::Vite, FrameWork::React, CodeLanguage::Ts) =>
            (ProjectType::ViteReactTs, TemplateType::ViteReactTsDir),
        (PackTool::Rsbuild, FrameWork::React, CodeLanguage::Js) =>
            (ProjectType::RspackReactJs, TemplateType::RspackReactJsDir),
        (PackTool::Rsbuild, FrameWork::React, CodeLanguage::Ts) =>
            (ProjectType::RspackReactTs, TemplateType::RspackReactTsDir),
        (PackTool::Farm, FrameWork::React, _) =>
            (ProjectType::FarmReact, TemplateType::FarmReactDir),
        (PackTool::Elza, FrameWork::React, CodeLanguage::Js) =>
            (ProjectType::ElzaReactJs, TemplateType::ElzaReactJsDir),
        (PackTool::Elza, FrameWork::React, CodeLanguage::Ts) =>
            (ProjectType::ElzaReactTs, TemplateType::ElzaReactTsDir),
    }
}

// 创建项目目录
fn create_project_dir(project_dir: &PathBuf) -> Result<()> {
    logger::event("开始创建项目目录");
    handle_result(fs::create_dir(project_dir), "创建项目目录失败");
    logger::info("创建项目目录成功");
    Ok(())
}

// 复制公共模块代码
fn copy_common_files(
    project_dir: &PathBuf,
    config: InlineConfig,
    copy_type: CopyType
) -> Result<()> {
    // 复制通用文件
    copy_template_files(project_dir, TemplateType::CommonDir, config, copy_type.clone())?;
    // 根据不同模板类型复制不同文件
    let common_react_dir = match config.lang {
        CodeLanguage::Js => TemplateType::CommonReactJsDir,
        CodeLanguage::Ts => TemplateType::CommonReactTsDir,
    };
    copy_template_files(project_dir, common_react_dir, config, copy_type.clone())?;
    Ok(())
}

// 复制模板文件
fn copy_template_files(
    project_dir: &PathBuf,
    template_type: TemplateType,
    config: InlineConfig,
    copy_type: CopyType
) -> Result<()> {
    for filename in template_type.iter_files() {
        if should_skip_file(&filename, config, copy_type.clone()) {
            continue;
        }
        copy_template_file(project_dir, filename.as_ref(), template_type.clone())?;
    }
    Ok(())
}

// 判断是否应该忽略某个文件
fn should_skip_file(filename: &str, config: InlineConfig, copy_type: CopyType) -> bool {
    match config.pack_tool {
        PackTool::Webpack => {
            (config.loader == JsLoader::Babel && filename.contains(".swcrc")) ||
                (config.loader == JsLoader::Swc && filename.contains("babel.config.json"))
        }
        PackTool::Rsbuild => copy_type == CopyType::Common && filename.contains("src/index"),
        PackTool::Farm | PackTool::Vite =>
            copy_type == CopyType::Common &&
                (filename.contains("src/index") ||
                    filename.contains("public/index.html") ||
                    filename.contains("src/router")),
        _ => false,
    }
}

// 遍历template内部文件，并写入
fn copy_template_file(
    project_dir: &PathBuf,
    filename: &str,
    template_type: TemplateType
) -> Result<()> {
    let file_content = handle_option(
        template_type.get_file_content(filename),
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
    let file_name = match config.lang {
        CodeLanguage::Js => "scripts/webpack.common.js",
        CodeLanguage::Ts => "scripts/webpack.common.ts",
    };
    let replace_vec = match (config.css, config.loader) {
        (CssPreset::Sass, JsLoader::Babel) => vec!["sass-loader", "scss", "babel-loader"],
        (CssPreset::Sass, JsLoader::Swc) => vec!["sass-loader", "scss", "swc-loader"],
        (CssPreset::Less, JsLoader::Babel) => vec!["less-loader", "less", "babel-loader"],
        (CssPreset::Less, JsLoader::Swc) => vec!["less-loader", "less", "swc-loader"],
        _ => vec![],
    };
    update_config_file(project_dir, file_name, replace_vec)
}

// 更新rsbuild.config.js
fn update_rsbuild_config(project_dir: &PathBuf, config: InlineConfig) -> Result<()> {
    let file_name = match config.lang {
        CodeLanguage::Js => "rsbuild.config.mjs",
        CodeLanguage::Ts => "rsbuild.config.ts",
    };
    let replace_vec = match config.css {
        CssPreset::Sass =>
            vec!["\nimport { pluginSass } from '@rsbuild/plugin-sass';", "pluginSass()"],
        CssPreset::Less =>
            vec!["\nimport { pluginLess } from '@rsbuild/plugin-less';", "pluginLess()"],
        _ => vec![],
    };
    update_config_file(project_dir, file_name, replace_vec)
}

// 更新farm.config.ts
fn update_farm_config(project_dir: &PathBuf, config: InlineConfig) -> Result<()> {
    let replace_vec = match config.css {
        CssPreset::Sass => vec!["", "'@farmfe/plugin-sass'"],
        CssPreset::Less => vec!["\nimport less from '@farmfe/js-plugin-less';", "less()"],
        _ => vec![],
    };
    update_config_file(project_dir, "farm.config.ts", replace_vec)
}

// 通用更新配置文件
fn update_config_file(
    project_dir: &PathBuf,
    file_name: &str,
    replace_vec: Vec<&str>
) -> Result<()> {
    let mut path = project_dir.clone();
    path.push(file_name);
    let mut content = fs::read_to_string(&path).unwrap();
    for (i, item) in replace_vec.iter().enumerate() {
        content = content.replace(&format!("`placeholder:{i}`"), item);
    }
    handle_result(fs::write(&path, content), &format!("更新 {:?} 失败", path));
    Ok(())
}
