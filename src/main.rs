#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service, Route},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

mod error;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_statics());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {addr}\n");
    axum_server::bind(addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_statics() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

// For /hello?name=Jen
// The argument of the function mapping to the url query parameters, this is what query extractor allow us to do
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}

// For /hello/Miake
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
