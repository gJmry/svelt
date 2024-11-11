use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(alias = "create")]
    Init {
        #[arg(value_name = "NAME")]
        name: Option<String>,
        #[arg(value_name = "UI_TOOLKIT")]
        ui_toolkit: Option<String>,
    },
    #[command(alias = "v")]
    Version,
    #[command(alias = "h")]
    Help,
    #[command(aliases = &["i", "ins, add"])]
    Install{
        #[arg(value_name = "PACKAGE")]
        package: Option<String>,
        #[arg(value_name = "FLAG")]
        flag: Option<String>,
    },
}