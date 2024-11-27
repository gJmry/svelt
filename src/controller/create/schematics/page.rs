use crate::controller::create::schematics::{error, layout, param, script, server, style};
use crate::controller::utils::path_utils;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

pub fn main(name: String, args: Vec<String>){
    if let Err(e) = make_page_file(name.clone()) {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = parse_args(name.clone(), args){
        eprintln!("Error: {}", e);
    }
}   

fn parse_args(name: String, args: Vec<String>) -> Result<()>{
    for arg in args.iter(){
        let page_name = name.clone();

        match arg.as_str() {
            "layout" => layout::main(page_name),
            "l" => layout::main(page_name),
            "error" => error::main(page_name),
            "e" => error::main(page_name),
            "script" => script::main(page_name, true),
            "ts" => script::main(page_name, true),
            "js" => script::main(page_name, false),
            "param" => param::main(page_name),
            "p" => param::main(page_name),
            "css" => style::main(page_name, 0),
            "scss" => style::main(page_name, 1),
            "sass" => style::main(page_name, 2),
            "server" => server::main(page_name),
            _ => {
                println!("Invalid argument: {}", arg);
            } }
    }
    Ok(())
}

fn make_page_file(name: String) -> Result<()> {
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/+page.svelte", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("Page file already exists");
        std::process::exit(0);
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = page_file_configuration(name);
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!("Successfully generated: {}", display_path);

    Ok(())
}

fn page_file_configuration(name: String) -> String {
    format!(
        r#"
<script></script>
<style></style>
<main>
    {} page
</main>
"#,
        name
    )
}
