use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateSessionResp {
    #[serde(rename = "sessionId")]
    session_id: Uuid,
}


#[tokio::main]
async fn main() -> Result<()> {
    let base_url = "http://localhost:7887";
    let api_key = "changeme1";

    let http = Client::new();
    
    let session: CreateSessionResp = http
        .post(format!("{base_url}/sessions"))
        .header("mainzellisteApiKey", api_key)
        .send()
        .await
        .context("POST /sessions failed")?
        .error_for_status()
        .context("POST /sessions returned error")?
        .json()
        .await
        .context("parse /sessions JSON failed")?;

    println!("sessionId = {}", session.session_id);

    Ok(())
}