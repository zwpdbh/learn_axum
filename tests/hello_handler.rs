#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn hello_handler2() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello2/Mike").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn hello_handler() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Jen").await?.print().await?;

    Ok(())
}
