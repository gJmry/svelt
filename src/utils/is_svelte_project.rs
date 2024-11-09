use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;
use std::sync::OnceCell;
use std::env;

struct ProjectInfo {
    root_path: OnceCell<PathBuf>,
}

impl ProjectInfo {
    fn get_root_path<P: AsRef<Path>>(path: P) -> PathBuf {
        static ROOT_PATH: OnceCell<PathBuf> = OnceCell::new();

        if let Some(cached_path) = ROOT_PATH.get() {
            return cached_path.clone();
        }

        let mut current_path = path.as_ref().to_path_buf();

        while let Some(parent) = current_path.parent() {
            let package_json_path = parent.join("package.json");

            if package_json_path.exists() {
                ROOT_PATH.set(parent.to_path_buf()).ok();
                return parent;
            }
            current_path = parent.to_path_buf();
        }

        ROOT_PATH.set(path.as_ref().to_path_buf()).ok();
        path.as_ref().to_path_buf()
    }

    fn is_svelte_project<P: AsRef<Path>>(path: P) -> bool {
        let root_path = Self::get_root_path(path);

        let package_json_path = root_path.join("package.json");
        if !package_json_path.exists() {
            return false;
        }

        let package_json_content = fs::read_to_string(&package_json_path);
        match package_json_content {
            Ok(content) => {
                let json: Value = match serde_json::from_str(&content) {
                    Ok(json) => json,
                    Err(_) => return false,
                };

                if let Some(dependencies) = json.get("dependencies") {
                    if dependencies.get("svelte").is_some() {
                        return true;
                    }
                }

                if let Some(dev_dependencies) = json.get("devDependencies") {
                    if dev_dependencies.get("svelte").is_some() {
                        return true;
                    }
                }
            }
            Err(_) => return false,
        }
        false
    }
}

fn get_current_directory() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::new())
}

fn main() {
    let current_dir = get_current_directory();

    if ProjectInfo::is_svelte_project(current_dir) {
        println!("C'est un projet Svelte !");
    } else {
        println!("Ce n'est pas un projet Svelte.");
    }
}
