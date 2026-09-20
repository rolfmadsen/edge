use kant_relay::{create_app, AppState, RelayConfig};
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber from RUST_LOG env or fallback to "info"
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info,kant_relay=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("🚀 Kant Stateless Relay listening on http://{}", addr);
    tracing::info!(
        "📡 WebSocket endpoint available at ws://{}/ws?room=<ROOM_ID>",
        addr
    );
    tracing::info!("🩺 Health endpoint available at http://{}/health", addr);

    let state = AppState::new(RelayConfig::default());
    let app = create_app(state);

    axum::serve(listener, app).await?;

    Ok(())
}
