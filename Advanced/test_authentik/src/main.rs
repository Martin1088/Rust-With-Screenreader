use reqwest::{header, StatusCode};
use serde_json::json;

const TOKEN: &str = "UQkaNZQi7vPpeMNOuLEVik5V1zu6YONPH7arBzjYE3U8uf1Gghsy8M7hlsWQ";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("Accept", "application/json".parse()?);
    headers.insert("Authorization", format!("Bearer {TOKEN}").parse()?);

    let data = r#"{
    "name": "beispiele1"
}"#;

    let json: serde_json::Value = serde_json::from_str(data)?;

    let request = client
        .post("http://localhost:9000/api/v3/core/groups/")
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_group() {
    post_group("tester8", TOKEN).await;
}
pub async fn post_group(name: &str, token: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:9000/api/v3/core/groups/")
        .bearer_auth(token)
        .json(&json!({
            "name": name
        }))
        .send()
        .await?;
    match res.status() {
        StatusCode::CREATED => println!("Created group {name}"),
        StatusCode::OK => println!("Created group {name}"),
        StatusCode::CONFLICT => println!("Group {name} already existed"),
        s => anyhow::bail!("Unexpected statuscode {s} while creating group {name}"),
    }
    Ok(())
}
