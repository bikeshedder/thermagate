use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use futures_util::StreamExt;
use serde::Serialize;
use socketcan::{tokio::CanSocket, CanError, EmbeddedFrame, Frame};
use time::format_description;
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        watch,
    },
    time::sleep,
};
use tracing::{debug, warn};

use crate::model::{
    Address, BoolParameter, EnumParameter, FloatParameter, IntParameter, Op, ParameterType, State,
    TimeRangeParameter,
};

static CHANNEL_BUFFER: usize = 100;

pub struct BusDriver {
    rx: Receiver<BusFrame>,
    // FIXME add accessor for this status so that borrowing
    // the driver mutable doesn't render this inaccessable.
    status: Arc<BusStatusInternal>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BusFrame {
    pub op: Op,
    pub device: String,
    pub parameter: String,
    pub value: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Value {
    Bool(bool),
    Int(i16),
    Float(f32),
    Enum(u16),
    TimeRange(String, String),
    Invalid(u16),
}

pub struct BusStatusInternal {
    state: watch::Receiver<BusState>,
}

#[derive(Debug, Serialize)]
pub struct BusStatus {
    state: BusState,
}

#[derive(Debug, Serialize, Clone)]
pub enum BusState {
    Connecting,
    Connected,
    Error(String),
    Disconnected,
}

impl BusDriver {
    pub fn new(device: &str, state: Arc<State>) -> Self {
        let (tx, rx) = channel(CHANNEL_BUFFER);
        let (state_tx, state_rx) = watch::channel(BusState::Connecting);
        let status = Arc::new(BusStatusInternal { state: state_rx });
        let internal = Internal {
            device: device.to_owned(),
            state,
            keep_running: AtomicBool::new(true),
            tx,
            bus_state: state_tx,
        };
        tokio::spawn(internal.start());
        Self { rx, status }
    }
    pub async fn recv(&mut self) -> Option<BusFrame> {
        self.rx.recv().await
    }
    pub fn status(&self) -> BusStatus {
        BusStatus {
            state: self.status.state.borrow().clone(),
        }
    }
}

struct Internal {
    device: String,
    state: Arc<State>,
    keep_running: AtomicBool,
    tx: Sender<BusFrame>,
    bus_state: watch::Sender<BusState>,
}

impl Internal {
    pub async fn start(self) {
        while self.keep_running.load(Ordering::Relaxed) {
            match CanSocket::open(&self.device) {
                Ok(socket) => {
                    self.bus_state.send_replace(BusState::Connected);
                    if let Err(e) = self.receiver(socket).await {
                        warn!("CAN bus error: {e:?}");
                    }
                }
                Err(e) => {
                    if self.keep_running.load(Ordering::Relaxed) {
                        let message =
                            format!("Could not open CAN bus device {}: {}", self.device, e);
                        warn!("{message}");
                        self.bus_state.send_replace(BusState::Error(message));
                        sleep(Duration::from_secs(5)).await;
                    } else {
                        return;
                    }
                }
            }
        }
        self.bus_state.send_replace(BusState::Disconnected);
    }
    pub async fn receiver(&self, mut socket: CanSocket) -> Result<(), CanError> {
        while let Some(Ok(frame)) = socket.next().await {
            if let socketcan::CanFrame::Data(data_frame) = frame {
                match data_frame.data() {
                    [a0, a1, a2, p0, p1, v0, v1] => {
                        let a = Address(frame.id_word(), *a0, *a1, *a2);
                        let Some((d_name, op)) = self.state.device_by_address.get(&a) else {
                            //debug!("Unsupported address: {:?}", a);
                            continue;
                        };
                        let p = ((*p0 as u16) << 8) + (*p1 as u16);
                        let Some(p_name) = self.state.parameter_by_address.get(&p) else {
                            debug!("Unknown parameter: {}", p);
                            continue;
                        };
                        let param = &self.state.parameters[p_name];
                        // Big endian and little endian is actually mixed up.
                        let v = if param.big_endian {
                            ((*v1 as u16) << 8) + (*v0 as u16)
                        } else {
                            ((*v0 as u16) << 8) + (*v1 as u16)
                        };
                        let value = if v == 32768 {
                            None
                        } else {
                            match &param.r#type {
                                ParameterType::Bool(BoolParameter { .. }) => {
                                    Some(Value::Bool(v != 0))
                                }
                                ParameterType::Int(IntParameter { .. }) => {
                                    Some(Value::Int(v as i16))
                                }
                                ParameterType::Float(FloatParameter { factor, .. }) => {
                                    let v = v as i16;
                                    if v == i16::MIN {
                                        None
                                    } else {
                                        Some(Value::Float(v as f32 / factor))
                                    }
                                }
                                ParameterType::Enum(EnumParameter { .. }) => Some(Value::Enum(v)),
                                ParameterType::TimeRange(TimeRangeParameter { .. }) => {
                                    if *v0 == 128 {
                                        None
                                    } else {
                                        let format =
                                            format_description::parse("[hour]:[minute]").unwrap();
                                        let start = (time::Time::MIDNIGHT
                                            + Duration::from_secs(60 * 15 * (*v0 as u64)))
                                        .format(&format)
                                        .unwrap();
                                        // FIXME what to do when v1 is 96
                                        // The original G1 code mapped that to "24:00"
                                        let end = if *v1 == 96 {
                                            String::from("24:00")
                                        } else {
                                            (time::Time::MIDNIGHT
                                                + Duration::from_secs(60 * 15 * (*v1 as u64)))
                                            .format(&format)
                                            .unwrap()
                                        };
                                        Some(Value::TimeRange(start, end))
                                    }
                                }
                            }
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
                    x => {
                        debug!("Unsupported CAN bus frame: {:?}", x)
                    }
                }
            }
        }
        Ok(())
    }
}
