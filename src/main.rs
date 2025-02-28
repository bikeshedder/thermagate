use clap::{Parser, Subcommand};
use thermagate::{
    catalog::Catalog,
    commands::{can_monitor, default_config, gateway, set_param},
    config::Config,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Parser)]
#[command(name = "thermagate")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(default_value = "config.toml")]
    config_file: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    CanMonitor,
    #[clap(about = "Print default configuration")]
    DefaultConfig,
    CheckConfig,
    Gateway,
    SetParam(set_param::Args),
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Cli::parse();

    if matches!(args.command, Command::DefaultConfig) {
        return default_config::cmd();
    }

    let config = Config::load(&args.config_file)?;

    let catalog = Catalog::load()?;

    for param in &config.query.params {
        if catalog.param_by_name(&param.param).is_none() {
            // XXX Add better error handling
            panic!("Unknown query param in config: {}", param.param);
        }
    }

    match args.command {
        Command::CanMonitor => can_monitor::cmd(config, catalog).await,
        Command::DefaultConfig => unreachable!(),
        Command::CheckConfig => Ok(()),
        Command::Gateway => gateway::cmd(config, catalog).await,
        Command::SetParam(args) => set_param::cmd(config, catalog, args).await,
    }
}
