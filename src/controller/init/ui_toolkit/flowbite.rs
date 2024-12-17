use crate::controller::init::ui_toolkit::tailwind;
use crate::NPM;
use std::process::Command;

pub fn main(project_dir: &str) {
    tailwind::main(project_dir);
    configure_flowbite(project_dir);
}

fn configure_flowbite(project_dir: &str) {
    install_flowbite(project_dir);
    add_configuration_flowbite(project_dir);
}

fn install_flowbite(project_dir: &str) {
    Command::new(NPM)
        .current_dir(project_dir)
        .arg("i")
        .arg("-D")
        .arg("flowbite-svelte")
        .arg("flowbite")
        .status()
        .expect("Failed to install flowbite.");

    Command::new(NPM)
        .current_dir(project_dir)
        .arg("i")
        .arg("-D")
        .arg("flowbite-svelte-icons")
        .status()
        .expect("Failed to install flowbite.");
}

fn add_configuration_flowbite(project_dir: &str) {
    let config_code = r#"import flowbite from 'flowbite/plugin';

const config = {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#FFF5F2',
          100: '#FFF1EE',
          200: '#FFE4DE',
          300: '#FFD5CC',
          400: '#FFBCAD',
          500: '#FE795D',
          600: '#EF562F',
          700: '#EB4F27',
          800: '#CC4522',
          900: '#A5371B'
        },
      },
    }
  },
  plugins: [flowbite],
};

export default config;
"#;

    tailwind::update_tailwindconfig(project_dir, config_code);
}
