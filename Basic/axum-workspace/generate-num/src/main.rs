use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::{net::SocketAddr, u64};

#[tokio::main()]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listen to {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
#[derive(Deserialize)]
struct RangeParameter {
    start: u64,
    end: u64,
}

async fn handler(Query(range): Query<RangeParameter>) -> Html<String> {
    let rand_number = thread_rng().gen_range(range.start..range.end);
    Html(format!("<h1> Random Numer {} </h1> <br>", rand_number))
}
