use std::fmt;

use num_enum::{FromPrimitive, IntoPrimitive};
use socketcan::{CanDataFrame, EmbeddedFrame, Frame};
use thiserror::Error;

use super::device::Device;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Message {
    pub sender: Device,
    pub receiver: Device,
    pub r#type: MessageType,
    pub param: u16,
    pub data: [u8; 2],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MessageType {
    Set = 0b0000,
    Request = 0b0001,
    Response = 0b0010,
    // The ping and pong message types are just an educated guess.
    // When looking at the bus monitor a `Pong` usually is sent
    // as response to a `Ping`. Though some devices (namely HC2)
    // do send `Pong` messages without a request. As this type of
    // message is only used for internal communication and we never
    // act upon it it doesn't matter if this is correct or not.
    Ping = 0b0110,
    Pong = 0b0111,
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug)]
pub struct Param {
    pub id: u16,
    pub data: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum FrameError {
    #[error("Invalid sender: {0}")]
    InvalidSender(u32),
    #[error("Invalid frame length: {0}")]
    InvalidFrameLength(usize),
}

impl Message {
    pub fn parse(sender_id: u16, raw: [u8; 7]) -> Self {
        let sender = Device::from(sender_id);
        let receiver =
            Device::from((((raw[0] & 0b11110000) as u16) << 3) + (raw[1] & 0b01111111) as u16);
        let (param, data) = if raw[2] == 0xFA {
            (u16::from_be_bytes([raw[3], raw[4]]), [raw[5], raw[6]])
        } else {
            (raw[2] as u16, [raw[3], raw[4]])
        };
        let r#type = MessageType::from(raw[0] & 0b00001111);
        Message {
            sender,
            receiver,
            r#type,
            param,
            data,
        }
    }
    pub fn compose(&self) -> [u8; 7] {
        let mut data = [0u8; 7];
        let addr = u16::from(self.receiver);
        data[0] = ((addr & 0b11110000000) >> 3) as u8 | u8::from(self.r#type);
        data[1] = (addr & 0b00001111111) as u8;
        // Parameter IDs < 0xFA can be encoded in two different ways. The
        // Rocon G1 gateways always uses the 0xFA encoded form. Even for
        // parameters such as 0x13. We emulate this behavior by also
        // encoding all messages that way.
        data[2] = 0xFA;
        data[3] = (self.param >> 8) as u8;
        data[4] = self.param as u8;
        data[5] = self.data[0];
        data[6] = self.data[1];
        data
    }
}

impl TryFrom<&CanDataFrame> for Message {
    type Error = FrameError;
    fn try_from(frame: &CanDataFrame) -> Result<Self, Self::Error> {
        let sender = frame
            .raw_id()
            .try_into()
            .map_err(|_| FrameError::InvalidSender(frame.raw_id()))?;
        let data = frame.data();
        let raw: [u8; 7] = data
            .try_into()
            .map_err(|_| FrameError::InvalidFrameLength(data.len()))?;
        Ok(Self::parse(sender, raw))
    }
}

impl From<Message> for CanDataFrame {
    fn from(msg: Message) -> Self {
        CanDataFrame::new(msg.sender, &msg.compose()).unwrap()
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sender = format!("{}", self.sender);
        let receiver = format!("{}", self.receiver);
        let param = self.param;
        let data = self.data;
        match self.r#type {
            MessageType::Set => {
                write!(f, "{sender:5} -> {receiver:5} SET {param:04x} {data:02x?}",)
            }
            MessageType::Request => {
                write!(f, "{sender:5} -> {receiver:5} REQ {param:04x}")
            }
            MessageType::Response => {
                write!(f, "{receiver:5} <- {sender:5} RSP {param:04x} {data:02x?}",)
            }
            MessageType::Ping => {
                write!(f, "{sender:5} -> {receiver:5} PING {param:04x} {data:02x?}",)
            }
            MessageType::Pong => {
                write!(f, "{receiver:5} <- {sender:5} PONG {param:04x} {data:02x?}",)
            }
            MessageType::Unknown(x) => {
                write!(f, "{sender:5} -> {receiver:5} UNKNOWN({x}) {data:02x?}",)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::can2::device::Device;

    use super::*;

    #[test]
    fn test_software_number_request() {
        let id = 0x69d;
        let data = [0x31, 0x00, 0xfa, 0x01, 0x99, 0x00, 0x00];
        let msg = Message {
            sender: Device::G1,
            receiver: Device::HG1,
            r#type: MessageType::Request,
            param: 0x0199,
            data: [0x00, 0x00],
        };
        assert_eq!(Message::parse(id, data), msg);
        assert_eq!(msg.compose(), data);
    }

    #[test]
    fn test_software_number_response() {
        let id = 0x0180;
        let data = [0xd2, 0x1d, 0xfa, 0x01, 0x99, 0x01, 0xa9];
        let msg = Message {
            sender: Device::HG1,
            receiver: Device::G1,
            r#type: MessageType::Response,
            param: 0x0199,
            data: [0x01, 0xa9],
        };
        assert_eq!(Message::parse(id, data), msg);
        assert_eq!(msg.compose(), data);
    }

    #[test]
    fn test_software_unterindex_request() {
        let id = 0x69d;
        let data = [0x31, 0x00, 0xfa, 0x02, 0x4b, 0x00, 0x00];
        let msg = Message {
            sender: Device::G1,
            receiver: Device::HG1,
            r#type: MessageType::Request,
            param: 0x024b,
            data: [0x00, 0x00],
        };
        assert_eq!(Message::parse(id, data), msg);
        assert_eq!(msg.compose(), data);
    }

    #[test]
    fn test_software_unterindex_response() {
        let id = 0x180;
        let data = [0xd2, 0x1d, 0xfa, 0x02, 0x4b, 0x20, 0x59];
        let msg = Message {
            sender: Device::HG1,
            receiver: (Device::G1),
            r#type: MessageType::Response,
            param: 0x024b,
            data: [0x20, 0x59],
        };
        assert_eq!(Message::parse(id, data), msg);
        assert_eq!(msg.compose(), data);
    }

    #[test]
    fn test_einstell_speichersolltemp_set() {
        let id = 0x69d;
        let data = [0x30, 0x00, 0xfa, 0x00, 0x13, 0x02, 0x30];
        let data_alt = [0x30, 0x00, 0x13, 0x02, 0x30, 0x00, 0x00];
        let msg = Message {
            sender: Device::G1,
            receiver: Device::HG1,
            r#type: MessageType::Set,
            param: 0x13,
            data: [0x02, 0x30],
        };
        assert_eq!(Message::parse(id, data), msg);
        assert_eq!(Message::parse(id, data_alt), msg);
        assert_eq!(msg.compose(), data);
    }

    #[test]
    fn test_unknown_response() {
        let id = 0x0301;
        let data = [0x36, 0x00, 0xfe, 0x01, 0x00, 0x00, 0x00];
        let msg = Message {
            sender: Device::HC2,
            receiver: Device::HG1,
            r#type: MessageType::Ping,
            param: 0xfe,
            data: [0x01, 0x00],
        };
        assert_eq!(Message::parse(id, data), msg);
        assert_eq!(msg.compose(), data);
    }
}
