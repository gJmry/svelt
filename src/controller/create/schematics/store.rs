use std::fs::{create_dir_all, File};
use std::io::{Write, Result};
use std::path::Path;
use crate::controller::utils::path_utils;

pub fn main(name: String) {
    if let Err(e) = make_store_file(name) {
        eprintln!("Error: {}", e);
    }
}
fn make_store_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/stores/{}.js", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("Store file already exists");
        std::process::exit(0);
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = store_file_configuration(name);
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!(
        "Successfully generated: {}",
        display_path
    );

    Ok(())
}

fn store_file_configuration(name: String) -> String {
    format!(
        r#"
import {{ writable }} from 'svelte/store';

export const {} = writable('initial value');

export function setStoreValue(value) {{
    {}.set(value);
}}

export function resetStore() {{
    {}.set('initial value');
}}
"#,
        name,
        name,
        name
    )
}
