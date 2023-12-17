use std::fmt;

use socketcan::{CanDataFrame, EmbeddedFrame, Frame};
use thiserror::Error;

use super::device::Addr;

#[derive(Debug)]
pub struct Message {
    pub sender: Addr,
    pub receiver: Receiver,
    pub payload: Payload,
}

#[derive(Debug)]
pub enum Receiver {
    Broadcast,
    Addr(Addr),
}

#[derive(Debug)]
pub enum Payload {
    Request(Param),
    Response(Param),
    Set(Param),
    Unknown(Vec<u8>),
}

#[derive(Debug)]
pub struct Param {
    pub id: u16,
    pub data: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum MessageParseError {
    #[error("Not enough data")]
    NotEnoughData,
}

impl Message {
    pub fn parse(frame: &CanDataFrame) -> Result<Self, MessageParseError> {
        let sender = Addr::from(frame.raw_id() as u16);
        let [r0, r1, p0, p1, p2, v0, v1] = *frame.data() else {
            return Err(MessageParseError::NotEnoughData);
        };
        let receiver = if r1 == 0x79 {
            Receiver::Broadcast
        } else {
            let addr = (((r0 & 0xF0) as u16) * 8) + (r1 & 0x0F) as u16;
            Receiver::Addr(Addr::from(addr))
        };
        let (data_id, data) = if p0 == 0xFA {
            (((p1 as u16) << 8) + p2 as u16, vec![v0, v1])
        } else {
            (p0 as u16, vec![p1, p2, v0, v1])
        };
        let msg = match r0 & 0x0F {
            0x0 => Message {
                sender,
                receiver,
                payload: Payload::Set(Param { id: data_id, data }),
            },
            0x1 => Message {
                sender,
                receiver,
                payload: Payload::Request(Param { id: data_id, data }),
            },
            0x2 => Message {
                sender,
                receiver,
                payload: Payload::Response(Param { id: data_id, data }),
            },
            _ => Message {
                sender,
                receiver,
                payload: Payload::Unknown(frame.data().to_vec()),
            },
        };
        Ok(msg)
    }
    pub fn param_id(&self) -> u16 {
        match &self.payload {
            Payload::Request(r) => r.id,
            Payload::Set(r) => r.id,
            Payload::Response(r) => r.id,
            _ => 0, // XXX
        }
    }
}

impl fmt::Display for Receiver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Broadcast => {
                write!(f, "*")
            }
            Self::Addr(addr) => {
                write!(f, "{}", addr)
            }
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sender = format!("{}", self.sender);
        let receiver = format!("{}", self.receiver);
        match &self.payload {
            Payload::Set(param) => {
                write!(
                    f,
                    "{:5} -> {:5} {:04x} = {:02x?}",
                    sender, receiver, param.id, param.data
                )
            }
            Payload::Request(param) => {
                write!(f, "{:5} -> {:5} {:04x} ?", sender, receiver, param.id)
            }
            Payload::Response(param) => {
                write!(
                    f,
                    "{:5} <- {:5} {:04x} ! {:02x?}",
                    receiver, sender, param.id, param.data
                )
            }
            Payload::Unknown(data) => {
                write!(f, "{:5} -> {:5} ??? {:02x?}", sender, receiver, data)
            }
        }
    }
}
