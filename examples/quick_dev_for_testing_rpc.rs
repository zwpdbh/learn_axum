#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // region:      --- Test RPC api for Task
    let req_list_tasks = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "list_tasks"
        }),
    );
    let _ = req_list_tasks.await?.print().await?;

    // endregion:   --- Test RPC api for Task

    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true
        }),
    );
    req_logoff.await?.print().await?;

    Ok(())
}
