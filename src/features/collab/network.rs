use crate::features::collab::crypto::RoomId;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::watch;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

/// Forbindelsens livscyklustilstand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

impl std::fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disconnected => write!(f, "Afbrudt"),
            Self::Connecting => write!(f, "Forbinder..."),
            Self::Connected => write!(f, "Forbundet"),
            Self::Reconnecting => write!(f, "Genforbinder..."),
        }
    }
}

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::LazyLock;

static NEXT_SUB_ID: AtomicU64 = AtomicU64::new(1);
static COLLAB_REGISTRY: LazyLock<
    std::sync::Mutex<HashMap<u64, Arc<tokio::sync::Mutex<UnboundedReceiver<CollabNetworkEvent>>>>>,
> = LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// Henter næste netværkshændelse for et givet abonnements-ID.
pub async fn next_registered_collab_event(sub_id: u64) -> Option<CollabNetworkEvent> {
    let receiver = {
        let guard = COLLAB_REGISTRY.lock().ok()?;
        guard.get(&sub_id).cloned()?
    };
    let mut rx = receiver.lock().await;
    rx.recv().await
}

/// Netværkshændelser fra WebSocket-kanalen til Iced/UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollabNetworkEvent {
    StatusChanged(ConnectionStatus),
    MessageReceived(Vec<u8>),
    PresenceUpdated(usize),
    HostEndedSession,
    Error(String),
}

/// Konstruerer en gyldig WebSocket-URL baseret på relay URL og rum-identifikator.
/// Invariant: URL'en indeholder KUN server-adresse og room-id - ALDRIG krypteringsnøglen!
pub fn build_relay_ws_url(relay_url: &str, room_id: &RoomId) -> Result<Url, String> {
    let trimmed = relay_url.trim();
    if trimmed.is_empty() {
        return Err("Relay-URL kan ikke være tom".to_string());
    }

    let mut url_str = trimmed.to_string();
    if url_str.starts_with("http://") {
        url_str = format!("ws://{}", &url_str["http://".len()..]);
    } else if url_str.starts_with("https://") {
        url_str = format!("wss://{}", &url_str["https://".len()..]);
    } else if !url_str.starts_with("ws://") && !url_str.starts_with("wss://") {
        url_str = format!("ws://{}", url_str);
    }

    let mut parsed = Url::parse(&url_str).map_err(|e| format!("Ugyldig relay-URL: {e}"))?;

    // Sørg for at stien har /ws som endepunkt
    let current_path = parsed.path().trim_end_matches('/');
    if !current_path.ends_with("/ws") {
        if current_path.is_empty() {
            parsed.set_path("/ws");
        } else {
            parsed.set_path(&format!("{}/ws", current_path));
        }
    }

    // Sæt kun ?room=<room_id> query parameter
    parsed.set_query(Some(&format!("room={}", room_id.as_str())));

    Ok(parsed)
}

/// Tovejs netværkskanal til asynkron kommunikation med relay-serveren.
#[derive(Clone)]
pub struct CollabChannel {
    sub_id: u64,
    outbound_tx: UnboundedSender<Vec<u8>>,
    cancel_tx: watch::Sender<bool>,
    status: Arc<RwLock<ConnectionStatus>>,
}

impl CollabChannel {
    /// Opretter forbindelse til relay-serveren for et givet rum og registrerer modtageren i det globale registry.
    pub fn connect_registered(relay_url: &str, room_id: &RoomId) -> Self {
        let (channel, rx) = Self::connect(relay_url, room_id);
        if let Ok(mut guard) = COLLAB_REGISTRY.lock() {
            guard.insert(channel.sub_id, Arc::new(tokio::sync::Mutex::new(rx)));
        }
        channel
    }

