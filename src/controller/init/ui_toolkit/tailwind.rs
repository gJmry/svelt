use std::fs::{self, OpenOptions};
use std::process::{Command, exit};
use std::io::{self, Write};
use std::path::Path;
use crate::{NPM, NPX};
use crate::controller::utils::path_utils::{get_current_directory, get_root_path};

pub fn main(project_dir: &str) {
    configure_tailwind(project_dir);
}

fn configure_tailwind(project_dir: &str) {
    install_tailwind(project_dir);
    init_tailwind(project_dir);
    update_svelteconfig(project_dir);
    update_tailwindconfig(project_dir);
    create_tailwind_css(project_dir);
    update_layout_svelte(project_dir);
}

fn install_tailwind(project_dir: &str) {
    Command::new(NPM)
        .current_dir(project_dir)
        .arg("install")
        .arg("-D")
        .arg("tailwindcss")
        .arg("postcss")
        .arg("autoprefixer")
        .status()
        .expect("Failed to install Tailwind CSS dependencies.");
}

fn init_tailwind(project_dir: &str) {
    Command::new(NPX)
        .current_dir(project_dir)
        .arg("tailwindcss")
        .arg("init")
        .arg("-p")
        .status()
        .expect("Failed to initialize Tailwind config.");
}

fn update_svelteconfig(project_dir: &str) {
    let svelte_config_path = Path::new(project_dir).join("svelte.config.js");

    let mut svelte_config = match OpenOptions::new().write(true).create(true).open(&svelte_config_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open or create svelte.config.js at {}", svelte_config_path.display());
            exit(1);
        }
    };

    let config_code = r#"
import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter()
  },
  preprocess: vitePreprocess()
};

export default config;
"#;

    writeln!(svelte_config, "{}", config_code).expect("Failed to write to svelte.config.js");
    println!("svelte.config.js has been updated to use vitePreprocess.");
}

fn update_tailwindconfig(project_dir: &str) {
    let mut tailwind_config = Path::new(project_dir).join("tailwind.config.js");
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open or create tailwind.config.js");
            exit(1);
        }
    };

    let config_code = r#"
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {}
  },
  plugins: []
};
"#;

    writeln!(tailwind_config, "{}", config_code).expect("Failed to write to tailwind.config.js");
    println!("tailwind.config.js has been updated with content paths.");
}

fn create_tailwind_css(project_dir: &str) {
    let css_path = format!("{}/src/app.css", project_dir);

    let mut css_file = match OpenOptions::new().write(true).create(true).open(css_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to create or open app.css");
            exit(1);
        }
    };

    let css_code = r#"
@tailwind base;
@tailwind components;
@tailwind utilities;
"#;

    writeln!(css_file, "{}", css_code).expect("Failed to write to app.css");
    println!("app.css has been created with Tailwind directives.");
}

fn update_layout_svelte(project_dir: &str) {
    let layout_svelte_path = format!("{}/src/routes/+layout.svelte", project_dir);
    let mut layout_file = match OpenOptions::new().write(true).create(true).open(layout_svelte_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to create or open +layout.svelte");
            exit(1);
        }
    };

    let import_code = r#"
<script>
  import "../app.css";
</script>

<slot />
"#;

    writeln!(layout_file, "{}", import_code).expect("Failed to write to +layout.svelte");
    println!("+layout.svelte has been updated to import app.css.");
}
