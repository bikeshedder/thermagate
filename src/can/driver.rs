use std::{
    ops::Deref,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::Serialize;
use socketcan::{tokio::CanSocket, CanDataFrame, CanError, CanFrame};
use thiserror::Error;
use tokio::{
    sync::{
        broadcast,
        mpsc::{self, error::TrySendError},
        watch, Mutex,
    },
    time::sleep,
};
use tracing::warn;

use crate::can::{device::Device, message::MessageType};

use super::{
    message::Message,
    param::{AnyValue, DecodeParam, Param},
};

const CHANNEL_BUFFER: usize = 100;

pub struct CanDriver {
    tx: broadcast::Sender<ReceivedMessage>,
    // FIXME add accessor for this status so that borrowing
    // the driver mutable doesn't render this inaccessable.
    state_rx: watch::Receiver<BusState>,
    send_tx: mpsc::Sender<SendMsg>,
}

struct EventLoop {
    device: String,
    keep_running: AtomicBool,
    tx: broadcast::Sender<ReceivedMessage>,
    bus_state: watch::Sender<BusState>,
    send_rx: Mutex<mpsc::Receiver<SendMsg>>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub enum BusState {
    Connecting,
    Connected,
    Error(String),
    Disconnected,
    Shutdown,
}

enum SendMsg {
    Message(Message),
    Shutdown,
}

#[derive(Clone)]
pub struct ReceivedMessage {
    pub frame: CanDataFrame,
    pub message: Message,
}

impl CanDriver {
    pub fn new(device: &str) -> Self {
        let (tx, _) = broadcast::channel(CHANNEL_BUFFER);
        let (state_tx, state_rx) = watch::channel(BusState::Connecting);
        let (send_tx, send_rx) = mpsc::channel(256);
        let internal = EventLoop {
            device: device.to_owned(),
            keep_running: AtomicBool::new(true),
            tx: tx.clone(),
            bus_state: state_tx,
            send_rx: Mutex::new(send_rx),
        };
        tokio::spawn(internal.start());
        Self {
            tx,
            state_rx,
            send_tx,
        }
    }
    pub fn rx(&self) -> broadcast::Receiver<ReceivedMessage> {
        self.tx.subscribe()
    }
    pub fn state(&self) -> BusState {
        self.state_rx.borrow().clone()
    }
    pub async fn wait_for_state(&self, state: BusState) {
        let _ = self.state_rx.clone().wait_for(|s| *s == state).await;
    }
    pub fn send(&self, msg: Message) -> Result<(), TrySendError<Message>> {
        self.send_tx
            .try_send(SendMsg::Message(msg))
            .map_err(|e| match e {
                TrySendError::Full(SendMsg::Message(m)) => TrySendError::Full(m),
                TrySendError::Closed(SendMsg::Message(m)) => TrySendError::Closed(m),
                _ => unreachable!(),
            })
    }
    pub async fn get<P: DecodeParam>(
        &self,
        dev: Device,
        param: &P,
    ) -> Result<Option<P::Value>, GetError> {
        let msg = self.get_raw(dev, param).await?;
        Ok(param.decode(msg.data))
    }
    pub async fn get_any(
        &self,
        dev: Device,
        param: &dyn Param,
    ) -> Result<Option<AnyValue>, GetError> {
        let msg = self.get_raw(dev, param).await?;
        Ok(param.decode_any(msg.data))
    }
    async fn get_raw(&self, dev: Device, param: &dyn Param) -> Result<ReceivedMessage, GetError> {
        let mut rx = self.rx();
        let req = Message {
            sender: Device::TG,
            receiver: dev,
            param: param.id(),
            r#type: MessageType::Request,
            data: [0, 0],
        };
        self.send(req).map_err(|e| match e {
            TrySendError::Closed(_) => GetError::Shutdown,
            TrySendError::Full(_) => GetError::QueueFull,
        })?;
        // FIXME add timeout
        while let Ok(msg) = rx.recv().await {
            if msg.receiver != Device::TG {
                continue;
            }
            if msg.sender != req.receiver {
                continue;
            }
            if msg.param != req.param {
                continue;
            }
            return Ok(msg);
        }
        unreachable!()
    }
    pub async fn shutdown(&self) {
        let _ = self.send_tx.send(SendMsg::Shutdown).await;
        self.wait_for_state(BusState::Shutdown).await;
    }
}

#[derive(Error, Debug)]
pub enum GetError {
    #[error("Shutdown")]
    Shutdown,
    #[error("Queue full")]
    QueueFull,
}

impl EventLoop {
    pub async fn start(self) {
        while self.keep_running.load(Ordering::Relaxed) {
            match CanSocket::open(&self.device) {
                Ok(socket) => {
                    let (sink, stream) = socket.split();
                    self.bus_state.send_replace(BusState::Connected);
                    tokio::select! {
                        res = self.receiver(stream) => {
                            if let Err(e) = res {
                                warn!("CAN bus receive error: {e:?}");
                            }
                        },
                        res = self.sender(sink) => {
                            match res {
                                Ok(_) => {
                                    // The sender task only returns when the user
                                    // has requested a shutdown and all messages until
                                    // this point have been sent.
                                    break;
                                }
                                Err(e) => {
                                    warn!("CAN bus send error: {e:?}");
                                }
                            }
                        },
                    }
                    self.bus_state.send_replace(BusState::Disconnected);
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
        self.bus_state.send_replace(BusState::Shutdown);
    }
    pub async fn receiver(&self, mut socket: SplitStream<CanSocket>) -> Result<(), CanError> {
        while let Some(Ok(frame)) = socket.next().await {
            let socketcan::CanFrame::Data(data_frame) = frame else {
                continue;
            };
            let Ok(message) = Message::try_from(&data_frame) else {
                continue;
            };
            let _ = self.tx.send(ReceivedMessage {
                frame: data_frame,
                message,
            });
        }
        Ok(())
    }
    pub async fn sender(
        &self,
        mut socket: SplitSink<CanSocket, CanFrame>,
    ) -> Result<(), socketcan::Error> {
        let mut send_rx = self
            .send_rx
            .try_lock()
            .expect("Multiple sender tasks running");
        while let Some(message) = send_rx.recv().await {
            match message {
                SendMsg::Message(message) => {
                    socket
                        .send(CanFrame::Data(CanDataFrame::from(message)))
                        .await?
                }
                SendMsg::Shutdown => break,
            }
        }
        Ok(())
    }
}

impl Deref for ReceivedMessage {
    type Target = Message;
    fn deref(&self) -> &Self::Target {
        &self.message
    }
}
