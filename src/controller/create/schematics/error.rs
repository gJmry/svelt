use crate::controller::utils::path_utils;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

pub fn main(name: String) {
    if let Err(e) = make_error_file(name) {
        eprintln!("Error: {}", e);
    }
}

fn make_error_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/+error.svelte", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("Error file already exists");
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = error_file_configuration();
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!("Successfully generated: {}", display_path);

    Ok(())
}

fn error_file_configuration() -> String {
    format!(
        "<script>
    import {{ page }} from '$app/stores';
</script>

<h1>{{$page.status}}: {{$page.error.message}}</h1>
",
    )
}

