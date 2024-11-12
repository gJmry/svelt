use crate::controller::utils::path_utils;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

pub fn main(name: String) {
    if let Err(e) = make_component_file(name) {
        eprintln!("Error: {}", e);
    }
}

fn make_component_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/components/{}.svelte", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path){
        println!("Component file already exists");
        std::process::exit(0);
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = component_file_configuration(name);
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!(
        "Successfully generated: {}",
        display_path
    );

    Ok(())
}

fn component_file_configuration(name: String) -> String {
    format!(
        r#"<!-- {name}.svelte -->
<script lang="ts">
    export let message = "Hello from {name}!";
</script>

<style>
    h1 {{
        color: #ff3e00;
    }}
</style>

<h1>{{ message }}</h1>
"#
    )
}
