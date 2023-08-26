use tokio::io::AsyncReadExt;
use tokio_serial::SerialPortBuilderExt;

use crate::config::Config;

pub async fn cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut receiver = tokio_serial::new(&config.serial.path, config.serial.baud_rate)
        .data_bits(config.serial.data_bits)
        .parity(config.serial.parity)
        .stop_bits(config.serial.stop_bits)
        .flow_control(config.serial.flow_control)
        .open_native_async()?;
    println!("Receiving from {} ...", config.serial.path);
    let mut buf = [0u8; 32];
    loop {
        let count = receiver.read(&mut buf).await?;
        if count == 0 {
            continue;
        }
        println!("<<< {:?}", &buf[..count]);
    }
}
