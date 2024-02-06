use std::i32;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct User {
    login: String,
    id: i32,
}

pub async fn git_test() {
    // Result<(), reqwest::Error>
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url)
        .await
        .unwrap()
        .json::<Vec<User>>()
        .await
        .unwrap();

    // let users: Vec<User> = response.json().await?;
    println!("{:?}", response);
    // Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Area {
    area_name: String,
    id: i32,
}
#[tokio::main]
pub async fn mrbs_test() {
    let request = format!("http://localhost:7878/allareas");
    let response = reqwest::get(&request)
        .await
        .unwrap()
        .json::<Vec<Area>>()
        .await
        .unwrap();
    // let area: Vec<Area> = response.json().await.unwrap();
    println!("{:?}", response);
}
