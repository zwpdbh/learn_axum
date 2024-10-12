#![allow(unused)]

pub use self::error::{Error, Result};
use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, Route},
    Router,
};
use model::ModelController;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        // when we want to apply middleware to only some route, use route_layer
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        // layer get executed from bottom to top
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_statics());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {addr}\n");
    axum_server::bind(addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

/// Handle client and server error seperately
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
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
