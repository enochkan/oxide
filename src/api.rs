use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start_server(state: Arc<Mutex<()>>) {
    let health_route = warp::path!("health")
        .map(|| warp::reply::json(&serde_json::json!({"status": "OK"})));

    let routes = health_route;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
