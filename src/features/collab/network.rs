use crate::features::collab::crypto::RoomId;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollabNetworkEvent {
    StatusChanged(ConnectionStatus),
    MessageReceived(Vec<u8>),
    Error(String),
}

#[derive(Clone)]
pub struct CollabChannel {
    _private: (),
}

impl CollabChannel {
    pub fn connect(
        _relay_url: &str,
        _room_id: &RoomId,
    ) -> (Self, UnboundedReceiver<CollabNetworkEvent>) {
        todo!("RED: not implemented yet")
    }

    pub fn send(&self, _data: Vec<u8>) -> Result<(), String> {
        todo!("RED: not implemented yet")
    }

    pub fn disconnect(&self) {
        todo!("RED: not implemented yet")
    }

    pub fn status(&self) -> ConnectionStatus {
        todo!("RED: not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_connect_and_exchange() {
        let room = RoomId::new("TEST-ROOM-1");
        let (_channel, mut rx) = CollabChannel::connect("ws://127.0.0.1:9999", &room);
        let event = rx.recv().await;
        assert!(event.is_some());
    }
}
