use axum::{
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue},
    response::Html,
    Json,
};
use serde::{Deserialize, Serialize};

pub async fn test_html() -> Html<&'static str> {
    Html(include_str!("../../index.html"))
}

pub async fn path_var(Path(id): Path<i32>) -> String {
    id.to_string()
}

#[derive(Serialize, Deserialize)]
pub struct Example {
    id: i32,
    message: String,
}

pub async fn param_var(Query(example): Query<Example>) -> Json<Example> {
    Json(example)
}

pub async fn show_header(headers: HeaderMap) -> String {
    // X-Date as param or User-Agent
    let test_val = headers.get("User-Agent").unwrap();
    let test = test_val.to_str().unwrap().to_owned();
    test
}
