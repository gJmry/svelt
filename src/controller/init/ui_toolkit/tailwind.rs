use std::process::Command;
use crate::{NPM, NPX};
use crate::controller::utils::path_utils::{get_current_directory, get_root_path};

pub fn main(project_dir: &str) {
    install_tailwind(project_dir);

}

fn install_tailwind(project_dir: &str) {
    let npm_command = Command::new(NPM)
        .current_dir(project_dir)
        .arg("install")
        .arg("-D")
        .arg("tailwindcss")
        .arg("postcss")
        .arg("autoprefixer")
        .status()
        .expect("Failed to install tailwind");
}

fn init_tailwind(project_dir: &str) {
    let npm_command = Command::new(NPX)
        .current_dir(project_dir)
        .arg("tailwindcss")
        .arg("init")
        .arg("-p");
}

fn update_svelteconfig(){
    let root_path = get_root_path(get_current_directory());

}


