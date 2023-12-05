mod route;
use axum::routing::get;
use axum::routing::Router;
use route::{json_test, plain_text, soap_test};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = Router::new()
        .route("/soap", get(soap_test))
        .route("/plain_text", get(plain_text))
        .route("/json", get(json_test));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    // Run the server on port 8000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
