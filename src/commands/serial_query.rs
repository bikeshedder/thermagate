use crate::{serial::{reg_query, self}, config::Config};

use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Parser)]
pub struct Args {
    registry_id: u8,
    response_length: usize,
}

pub async fn cmd(config: Config, args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = serial::open_stream(&config.serial)?;
    println!("Sending to {} ...", config.serial.path);
    let request = reg_query(args.registry_id);
    println!(">>> {:?}", request);
    stream.write_all(&request).await?;
    let mut response = vec![0u8; args.response_length];
    stream.read_exact(&mut response).await?;
    println!("<<< {:?}", response);
    Ok(())
}
