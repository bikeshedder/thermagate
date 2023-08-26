use altherma_gateway::commands::{serial_query, serial_monitor, mqtt_test, gateway, can_monitor, convert_data};
use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Parser)]
#[command(name = "altherma-gateway")]
#[command(about = "Altherma Gateway", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Debug, Subcommand)]
enum Command {
    CanMonitor,
    ConvertData,
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

    match args.command {
        Command::CanMonitor => {
            can_monitor::cmd().await
        }
        Command::ConvertData => {
            convert_data::cmd()
        }
        Command::Gateway => {
            gateway::cmd().await
        }
        Command::MqttTest => {
            mqtt_test::cmd().await
        }
        Command::SerialQuery(args) => {
            serial_query::cmd(args).await
        }
        Command::SerialMonitor => {
            serial_monitor::cmd().await
        }
    }
}
