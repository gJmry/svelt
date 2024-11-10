use std::fs;
use std::path::PathBuf;
use serde_json::Value;
use is_svelte_project::get_root_path;
use crate::utils::is_svelte_project;
use crate::utils::is_svelte_project::get_current_directory;

pub fn main() {
    let root_path = get_root_path(get_current_directory());
    print!("{}", get_version(&root_path));
}

fn get_version(root_path: &PathBuf) -> String {
    let package_json_path = root_path.join("package.json");

    if !package_json_path.exists() {
        return "No package.json".to_string();
    }

    let package_json_content = match fs::read_to_string(&package_json_path) {
        Ok(content) => content,
        Err(_) => return "Error when fetching package.json".to_string(),
    };

    let json: Value = match serde_json::from_str(&package_json_content) {
        Ok(json) => json,
        Err(_) => return "Error when checking package.json".to_string(),
    };

    if let Some(dev_dependencies) = json.get("devDependencies") {
        if let Some(svelte) = dev_dependencies.get("svelte") {
            svelte.to_string()
        } else {
            "Svelte not found in devDependencies".to_string()
        }
    } else {
        "No devDependencies found".to_string()
    }
}