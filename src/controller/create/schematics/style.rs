use crate::controller::utils::path_utils;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

pub fn main(name: String, style: u8) {
    match style {
        0 => { make_css_file(name).expect("Error when creating style file"); }
        1 => { make_scss_file(name).expect("Error when creating style file"); }
        2 => { make_sass_file(name).expect("Error when creating style file"); }
        _ => {}
    }
}

fn make_css_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/styles.css", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("css file already exists");
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = style_file_configuration();
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!("Successfully generated: {}", display_path);

    Ok(())
}

fn make_scss_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/styles.scss", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("scss file already exists");
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = style_file_configuration();
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!("Successfully generated: {}", display_path);

    Ok(())
}

fn make_sass_file(name: String) -> Result<()>{
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/styles.sass", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("sass file already exists");
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = style_file_configuration();
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!("Successfully generated: {}", display_path);

    Ok(())
}

fn style_file_configuration() -> String {
    format!("body {{

}}")
}