use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_root_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut current_path = path.as_ref().to_path_buf();

    while let Some(parent) = current_path.parent() {
        let package_json_path = parent.join("package.json");

        if package_json_path.exists() {
            return parent.to_path_buf();
        }
        current_path = parent.to_path_buf();
    }

    path.as_ref().to_path_buf()
}

fn is_svelte_project<P: AsRef<Path>>(path: P) -> bool {
    let root_path = get_root_path(path);
    let package_json_path = root_path.join("package.json");

    if !package_json_path.exists() {
        return false;
    }

    let package_json_content = match fs::read_to_string(&package_json_path) {
        Ok(content) => content,
        Err(_) => return false,
    };

    let json: Value = match serde_json::from_str(&package_json_content) {
        Ok(json) => json,
        Err(_) => return false,
    };

    json.get("dependencies")
        .and_then(|deps| deps.get("svelte"))
        .is_some()
        || json
            .get("devDependencies")
            .and_then(|dev_deps| dev_deps.get("svelte"))
            .is_some()
}

pub fn get_current_directory() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::new())
}

pub fn main() -> bool {
    let current_dir = get_current_directory();
    is_svelte_project(current_dir)
}
