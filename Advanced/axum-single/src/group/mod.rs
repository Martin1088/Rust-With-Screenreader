mod api_call;

use api_call::index;
use tokio::net::TcpListener;

use axum::{routing::get, Router};

#[tokio::main]
pub async fn run_serve() {
    let app = Router::new().route("/", get(index));

    let listener = TcpListener::bind("0.0.0.0:7880").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
