use super::build::ProjectType;
use super::cli::DependenciesMod;
use crate::logger;
use anyhow::{ Ok, Result };
use serde_json::{ Map, Value };
use std::fs;
use std::path::PathBuf;

pub struct PackageJson {
    project_dir: PathBuf,
    json: Value,
}

#[derive(Debug, Clone)]
pub struct PackageBasicInfo {
    pub name: String,
    pub project_type: ProjectType,
}

impl PackageJson {
    pub fn new(project_dir: &PathBuf) -> Result<Self> {
        let mut path = project_dir.clone();
        path.push("package.json");
        let content = fs::read_to_string(&path).unwrap();
        let json = serde_json::from_str(&content).unwrap();
        Ok(Self {
            project_dir: project_dir.clone(),
            json,
        })
    }

    pub fn update_basic(&mut self, basic_info: PackageBasicInfo) -> Result<()> {
        self.json["name"] = Value::String(basic_info.name);
        match basic_info.project_type {
            ProjectType::WebpackReactJs => {
                set_scripts(
                    &mut self.json,
                    "cross-env NODE_ENV=development webpack-dev-server --config ./scripts/webpack.dev.js",
                    "cross-env NODE_ENV=production webpack --config ./scripts/webpack.prod.js",
                    None
                );
            }
            ProjectType::WebpackReactTs => {
                set_scripts(
                    &mut self.json,
                    "cross-env NODE_ENV=development webpack-dev-server --config ./scripts/webpack.dev.ts",
                    "cross-env NODE_ENV=production webpack --config ./scripts/webpack.prod.ts",
                    None
                );
            }
            ProjectType::ViteReactJs => {
                self.json["type"] = Value::String("module".to_string());
                set_scripts(&mut self.json, "vite", "vite build", Some("vite preview"));
            }
            ProjectType::ViteReactTs => {
                self.json["type"] = Value::String("module".to_string());
                set_scripts(&mut self.json, "vite", "vite build", Some("vite preview"));
            }
            ProjectType::RspackReactJs => {
                set_scripts(
                    &mut self.json,
                    "rsbuild dev",
                    "rsbuild build",
                    Some("rsbuild preview")
                );
            }
            ProjectType::RspackReactTs => {
                set_scripts(
                    &mut self.json,
                    "rsbuild dev",
                    "rsbuild build",
                    Some("rsbuild preview")
                );
            }
            ProjectType::FarmReact => {
                set_scripts(&mut self.json, "farm start", "farm build", Some("farm preview"));
            }
            ProjectType::ElzaReactJs => {
                self.json["type"] = Value::String("module".to_string());
                set_scripts(&mut self.json, "elza dev", "elza build", None);
            }
            ProjectType::ElzaReactTs => {
                self.json["type"] = Value::String("module".to_string());
                set_scripts(&mut self.json, "elza dev", "elza build", None);
            }
        }
        Ok(())
    }

    pub fn update_dependencies(
        &mut self,
        dependency_name: &str,
        dependency_version: &str,
        mode: DependenciesMod
    ) -> Result<()> {
        logger::event(&format!("开始添加依赖: {} => {}", dependency_name, dependency_version));
        let dev_or_prod = match mode {
            DependenciesMod::Dev => "devDependencies",
            DependenciesMod::Prod => "dependencies",
        };
        let deps = self.json
            .as_object_mut()
            .and_then(|obj| obj.get_mut(dev_or_prod))
            .and_then(|value| value.as_object_mut())
            .ok_or_else(|| {
                anyhow::anyhow!("'{}' field not found or is not an object", dev_or_prod)
            })?;
        deps.insert(dependency_name.to_string(), Value::String(dependency_version.to_string()));

        Ok(())
    }

    pub fn sort(&mut self) {
        sort_json(&mut self.json);
    }

    pub fn write(&self) -> Result<()> {
        let mut path = self.project_dir.clone();
        path.push("package.json");
        let content = serde_json::to_string_pretty(&self.json).unwrap();
        fs::write(&path, content).unwrap();

        Ok(())
    }
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

fn set_scripts(json_value: &mut Value, start: &str, build: &str, preview: Option<&str>) {
    json_value["scripts"]["start"] = Value::String(start.to_string());
    json_value["scripts"]["build"] = Value::String(build.to_string());
    if let Some(preview_script) = preview {
        json_value["scripts"]["preview"] = Value::String(preview_script.to_string());
    }
}
