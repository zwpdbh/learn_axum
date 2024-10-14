#![allow(unused)]

pub use self::ctx::Ctx;
pub use self::error::{Error, Result};
use std::net::SocketAddr;

use axum::Json;
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, Route},
    Router,
};
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

mod ctx;
mod error;
mod log;
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
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
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
    let uuid = Uuid::new_v4();

    // Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());
    let error_response = client_status_error.map(|(status_code, client_error)| {
        let client_error_body = json!({
            "error": {
                "type": client_error,
                "req_uuid": uuid.to_string(),
            }
        });
        println!("->> client_error_body: {client_error_body}");

        // Build the new resonse from the client_error_body
        (status_code, Json(client_error_body)).into_response()
    });

    // TODO:: Build and log the server log line.
    println!("->> server log line - {uuid} - Error: {service_error:?}");

    error_response.unwrap_or(res)
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
