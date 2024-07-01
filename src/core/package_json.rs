use anyhow::Result;
use serde_json::{ Value, Map };
use std::fs;
use std::path::PathBuf;
use crate::logger;
use super::cli::{ DependenciesMod, CodeLanguage };

pub fn update_pkg_basic(project_dir: &PathBuf, project_name: String) -> Result<()> {
    let mut json = read_package_json(project_dir)?;
    json.as_object_mut().unwrap().insert("name".to_string(), Value::String(project_name));
    write_package_json(project_dir, &json)
}

pub fn update_pkg_scripts(project_dir: &PathBuf, lang: CodeLanguage) -> Result<()> {
    let mut path = project_dir.clone();
    path.push("package.json");
    let mut content = fs::read_to_string(&path).unwrap();

    if lang == CodeLanguage::Ts {
        content = content.replace(".js", ".ts");
        fs::write(&path, content).unwrap();
    }

    Ok(())
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

pub fn sort_package_json(project_dir: &PathBuf) -> Result<()> {
    let mut json = read_package_json(project_dir)?;
    sort_json(&mut json);
    write_package_json(project_dir, &json)
}

fn sort_json(json_value: &mut Value) {
    match json_value {
        Value::Object(map) => {
            // 针对dependencies和devDependencies进行排序
            if let Some(dependencies) = map.get_mut("dependencies") {
                sort_map(dependencies);
            }
            if let Some(dev_dependencies) = map.get_mut("devDependencies") {
                sort_map(dev_dependencies);
            }
        }
        Value::Array(array) => {
            for value in array {
                sort_json(value);
            }
        }
        _ => {}
    }
}

fn sort_map(json_value: &mut Value) {
    if let Value::Object(map) = json_value {
        let mut sorted_map = Map::new();
        let mut keys: Vec<_> = map.keys().collect();
        keys.sort();
        for key in keys {
            let value = map.get(key).unwrap().clone();
            let mut sorted_value = value.clone();
            sort_json(&mut sorted_value);
            sorted_map.insert(key.clone(), sorted_value);
        }
        *map = sorted_map;
    }
}
