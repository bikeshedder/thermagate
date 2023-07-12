use std::{sync::Arc, time::Duration};

use futures_util::StreamExt;
use socketcan::{tokio::CanSocket, CanError, EmbeddedFrame, Frame};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::sleep,
};
use tracing::{debug, warn};

use crate::model::{Address, FloatParameter, Op, ParameterType, State};

static CHANNEL_BUFFER: usize = 100;

pub struct BusDriver {
    rx: Receiver<BusFrame>,
}

#[derive(Debug)]
pub struct BusFrame {
    pub op: Op,
    pub device: String,
    pub parameter: String,
    pub value: Option<Value>,
}

#[derive(Debug)]
pub enum Value {
    Float(f32),
    Invalid(u16),
}

impl BusDriver {
    pub fn new(device: &str, state: Arc<State>) -> Self {
        let (tx, rx) = channel(CHANNEL_BUFFER);
        let internal = Internal {
            device: device.to_owned(),
            state: state,
            keep_running: true,
            tx,
        };
        tokio::spawn(internal.start());
        Self { rx }
    }
    pub async fn recv(&mut self) -> Option<BusFrame> {
        self.rx.recv().await
    }
}

struct Internal {
    device: String,
    keep_running: bool,
    state: Arc<State>,
    tx: Sender<BusFrame>,
}

impl Internal {
    pub async fn start(self) {
        while self.keep_running {
            match CanSocket::open(&self.device) {
                Ok(socket) => {
                    if let Err(e) = self.receiver(socket).await {
                        warn!("CAN bus error: {e:?}");
                    }
                }
                Err(e) => {
                    if self.keep_running {
                        warn!("Could not open CAN bus device {}: {}", self.device, e);
                        sleep(Duration::from_secs(5)).await;
                    } else {
                        return;
                    }
                }
            }
        }
    }
    pub async fn receiver(&self, mut socket: CanSocket) -> Result<(), CanError> {
        while let Some(Ok(frame)) = socket.next().await {
            if let socketcan::CanFrame::Data(data_frame) = frame {
                match data_frame.data() {
                    [a0, a1, a2, p0, p1, v0, v1] => {
                        let a = Address(frame.id_word(), *a0, *a1, *a2);
                        let Some((d_name, op)) = self.state.device_by_address.get(&a) else {
                            debug!("Unsupported address: {:?}", a);
                            continue;
                        };
                        let p = ((*p0 as u16) << 8) + (*p1 as u16);
                        let Some(p_name) = self.state.parameter_by_address.get(&p) else {
                            debug!("Unknown parameter: {}", p);
                            continue;
                        };
                        let v = ((*v0 as u16) << 8) + (*v1 as u16);
                        let v = v as i16;
                        let param = &self.state.parameters[p_name];
                        match param.r#type {
                            ParameterType::Float(FloatParameter { factor, .. }) => {
                                let value = if v == i16::MIN {
                                    None
                                } else {
                                    Some(Value::Float(v as f32 / factor))
                                };
                                if self
                                    .tx
                                    .try_send(BusFrame {
                                        op: op.to_owned(),
                                        device: d_name.to_owned(),
                                        parameter: p_name.to_owned(),
                                        value,
                                    })
                                    .is_err()
                                {
                                    debug!("CAN bus driver queue full")
                                }
                            }
                            _ => {}
                        }
                    }
                    x => {
                        debug!("Unsupported CAN bus frame: {:?}", x)
                    }
                }
            }
        }
        Ok(())
    }
}
