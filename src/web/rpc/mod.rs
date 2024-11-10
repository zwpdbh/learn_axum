// region:      --- Modules

mod task_rpc;

use crate::{
    model::ModelManager,
    web::{Error, Result},
    Ctx,
};
use axum::{extract::State, Router};
use axum::{response::IntoResponse, Json};
use axum::{response::Response, routing::post};
use serde::Deserialize;
use serde_json::{json, to_value, Value};
use task_rpc::list_tasks;
use tracing::debug;

// endregion:   --- Modules

// region:      --- RPC types

/// JSON-RPC Request Body.
#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
    id: i64,
}

// endregion:   --- RPC types

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    _rpc_handler(ctx, mm, rpc_req).await.into_response()
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");
    let result_json: Value = match rpc_method.as_str() {
        "create_task" => todo!(),
        "list_tasks" => {
            let r = list_tasks(ctx, mm).await.map(|r| to_value(r))??;
            r
        }
        "update_task" => todo!(),
        "delete_task" => todo!(),
        _ => return Err(Error::RpcMethodUnknow(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json
    });

    Ok(Json(body_response))
}
