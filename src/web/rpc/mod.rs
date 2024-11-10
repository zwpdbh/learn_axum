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
use serde_json::{from_value, json, to_value, Value};
use task_rpc::{create_task, delete_task, list_tasks, update_task};
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

macro_rules! exec_rpc_fn {
    // With params
    // Notice the extra "{}" around the block:
    // Block expression: The curly braces create a block expression, which allows multiple statements to be grouped together and treated as a single expression.
    // Macro hygiene: The block helps maintain macro hygiene by isolating the macro's internal variables (like rpc_fn_name and params) from the surrounding context.
    ($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
        let rpc_fn_name = stringify!($rpc_fn);
        let params = $rpc_params.ok_or(Error::RpcMissParams {
            prc_method: rpc_fn_name.to_string(),
        })?;
        let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;

        $rpc_fn($ctx, $mm, params).await.map(to_value)??
    }};

    // Without params
    ($rpc_fn:expr, $ctx:expr, $mm:expr) => {
        $rpc_fn($ctx, $mm).await.map(to_value)??
    };
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");
    let result_json: Value = match rpc_method.as_str() {
        // "create_task" => {
        //     let params = rpc_params.ok_or(Error::RpcMissParams {
        //         prc_method: "create_task".to_string(),
        //     })?;
        //     let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
        //         rpc_method: "create_task".to_string(),
        //     })?;
        //     let r = create_task(ctx, mm, params).await.map(to_value)??;
        //     r
        // }
        // "list_tasks" => {
        //     let r = list_tasks(ctx, mm).await.map(|r| to_value(r))??;
        //     r
        // }
        "create_task" => {
            exec_rpc_fn!(create_task, ctx, mm, rpc_params)
        }
        "list_tasks" => {
            exec_rpc_fn!(list_tasks, ctx, mm)
        }
        "update_task" => exec_rpc_fn!(update_task, ctx, mm, rpc_params),
        "delete_task" => exec_rpc_fn!(delete_task, ctx, mm, rpc_params),
        _ => return Err(Error::RpcMethodUnknow(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json
    });

    Ok(Json(body_response))
}
