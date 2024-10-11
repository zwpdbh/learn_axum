use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn create_ticket() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA",

        }),
    );
    req_create_ticket.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn list_tickets() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn delete_ticket() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    hc.do_delete("/api/tickets/0").await?.print().await?;

    Ok(())
}
