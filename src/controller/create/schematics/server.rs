use crate::controller::utils::path_utils;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

pub fn main(name: String) {
    if let Err(e) = make_server_file(name) {
        eprintln!("Error: {}", e);
    }
}

fn make_server_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/+page.server.ts", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("Server file already exists");
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = server_file_configuration();
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!("Successfully generated: {}", display_path);

    Ok(())
}

fn server_file_configuration() -> String {
    format!(
        "import {{ error }} from '@sveltejs/kit';

export async function load() {{
  try {{
    const data = await fetchDataFromApi();
    return {{ props: {{ data }} }};
  }} catch (err) {{
    throw error(500, 'Failed to fetch data');
  }}
}}

async function fetchDataFromApi() {{
  const success = false;

  if (!success) {{
    throw new Error('Data fetch failed');
  }}

  return {{ message: 'Fetched data successfully!' }};
}}",
    )
}

