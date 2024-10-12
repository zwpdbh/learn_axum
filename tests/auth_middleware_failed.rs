use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn auth_middleware() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA",

        }),
    );
    req_create_ticket.await?.print().await?;

    Ok(())
}
