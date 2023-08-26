use altherma_gateway::{
    commands::{
        can_monitor, convert_data, default_config, gateway, mqtt_test, serial_monitor, serial_query,
    },
    config::Config,
};
use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Parser)]
#[command(name = "altherma-gateway")]
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
    ConvertData,
    #[clap(about = "Print default configuration")]
    DefaultConfig,
    Gateway,
    MqttTest,
    SerialMonitor,
    SerialQuery(serial_query::Args),
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Cli::parse();

    let config = Config::load(&args.config_file)?;

    match args.command {
        Command::CanMonitor => can_monitor::cmd(config).await,
        Command::ConvertData => convert_data::cmd(),
        Command::DefaultConfig => default_config::cmd(),
        Command::Gateway => gateway::cmd(config).await,
        Command::MqttTest => mqtt_test::cmd(config).await,
        Command::SerialQuery(args) => serial_query::cmd(config, args).await,
        Command::SerialMonitor => serial_monitor::cmd(config).await,
    }
}
