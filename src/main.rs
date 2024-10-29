use clap::{Parser, Subcommand};
use thermagate::{
    commands::{can_can, can_monitor, default_config, gateway},
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
    Gateway,
    CanCan,
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

    match args.command {
        Command::CanMonitor => can_monitor::cmd(config).await,
        Command::DefaultConfig => unreachable!(),
        Command::Gateway => gateway::cmd(config).await,
        Command::CanCan => can_can::cmd(config).await,
    }
}
