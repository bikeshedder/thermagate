use crate::{serial::reg_query, config::Config};

use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;

#[derive(Debug, Parser)]
pub struct Args {
    registry_id: u8,
    response_length: usize,
}

pub async fn cmd(config: Config, args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = tokio_serial::new(&config.serial.path, 9600)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::Even)
        .stop_bits(tokio_serial::StopBits::One)
        .open_native_async()?;
    println!("Sending to {} ...", config.serial.path);
    let request = reg_query(args.registry_id);
    println!(">>> {:?}", request);
    stream.write_all(&request).await?;
    let mut response = vec![0u8; args.response_length];
    stream.read_exact(&mut response).await?;
    println!("<<< {:?}", response);
    Ok(())
}
