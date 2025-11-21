use std::net::{Ipv4Addr, SocketAddr};
use anyhow::Result;
use axum::{routing::get, Router, http::StatusCode};
use tokio::net::TcpListener;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}


#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/health_check", get(health_check));
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8080));

    // await? とすることで下記を簡潔に書ける
    // 但し、 Result とセットで使う。
    // let listener = match TcpListener::bind(addr).await {
    //     Ok(l) => l,
    //     Err(e) => return Err(e.into()),
    // };
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
}
