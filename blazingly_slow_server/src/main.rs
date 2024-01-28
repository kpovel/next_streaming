use axum::{routing::get, Router};
use std::{thread, time::Duration};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/slow", get(slow_route))
        .route("/a-brand-new", get(a_brand_new));

    let listener = TcpListener::bind("0.0.0.0:42069").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn slow_route() -> &'static str {
    thread::sleep(Duration::from_secs(2));

    "slow response"
}

async fn a_brand_new() -> &'static str {
    thread::sleep(Duration::from_secs(2));

    "a branch new response"
}

async fn root() -> &'static str {
    "Hello, World!"
}
