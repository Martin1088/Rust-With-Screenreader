mod json_mirrow;
mod string_mirrow;
mod test_json;
mod text;

use axum::{
    routing::{get, post},
    Router,
};

use json_mirrow::json_mirrow;
use string_mirrow::test_mirrow;
use test_json::json_test;
use text::plain_text;

use std::net::SocketAddr;

pub async fn test_run() -> () {
    // Create a new Axum router
    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json_test))
        .route("/string_mirrow", post(test_mirrow))
        .route("/json_mirrow", post(json_mirrow));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    // Run the server on port 8000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
