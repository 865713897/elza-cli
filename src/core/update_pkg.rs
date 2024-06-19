use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use crate::logger;
use crate::project::DependenciesMod;

pub fn update_pkg_basic(project_dir: &PathBuf, project_name: String) -> Result<()> {
    let mut path = PathBuf::from(project_dir);
    path.push("package.json");
    let content = fs::read_to_string(&path)?;
    let mut json: Value = serde_json::from_str(&content)?;

    json.as_object_mut().unwrap().insert("name".to_string(), Value::String(project_name));
    let modified_json_str = serde_json::to_string_pretty(&json)?;
    fs::write(&path, modified_json_str)?;

    Ok(())
}

pub fn update_pkg_dependencies(
    project_dir: &PathBuf,
    dependency_name: &str,
    dependency_version: &str,
    mode: DependenciesMod
) -> Result<()> {
    logger::event(&format!("开始添加依赖: {} => {}", dependency_name, dependency_version));
    let mut path = PathBuf::from(project_dir);
    path.push("package.json");
    let content = fs::read_to_string(&path)?;
    let mut json: Value = serde_json::from_str(&content)?;

    let dev_or_prod = match mode {
        DependenciesMod::Dev => "devDependencies".to_string(),
        DependenciesMod::Prod => "dependencies".to_string(),
    };
    let deps = json
        .as_object_mut()
        .unwrap()
        .entry(dev_or_prod)
        .or_insert(Value::Object(serde_json::Map::new()));
    deps.as_object_mut()
        .unwrap()
        .insert(dependency_name.to_string(), Value::String(dependency_version.to_string()));
    let modified_json_str = serde_json::to_string_pretty(&json)?;
    fs::write(&path, modified_json_str)?;
    Ok(())
}
