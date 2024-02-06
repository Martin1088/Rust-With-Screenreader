use serde::{Deserialize, Serialize};

pub async fn index() {
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
#[derive(Debug, Deserialize, Serialize)]
struct Area {
    area_name: String,
    id: i32,
}
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
