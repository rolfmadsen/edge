use axum::Router;
use std::time::Duration;

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

#[derive(Clone)]
pub struct AppState {
    pub config: RelayConfig,
}

impl AppState {
    pub fn new(config: RelayConfig) -> Self {
        Self { config }
    }

    pub fn room_count(&self) -> usize {
        0
    }
}

pub fn create_app(_state: AppState) -> Router {
    // RED Phase: Unimplemented router
    Router::new()
}
