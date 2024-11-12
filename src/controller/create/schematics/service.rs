use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;
use crate::controller::utils::path_utils;

pub fn main(name: String){
    if let Err(e) = make_service_file(name) {
        eprintln!("Error: {}", e);
    }
}

fn make_service_file(name: String) -> Result<()>{
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/services/{}.js", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("Service file already exists");
        std::process::exit(0);
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = service_file_configuration(name);
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!(
        "Successfully generated: {}",
        display_path
    );

    Ok(())
}

fn service_file_configuration(name: String) -> String {
    format!(
        r#"
export const {} = {{
  data: [],
  getData() {{
    return this.data;
  }},

  addData(newItem) {{
    this.data.push(newItem);
  }},

  resetData() {{
    this.data = [];
  }},
}};
"#,
        name
    )
}
