use tokio::io::AsyncReadExt;
use tokio_serial::SerialPortBuilderExt;

pub async fn cmd() -> Result<(), Box<dyn std::error::Error>> {
    let dev = "/dev/ttyUSB0";
    let mut receiver = tokio_serial::new(dev, 9600)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::Even)
        .stop_bits(tokio_serial::StopBits::One)
        .open_native_async()?;
    println!("Receiving from {} ...", dev);
    let mut buf = [0u8; 32];
    loop {
        let count = receiver.read(&mut buf).await?;
        if count == 0 {
            continue;
        }
        println!("<<< {:?}", &buf[..count]);
    }
}