    /// Opretter forbindelse til relay-serveren for et givet rum.
    /// Returnerer kanal-håndtaget og en modtager for indgående netværkshændelser.
    pub fn connect(
        relay_url: &str,
        room_id: &RoomId,
    ) -> (Self, UnboundedReceiver<CollabNetworkEvent>) {
        let sub_id = NEXT_SUB_ID.fetch_add(1, Ordering::Relaxed);
        let (outbound_tx, mut outbound_rx) = unbounded_channel::<Vec<u8>>();
        let (event_tx, event_rx) = unbounded_channel::<CollabNetworkEvent>();
        let (cancel_tx, mut cancel_rx) = watch::channel(false);
        let status = Arc::new(RwLock::new(ConnectionStatus::Connecting));

        let ws_url_res = build_relay_ws_url(relay_url, room_id);
        let status_clone = Arc::clone(&status);

        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(async move {
            let ws_url = match ws_url_res {
                Ok(url) => url.to_string(),
                Err(err) => {
                    let _ = event_tx.send(CollabNetworkEvent::Error(err));
                    let _ = event_tx.send(CollabNetworkEvent::StatusChanged(
                        ConnectionStatus::Disconnected,
                    ));
                    if let Ok(mut s) = status_clone.write() {
                        *s = ConnectionStatus::Disconnected;
                    }
                    return;
                }
            };

            let mut backoff = Duration::from_millis(500);
            let max_backoff = Duration::from_secs(10);
            let mut is_reconnect = false;

            loop {
                if *cancel_rx.borrow() {
                    break;
                }

                let current_phase = if is_reconnect {
                    ConnectionStatus::Reconnecting
                } else {
                    ConnectionStatus::Connecting
                };

                if let Ok(mut s) = status_clone.write() {
                    *s = current_phase;
                }
                let _ = event_tx.send(CollabNetworkEvent::StatusChanged(current_phase));

                // Forsøg at etablere WebSocket-forbindelse med 5s timeout
                let connect_attempt =
                    tokio::time::timeout(Duration::from_secs(5), connect_async(&ws_url)).await;

                match connect_attempt {
                    Ok(Ok((ws_stream, _))) => {
                        // Nulstil backoff ved succesfuld forbindelse
                        backoff = Duration::from_millis(500);
                        is_reconnect = true;

                        if let Ok(mut s) = status_clone.write() {
                            *s = ConnectionStatus::Connected;
                        }
                        let _ = event_tx.send(CollabNetworkEvent::StatusChanged(
                            ConnectionStatus::Connected,
                        ));

                        let (mut ws_sink, mut ws_stream) = ws_stream.split();

                        // Indre loop: pump data frem og tilbage
                        loop {
                            tokio::select! {
                                _ = cancel_rx.changed() => {
                                    if *cancel_rx.borrow() {
                                        while let Ok(data) = outbound_rx.try_recv() {
                                            let _ = ws_sink.send(Message::Binary(bytes::Bytes::from(data))).await;
                                        }
                                        let _ = ws_sink.send(Message::Close(None)).await;
                                        break;
                                    }
                                }

                                outbound = outbound_rx.recv() => {
                                    match outbound {
                                        Some(data) => {
                                            if let Err(e) = ws_sink.send(Message::Binary(bytes::Bytes::from(data))).await {
                                                let _ = event_tx.send(CollabNetworkEvent::Error(format!("Fejl ved afsendelse: {e}")));
                                                break;
                                            }
                                        }
                                        None => {
                                            // Outbound sender droppet - afslutter forbindelsen
                                            break;
                                        }
                                    }
                                }
                                inbound = ws_stream.next() => {
                                    match inbound {
                                        Some(Ok(msg)) => {
                                            match msg {
                                                Message::Binary(bytes) => {
                                                    match bytes.first() {
                                                        Some(&0x03) if bytes.len() >= 5 => {
                                                            let count = u32::from_be_bytes(bytes[1..5].try_into().unwrap_or([0; 4]));
                                                            let _ = event_tx.send(CollabNetworkEvent::PresenceUpdated(count as usize));
                                                        }
                                                        Some(&0x04) => {
                                                            let _ = event_tx.send(CollabNetworkEvent::HostEndedSession);
                                                        }
                                                        Some(&0x01) | Some(&0x02) => {
                                                            // Strip framing byte so message contains ciphertext directly
                                                            let _ = event_tx.send(CollabNetworkEvent::MessageReceived(bytes[1..].to_vec()));
                                                        }
                                                        _ => {
                                                            // Legacy / unframed
                                                            let _ = event_tx.send(CollabNetworkEvent::MessageReceived(bytes.to_vec()));
                                                        }
                                                    }
                                                }
                                                Message::Ping(payload) => {
                                                    let _ = ws_sink.send(Message::Pong(payload)).await;
                                                }
                                                Message::Close(_) => {
                                                    break;
                                                }
                                                _ => {}
                                            }
                                        }
                                        Some(Err(e)) => {
                                            let _ = event_tx.send(CollabNetworkEvent::Error(format!("Netværksfejl i modtagelse: {e}")));
                                            break;
                                        }
                                        None => {
                                            // Socket lukket af modparten
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if *cancel_rx.borrow() {
                            break;
                        }
                    }
                    Ok(Err(e)) => {
                        let _ = event_tx.send(CollabNetworkEvent::Error(format!(
                            "Kunne ikke forbinde til relay: {e}"
                        )));
                        is_reconnect = true;
                    }
                    Err(_) => {
                        let _ = event_tx.send(CollabNetworkEvent::Error(
                            "Forbindelse fik timeout (5s)".to_string(),
                        ));
                        is_reconnect = true;
                    }
                }

                if *cancel_rx.borrow() {
                    break;
                }

                // Exponential backoff pause før næste forsøg
                tokio::select! {
                    _ = cancel_rx.changed() => {
                        if *cancel_rx.borrow() {
                            break;
                        }
                    }
                    _ = tokio::time::sleep(backoff) => {
                        backoff = (backoff * 2).min(max_backoff);
                    }
                }
            }

            if let Ok(mut s) = status_clone.write() {
                *s = ConnectionStatus::Disconnected;
            }
            let _ = event_tx.send(CollabNetworkEvent::StatusChanged(
                ConnectionStatus::Disconnected,
            ));
            });
        }

        (
            Self {
                sub_id,
                outbound_tx,
                cancel_tx,
                status,
            },
            event_rx,
        )
    }

    pub fn sub_id(&self) -> u64 {
        self.sub_id
    }

    /// Sender snapshot med 0x01 frame type.
    pub fn send_snapshot(&self, data: Vec<u8>) -> Result<(), String> {
        let mut framed = Vec::with_capacity(1 + data.len());
        framed.push(0x01);
        framed.extend_from_slice(&data);
        self.send(framed)
    }

    /// Sender mutation med 0x02 frame type.
    pub fn send_mutation(&self, data: Vec<u8>) -> Result<(), String> {
        let mut framed = Vec::with_capacity(1 + data.len());
        framed.push(0x02);
        framed.extend_from_slice(&data);
        self.send(framed)
    }

    /// Sender besked om at værten har forladt sessionen (0x04 frame type).
    pub fn send_host_left(&self) -> Result<(), String> {
        self.send(vec![0x04])
    }

    /// Sender rå bytes over netværkskanalen.
    pub fn send(&self, data: Vec<u8>) -> Result<(), String> {
        self.outbound_tx
            .send(data)
            .map_err(|_| "Netværkskanalens afsender er lukket".to_string())
    }

    /// Afbryder netværksforbindelsen rent.
    pub fn disconnect(&self) {
        let _ = self.cancel_tx.send(true);
        if let Ok(mut map) = COLLAB_REGISTRY.lock() {
            map.remove(&self.sub_id);
        }
    }

    /// Henter den aktuelle forbindelsestilstand.
    pub fn status(&self) -> ConnectionStatus {
        self.status
            .read()
            .map(|s| *s)
            .unwrap_or(ConnectionStatus::Disconnected)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::collab::crypto::{decrypt, encrypt, CollabKey};
    use edge_relay::{create_app, AppState, RelayConfig};
    use tokio::net::TcpListener;

    async fn spawn_ephemeral_relay() -> String {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Fejl ved binding af test TCP port");
        let addr = listener
            .local_addr()
            .expect("Fejl ved læsning af lokal adresse");
        let state = AppState::new(RelayConfig::default());
        let app = create_app(state);

        tokio::spawn(async move {
            axum::serve(listener, app)
                .await
                .expect("Test relay server stoppede uventet");
        });

        format!("ws://{}", addr)
    }

    #[test]
    fn test_build_relay_ws_url_formats() {
        let room = RoomId::new("KU-4821");

        let u1 = build_relay_ws_url("http://127.0.0.1:3000", &room).unwrap();
        assert_eq!(u1.as_str(), "ws://127.0.0.1:3000/ws?room=KU-4821");

        let u2 = build_relay_ws_url("https://edge.relay.internal/ws", &room).unwrap();
        assert_eq!(u2.as_str(), "wss://edge.relay.internal/ws?room=KU-4821");

        let u3 = build_relay_ws_url("127.0.0.1:8080", &room).unwrap();
        assert_eq!(u3.as_str(), "ws://127.0.0.1:8080/ws?room=KU-4821");

        // Invariant: Nøglen må ALDRIG fremgå af URL
        assert!(!u1.as_str().contains("key"));
        assert!(!u2.as_str().contains("key"));
        assert!(!u3.as_str().contains("key"));
    }

    #[tokio::test]
    async fn test_network_channel_e2e_communication() {
        let relay_addr = spawn_ephemeral_relay().await;
        let room = RoomId::new("TEST-COLLAB-1");
        let key = CollabKey::generate();

        // 1. Opret klient 1 og klient 2
        let (channel1, mut rx1) = CollabChannel::connect(&relay_addr, &room);
        let (channel2, mut rx2) = CollabChannel::connect(&relay_addr, &room);

        // Vent på at begge klienter når status Connected
        let mut c1_connected = false;
        let mut c2_connected = false;

        for _ in 0..10 {
            if let Ok(event) = tokio::time::timeout(Duration::from_millis(500), rx1.recv()).await {
                if event
                    == Some(CollabNetworkEvent::StatusChanged(
                        ConnectionStatus::Connected,
                    ))
                {
                    c1_connected = true;
                    break;
                }
            }
        }
        assert!(c1_connected, "Klient 1 nåede ikke Connected");

        for _ in 0..10 {
            if let Ok(event) = tokio::time::timeout(Duration::from_millis(500), rx2.recv()).await {
                if event
                    == Some(CollabNetworkEvent::StatusChanged(
                        ConnectionStatus::Connected,
                    ))
                {
                    c2_connected = true;
                    break;
                }
            }
        }
        assert!(c2_connected, "Klient 2 nåede ikke Connected");

        // 2. Klient 1 krypterer og sender payload med mutation framing
        let original_data = b"FDA E2EE Kollaborering Test Besked";
        let encrypted_payload = encrypt(&key, original_data).expect("Kryptering fejlede");
        channel1
            .send_mutation(encrypted_payload.clone())
            .expect("Afsendelse fejlede");

        // 3. Klient 2 modtager beskeden (kan have modtaget PresenceUpdated først)
        let mut received_payload = None;
        for _ in 0..5 {
            if let Ok(Some(event)) =
                tokio::time::timeout(Duration::from_millis(500), rx2.recv()).await
            {
                if let CollabNetworkEvent::MessageReceived(bytes) = event {
                    received_payload = Some(bytes);
                    break;
                }
            }
        }
        let bytes = received_payload.expect("Klient 2 modtog ikke MessageReceived");
        assert_eq!(bytes, encrypted_payload);
        let decrypted = decrypt(&key, &bytes).expect("Dekryptering hos klient 2 fejlede");
        assert_eq!(decrypted, original_data);

        // 4. Invariant: Klient 1 må IKKE modtage sit eget ekko
        let echo = tokio::time::timeout(Duration::from_millis(150), rx1.recv()).await;
        if let Ok(Some(CollabNetworkEvent::MessageReceived(bytes))) = echo {
            panic!("Klient 1 modtog sit eget ekko: {:?}", bytes);
        }


        // 5. Afbrydelse
        channel1.disconnect();
        channel2.disconnect();
    }

    #[tokio::test]
    async fn test_network_reconnect_status_on_unavailable_server() {
        let room = RoomId::new("UNAVAILABLE-ROOM");
        // Forbind til en usandsynlig lokal port for at teste reconnect-håndtering
        let (channel, mut rx) = CollabChannel::connect("ws://127.0.0.1:59999", &room);

        let first = tokio::time::timeout(Duration::from_millis(500), rx.recv())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            first,
            CollabNetworkEvent::StatusChanged(ConnectionStatus::Connecting)
        );

        // Næste hændelse bør være enten en fejl eller overgang til Reconnecting
        let mut got_reconnecting = false;
        for _ in 0..5 {
            if let Ok(Some(event)) =
                tokio::time::timeout(Duration::from_millis(1000), rx.recv()).await
            {
                if event == CollabNetworkEvent::StatusChanged(ConnectionStatus::Reconnecting) {
                    got_reconnecting = true;
                    break;
                }
            }
        }
        assert!(got_reconnecting, "Forventede overgang til Reconnecting");
        channel.disconnect();
    }
}
