#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::{get, Route},
    Router,
};

#[tokio::main]
async fn main() {
    let routers_hello = Router::new().route("/hello", get(handler_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {addr}\n");
    axum_server::bind(addr)
        .serve(routers_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong> World </strong>")
}
