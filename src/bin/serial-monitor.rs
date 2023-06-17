use tokio::io::AsyncReadExt;
use tokio_serial::SerialPortBuilderExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev = "/dev/ttyUSB0";
    let mut receiver = tokio_serial::new(dev, 9600).open_native_async()?;
    println!("Receiving from {} ...", dev);
    let mut buf = [0u8; 32];
    loop {
        let count = receiver.read(&mut buf).await?;
        println!("<<< {:?}", &buf[count..]);
    }
}
