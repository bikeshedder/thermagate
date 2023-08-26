use tokio::io::AsyncReadExt;

use crate::{config::Config, serial};

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = serial::open_stream(&config.serial)?;
    println!("Receiving from {} ...", config.serial.path);
    let mut buf = [0u8; 32];
    loop {
        let count = stream.read(&mut buf).await?;
        if count == 0 {
            continue;
        }
        println!("<<< {:?}", &buf[..count]);
    }
}
