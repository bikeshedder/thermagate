use serde::{Serialize, Deserialize};
use tokio_serial::{SerialPortBuilderExt, SerialStream, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct SerialConfig {
    pub path: String,
    pub baud_rate: u32,
    pub data_bits: tokio_serial::DataBits,
    pub flow_control: tokio_serial::FlowControl,
    pub parity: tokio_serial::Parity,
    pub stop_bits: tokio_serial::StopBits,
}

fn crc(buf: &[u8]) -> u8 {
    !buf.iter().fold(0u8, |s, &b| s.wrapping_add(b))
}

pub fn reg_query(reg_id: u8) -> Vec<u8> {
    let mut msg = vec![0x02, reg_id];
    msg.push(crc(&msg));
    msg
}

pub fn open_stream(config: &SerialConfig) -> Result<SerialStream, Error> {
    tokio_serial::new(&config.path, config.baud_rate)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::Even)
        .stop_bits(tokio_serial::StopBits::One)
        .open_native_async()
}
