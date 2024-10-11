#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn static_handler() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/src/main.rs").await?;

    Ok(())
}
