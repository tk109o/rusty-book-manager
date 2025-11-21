use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

// ハンドラと呼ばれる。
async fn hello_world() -> &'static str {
    "Hello, World!"
}


#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello_world));
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8080));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
