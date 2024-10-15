#![allow(unused)]
// #[config(test)]
pub mod _dev_utils;
mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::ctx::Ctx;
pub use self::error::{Error, Result};
pub use config::config;

use crate::model::ModelManager;
use crate::web::mw_res_map::mw_reponse_map;
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use web::routes_static;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    // let routes_rpc = rpc::routes(mm.clone())
    //   .route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("{:<12} - {addr}\n", "LISTENING");
    axum_server::bind(addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
