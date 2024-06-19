use std::path::PathBuf;
use std::fs;
use std::io::{ BufRead, BufReader };
use std::thread;
use std::process::{ Command, Stdio, exit };
use anyhow::Result;
use rust_embed::RustEmbed;

use crate::UIDesign;
use crate::CssPreset;
use crate::PackageManger;
use crate::utils::logger;
use crate::update_pkg::{ update_pkg_basic, update_pkg_dependencies };

pub enum DependenciesMod {
    Dev,
    Prod,
}

pub struct InlineConfig {
    pub ui: UIDesign,
    pub css: CssPreset,
    pub pkg_mgr: PackageManger,
}

#[derive(RustEmbed)]
#[folder = "react-template/"]
struct Template;

struct Dependency {
    name: &'static str,
    version: &'static str,
}

impl UIDesign {
    fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            UIDesign::Antd => vec![Dependency { name: "antd", version: "^5.3.0" }],
            UIDesign::ElementPlus => vec![Dependency { name: "element-plus", version: "^2.7.5" }],
        }
    }
}

impl CssPreset {
    fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            CssPreset::Sass =>
                vec![
                    Dependency { name: "sass-loader", version: "^14.2.1" },
                    Dependency { name: "sass", version: "^1.77.6" }
                ],
            CssPreset::Less =>
                vec![
                    Dependency { name: "less", version: "^4.1.3" },
                    Dependency { name: "less-loader", version: "^11.1.0" }
                ],
        }
    }
}

// 项目初始化
pub fn init(project_name: &str, config: InlineConfig) -> Result<()> {
    // 初始化项目路径
    let project_dir = PathBuf::from(project_name);
    logger::event("开始创建项目目录");
    match fs::create_dir(&project_dir) {
        Ok(_) => {
            logger::info("创建项目目录成功");
        }
        Err(e) => {
            logger::error(&format!("创建项目目录失败: {}", e));
            return Ok(());
        }
    }
    // 遍历template内部文件，并写入
    for filename in Template::iter() {
        let file_content = Template::get(filename.as_ref()).unwrap();
        let mut file_path = PathBuf::from(&project_dir);
        file_path.push(filename.as_ref());
        let mut directory_path = PathBuf::from(&file_path);
        directory_path.pop();

        logger::event(&format!("开始创建文件: {}", filename.as_ref()));
        fs::create_dir_all(directory_path)?;
        fs::write(file_path, file_content.data)?;
    }
    logger::info("文件创建完成");
    // 更新package.json
    let _ = update_pkg_basic(&project_dir, project_name.to_owned());
    // 引入UI框架相关的所有依赖项
    for dependency in config.ui.get_dependencies() {
        update_pkg_dependencies(
            &project_dir,
            dependency.name,
            dependency.version,
            DependenciesMod::Prod
        )?;
    }
    // 引入CSS预处理器相关的所有依赖项
    for dependency in config.css.get_dependencies() {
        update_pkg_dependencies(
            &project_dir,
            dependency.name,
            dependency.version,
            DependenciesMod::Dev
        )?;
    }
    logger::info("预设依赖项添加完成");
    git_init(&project_dir);
    install_dependencies(&project_dir, match config.pkg_mgr {
        PackageManger::Npm => "npm",
        PackageManger::Yarn => "yarn",
        PackageManger::Pnpm => "pnpm",
        PackageManger::Cnpm => "cnpm",
    });
    logger::ready("项目初始化完成");
    Ok(())
}

// 初始化git仓库
fn git_init(project_dir: &PathBuf) {
    // 初始化git
    logger::event("开始初始化git仓库");
    let git_init_cmd = Command::new("git")
        .current_dir(project_dir)
        .arg("init")
        .stdout(Stdio::null())
        .status()
        .expect("git初始化失败");
    if !git_init_cmd.success() {
        logger::error("git初始化失败");
        exit(1);
    }
    logger::event("git init");
    // git add
    let git_add_cmd = Command::new("git")
        .current_dir(project_dir)
        .arg("add")
        .arg("-A")
        .stdout(Stdio::null())
        .status()
        .expect("git暂存失败");
    if !git_add_cmd.success() {
        logger::error("git暂存失败");
        exit(1);
    }
    logger::event("git add -A");
    // git commit
    let git_add_cmd = Command::new("git")
        .current_dir(project_dir)
        .arg("commit")
        .arg("-m")
        .arg("initial commit")
        .stdout(Stdio::null())
        .status()
        .expect("git提交失败");
    if !git_add_cmd.success() {
        logger::error("git提交失败");
        exit(1);
    }
    logger::event("git commit -m 'initial commit'");
}

// 安装依赖
fn install_dependencies(project_dir: &PathBuf, pkg_mgr: &str) {
    logger::event("开始安装依赖");
    logger::event(&format!("{} install", pkg_mgr));

    let mut child = Command::new(pkg_mgr)
        .current_dir(project_dir)
        .arg("install")
        .stdout(Stdio::piped()) // 使用管道捕获stdout
        .spawn()
        .expect("依赖安装进程启动失败");

    // 获取命令的输出流
    if let Some(output) = child.stdout.take() {
        let reader = BufReader::new(output);

        // 在另一个线程中读取和打印输出
        thread::spawn(move || {
            reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| {
                    println!("{}", line); // 根据需要修改为使用 logger::event 等函数
                });
        });
    }

    // 等待命令执行完成
    let result = child.wait().expect("依赖安装过程中发生错误");

    if !result.success() {
        logger::error("依赖安装失败");
        std::process::exit(1);
    }
}
