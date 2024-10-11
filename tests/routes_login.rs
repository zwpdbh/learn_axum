#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn routes_login_succeed() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn routes_login_failed() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1xxx",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
