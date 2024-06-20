use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use crate::logger;
use crate::cli::DependenciesMod;

pub fn update_pkg_basic(project_dir: &PathBuf, project_name: String) -> Result<()> {
    let mut json = read_package_json(project_dir)?;
    json.as_object_mut().unwrap().insert("name".to_string(), Value::String(project_name));
    write_package_json(project_dir, &json)
}

pub fn update_pkg_dependencies(
    project_dir: &PathBuf,
    dependency_name: &str,
    dependency_version: &str,
    mode: DependenciesMod
) -> Result<()> {
    logger::event(&format!("开始添加依赖: {} => {}", dependency_name, dependency_version));

    let mut json = read_package_json(project_dir)?;

    let dev_or_prod = match mode {
        DependenciesMod::Dev => "devDependencies",
        DependenciesMod::Prod => "dependencies",
    };

    let deps = json
        .as_object_mut()
        .and_then(|obj| obj.get_mut(dev_or_prod))
        .and_then(|value| value.as_object_mut())
        .ok_or_else(|| anyhow::anyhow!("'{}' field not found or is not an object", dev_or_prod))?;

    deps.insert(dependency_name.to_string(), Value::String(dependency_version.to_string()));

    write_package_json(project_dir, &json)
}

fn read_package_json(project_dir: &PathBuf) -> Result<Value> {
    let mut path = project_dir.clone();
    path.push("package.json");
    let content = fs::read_to_string(&path).unwrap();

    let json: Value = serde_json::from_str(&content).unwrap();

    Ok(json)
}

fn write_package_json(project_dir: &PathBuf, json: &Value) -> Result<()> {
    let mut path = project_dir.clone();
    path.push("package.json");
    let modified_json_str = serde_json::to_string_pretty(json).unwrap();

    fs::write(&path, modified_json_str).unwrap();
    Ok(())
}

