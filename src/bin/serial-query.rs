use altherma_gateway::serial::reg_query;
use tokio::io::AsyncWriteExt;
use tokio_serial::SerialPortBuilderExt;


use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    registry_id: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let dev = "/dev/ttyUSB0";
    let mut sender = tokio_serial::new(dev, 9600).open_native_async()?;
    println!("Sending to {} ...", dev);
    let frame = reg_query(cli.registry_id);
    sender.write_all(&frame).await?;
    Ok(())
}
