use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::{broadcast, Mutex, RwLock};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(Clone, Debug)]
pub struct RelayConfig {
    pub cleanup_timeout: Duration,
}

impl Default for RelayConfig {
    fn default() -> Self {
        Self {
            cleanup_timeout: Duration::from_secs(60),
        }
    }
}

pub struct Room {
    pub tx: broadcast::Sender<(u64, Bytes)>,
    pub last_snapshot: Option<Bytes>,
    pub client_count: usize,
}

impl Room {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self {
            tx,
            last_snapshot: None,
            client_count: 0,
        }
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub rooms: Arc<RwLock<HashMap<String, Arc<Mutex<Room>>>>>,
    pub config: RelayConfig,
    next_client_id: Arc<AtomicU64>,
}

impl AppState {
    pub fn new(config: RelayConfig) -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            config,
            next_client_id: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn room_count(&self) -> usize {
        // Fast synchronous check using try_read, or block_in_place if needed
        // For testing, let's provide synchronous or async
        if let Ok(guard) = self.rooms.try_read() {
            guard.len()
        } else {
            0
        }
    }

    pub async fn get_or_create_room(&self, room_id: &str) -> Arc<Mutex<Room>> {
        let mut rooms = self.rooms.write().await;
        rooms
            .entry(room_id.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(Room::new())))
            .clone()
    }
}

#[derive(Deserialize)]
struct WsQuery {
    room: Option<String>,
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let room_id = match query.room {
        Some(ref r) if !r.trim().is_empty() => r.trim().to_string(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                "Missing 'room' query parameter",
            )
                .into_response();
        }
    };

    ws.on_upgrade(move |socket| handle_socket(socket, state, room_id))
}

async fn handle_socket(socket: WebSocket, state: AppState, room_id: String) {
    let client_id = state.next_client_id.fetch_add(1, Ordering::Relaxed);
    let room = state.get_or_create_room(&room_id).await;

    let (broadcast_rx, initial_snapshot) = {
        let mut r = room.lock().await;
        r.client_count += 1;
        (r.tx.subscribe(), r.last_snapshot.clone())
    };

    let (mut ws_sink, mut ws_stream) = socket.split();
    let (outbound_tx, mut outbound_rx) = tokio::sync::mpsc::channel::<Message>(256);

    // Outgoing writer task
    let writer_handle = tokio::spawn(async move {
        while let Some(msg) = outbound_rx.recv().await {
            if ws_sink.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Deliver last_snapshot immediately to the newly connected participant
    if let Some(snap) = initial_snapshot {
        let _ = outbound_tx.send(Message::Binary(snap)).await;
    }

    // Broadcast reader task: forward frames from peers to this client
    let outbound_tx_clone = outbound_tx.clone();
    let mut bcast_rx = broadcast_rx;
    let bcast_handle = tokio::spawn(async move {
        loop {
            match bcast_rx.recv().await {
                Ok((sender_id, data)) => {
                    // Loopback prevention: do not echo back to the sender
                    if sender_id != client_id
                        && outbound_tx_clone.send(Message::Binary(data)).await.is_err()
                    {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    tracing::warn!(client_id, lagged = n, "Client lagged behind in broadcast stream");
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    // Incoming WebSocket message processing
    while let Some(result) = ws_stream.next().await {
        let msg = match result {
            Ok(m) => m,
            Err(_) => break,
        };

        match msg {
            Message::Binary(bytes) => {
                let mut r = room.lock().await;
                // Buffer snapshot if empty or explicitly marked with 0x01 prefix
                if r.last_snapshot.is_none() || bytes.first() == Some(&0x01) {
                    r.last_snapshot = Some(bytes.clone());
                }
                let _ = r.tx.send((client_id, bytes));
            }
            Message::Ping(payload) => {
                let _ = outbound_tx.send(Message::Pong(payload)).await;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Client disconnected: abort background forwarders
    bcast_handle.abort();
    writer_handle.abort();

    // Decrement client count and schedule cleanup if room is empty
    let is_empty = {
        let mut r = room.lock().await;
        r.client_count = r.client_count.saturating_sub(1);
        r.client_count == 0
    };

    if is_empty {
        let rooms = state.rooms.clone();
        let timeout = state.config.cleanup_timeout;
        let room_id_clone = room_id.clone();
        let room_clone = room.clone();

        tokio::spawn(async move {
            tokio::time::sleep(timeout).await;
            let should_prune = {
                let r = room_clone.lock().await;
                r.client_count == 0
            };

            if should_prune {
                let mut map = rooms.write().await;
                let still_zero = match map.get(&room_id_clone) {
                    Some(r) => r.lock().await.client_count == 0,
                    None => false,
                };
                if still_zero {
                    map.remove(&room_id_clone);
                    tracing::info!("Room '{}' pruned from RAM after inactivity timeout", room_id_clone);
                }
            }
        });
    }
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
