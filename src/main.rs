mod api;
mod scheduler;
mod executor;
mod dag;
mod storage;
mod config;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Initialize shared state
    let state = Arc::new(Mutex::new(storage::init_storage().await));

    // Run API server
    api::start_server(state).await;
}
