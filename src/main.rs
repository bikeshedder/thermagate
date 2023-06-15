use std::path::Path;

use futures_util::stream::StreamExt;
use socketcan::{Error, tokio::CanSocket, EmbeddedFrame};

mod rotex;

use rotex::RotexData;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let rotex_data = RotexData::read(Path::new("rotex-data.json"));
    println!("{:?}", rotex_data);

    let mut socket_rx = CanSocket::open("can0")?;
    //let socket_tx = CanSocket::open("vcan0")?;
    while let Some(Ok(frame)) = socket_rx.next().await {
        if let socketcan::CanFrame::Data(data_frame) = frame {
            println!("{:?}", data_frame.data());
        }
        //socket_tx.write_frame(frame)?.await?;
    }
    Ok(())
}
