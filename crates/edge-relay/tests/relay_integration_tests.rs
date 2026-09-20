use edge_relay::{create_app, AppState, RelayConfig};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

async fn spawn_test_server(config: RelayConfig) -> (String, AppState) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind");
    let addr = listener.local_addr().expect("Failed to get local addr");
    let state = AppState::new(config);
    let app = create_app(state.clone());

    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("Server failed");
    });

    (addr.to_string(), state)
}

#[tokio::test]
async fn test_health_endpoint() {
    let (addr, _) = spawn_test_server(RelayConfig::default()).await;

    let mut stream = tokio::net::TcpStream::connect(&addr).await.expect("Failed to connect TCP");
    stream
        .write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .expect("Failed to write HTTP request");

    let mut resp = String::new();
    stream.read_to_string(&mut resp).await.expect("Failed to read response");

    assert!(
        resp.starts_with("HTTP/1.1 200 OK"),
        "Health check did not return 200 OK: {}",
        resp
    );
    assert!(resp.contains("OK"), "Health check body missing OK: {}", resp);
}

#[tokio::test]
async fn test_ws_handshake_and_broadcast() {
    let (addr, _) = spawn_test_server(RelayConfig::default()).await;
    let url = format!("ws://{}/ws?room=room-broadcast", addr);

    let (mut client1, _) = connect_async(&url).await.expect("Client 1 handshake failed");
    let (mut client2, _) = connect_async(&url).await.expect("Client 2 handshake failed");

    let payload = vec![10u8, 20, 30, 40];
    client1
        .send(Message::Binary(payload.clone().into()))
        .await
        .expect("Client 1 send failed");

    // Client 2 should receive the broadcasted message
    let received = tokio::time::timeout(Duration::from_millis(500), client2.next())
        .await
        .expect("Client 2 timeout waiting for message")
        .expect("Stream ended")
        .expect("Client 2 read error");

    match received {
        Message::Binary(data) => assert_eq!(data.as_ref(), payload.as_slice()),
        other => panic!("Expected binary message, got {:?}", other),
    }

    // Client 1 should NOT receive its own message back (loopback prevention)
    let echo = tokio::time::timeout(Duration::from_millis(100), client1.next()).await;
    assert!(echo.is_err(), "Client 1 received its own echo frame");
}

#[tokio::test]
async fn test_room_isolation() {
    let (addr, _) = spawn_test_server(RelayConfig::default()).await;
    let url_a = format!("ws://{}/ws?room=room-alpha", addr);
    let url_b = format!("ws://{}/ws?room=room-beta", addr);

    let (mut client_a, _) = connect_async(&url_a).await.expect("Client A connect failed");
    let (mut client_b, _) = connect_async(&url_b).await.expect("Client B connect failed");

    client_a
        .send(Message::Binary(vec![99, 88].into()))
        .await
        .expect("Client A send failed");

    // Client B in another room should NOT receive anything
    let result = tokio::time::timeout(Duration::from_millis(150), client_b.next()).await;
    assert!(
        result.is_err(),
        "Client B in room-beta received message from room-alpha"
    );
}

#[tokio::test]
async fn test_last_snapshot_delivery_to_late_joiner() {
    let (addr, _) = spawn_test_server(RelayConfig::default()).await;
    let url = format!("ws://{}/ws?room=room-snapshot", addr);

    let (mut host, _) = connect_async(&url).await.expect("Host connect failed");
    let snapshot_bytes = vec![0x01, 0xDE, 0xAD, 0xBE, 0xEF];

    host.send(Message::Binary(snapshot_bytes.clone().into()))
        .await
        .expect("Host snapshot send failed");

    // Small delay to ensure relay has buffered the snapshot
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Late joiner connects
    let (mut guest, _) = connect_async(&url).await.expect("Guest connect failed");

    let first_msg = tokio::time::timeout(Duration::from_millis(500), guest.next())
        .await
        .expect("Guest timeout waiting for initial snapshot")
        .expect("Stream ended")
        .expect("Guest read error");

    match first_msg {
        Message::Binary(data) => {
            assert_eq!(
                data.as_ref(),
                snapshot_bytes.as_slice(),
                "Late joiner did not receive last_snapshot as first frame"
            );
        }
        other => panic!("Expected binary snapshot, got {:?}", other),
    }
}

#[tokio::test]
async fn test_room_cleanup_after_disconnect() {
    let config = RelayConfig {
        cleanup_timeout: Duration::from_millis(60),
    };
    let (addr, state) = spawn_test_server(config).await;
    let url = format!("ws://{}/ws?room=room-ephemeral", addr);

    let (client, _) = connect_async(&url).await.expect("Client connect failed");
    tokio::time::sleep(Duration::from_millis(30)).await;
    assert_eq!(state.room_count(), 1, "Room count should be 1 after connect");

    // Drop client to disconnect
    drop(client);

    // Wait for timeout to expire
    tokio::time::sleep(Duration::from_millis(120)).await;
    assert_eq!(
        state.room_count(),
        0,
        "Room should be pruned from RAM after cleanup timeout"
    );
}
