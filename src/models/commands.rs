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
    Version,
    Help,
}