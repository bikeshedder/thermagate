use std::{
    ops::Deref,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use futures_util::StreamExt;
use serde::Serialize;
use socketcan::{tokio::CanSocket, CanDataFrame, CanError};
use tokio::{
    sync::{broadcast, watch},
    time::sleep,
};
use tracing::warn;

use super::message::Message;

const CHANNEL_BUFFER: usize = 100;

pub struct CanDriver {
    tx: broadcast::Sender<ReceivedMessage>,
    // FIXME add accessor for this status so that borrowing
    // the driver mutable doesn't render this inaccessable.
    state_rx: watch::Receiver<BusState>,
}

struct EventLoop {
    device: String,
    keep_running: AtomicBool,
    tx: broadcast::Sender<ReceivedMessage>,
    bus_state: watch::Sender<BusState>,
}

#[derive(Debug, Serialize, Clone)]
pub enum BusState {
    Connecting,
    Connected,
    Error(String),
    Disconnected,
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
        let internal = EventLoop {
            device: device.to_owned(),
            keep_running: AtomicBool::new(true),
            tx: tx.clone(),
            bus_state: state_tx,
        };
        tokio::spawn(internal.start());
        Self { tx, state_rx }
    }
    pub fn rx(&self) -> broadcast::Receiver<ReceivedMessage> {
        self.tx.subscribe()
    }
    pub fn status(&self) -> BusState {
        self.state_rx.borrow().clone()
    }
}

impl EventLoop {
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
}

impl Deref for ReceivedMessage {
    type Target = Message;
    fn deref(&self) -> &Self::Target {
        &self.message
    }
}
