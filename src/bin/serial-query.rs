use altherma_gateway::serial::reg_query;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio_serial::SerialPortBuilderExt;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    registry_id: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let dev = "/dev/ttyUSB0";
    let mut stream = tokio_serial::new(dev, 9600)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::Even)
        .stop_bits(tokio_serial::StopBits::One)
        .open_native_async()?;
    println!("Sending to {} ...", dev);
    let request = reg_query(cli.registry_id);
    println!(">>> {:?}", request);
    stream.write_all(&request).await?;
    let mut response = [0u8; 18];
    stream.read_exact(&mut response).await?;
    println!("<<< {:?}", response);
    Ok(())
}
