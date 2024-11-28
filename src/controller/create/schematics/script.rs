use crate::controller::utils::path_utils;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

pub fn main(name: String, typescript: bool) {
    match typescript {
        true => { make_typescript_file(name).expect("Error when creating typescript file"); }
        false => { make_javascript_file(name).expect("Error when creating javascript file"); }
    }
}

fn make_javascript_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/+page.js", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("+page.js file already exists");
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = script_file_configuration();
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!("Successfully generated: {}", display_path);

    Ok(())
}

fn make_typescript_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/+page.ts", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("+page.ts file already exists");
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = script_file_configuration();
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!("Successfully generated: {}", display_path);

    Ok(())
}

fn script_file_configuration() -> String {
    "console.log('Hello World');".to_string()
}