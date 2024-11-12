use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init {
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },
    #[command(alias = "v")]
    Version,
    #[command(alias = "h")]
    Help,
    #[command(aliases = &["i", "ins, add"])]
    Install {
        #[arg(value_name = "PACKAGE")]
        package: Option<String>,
        #[arg(value_name = "FLAG")]
        flag: Option<String>,
    },
    #[command(alias = "r")]
    Run {
        #[arg(value_name = "ENV")]
        env: Option<String>,
    },
    Dev,
    Build,
    Lint,
    Start,
    #[command(aliases = &["up", "upgrade"])]
    Update {
        #[arg(value_name = "PACKAGE")]
        package: Option<String>,
    },
    #[command(aliases = &["uninstall", "delete", "remove"])]
    Uninstall {
        #[arg(value_name = "PACKAGE", required = true)]
        package: String,
    },
    #[command(aliases = &["c", "g", "generate", "make"])]
    Create {
        #[arg(value_name = "SCHEMATIC")]
        schematic: Option<String>,

        #[arg(value_name = "NAME")]
        name: Option<String>,
    },
}
