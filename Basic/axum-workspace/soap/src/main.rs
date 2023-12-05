use std::net::SocketAddr;

use axum::body;
use axum::http::{Request, Response, StatusCode};
use axum::routing::get;
use axum::routing::Router;

async fn soap_request_handler() -> Result<Response<T>, StatusCode> {
    // Simulate handling the SOAP request
    // Here, you'd parse the SOAP request and perform the required operations

    // Simulate a SOAP response
    let soap_response = r#"
        <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/">
            <SOAP-ENV:Body>
                <Response>Data from SOAP request</Response>
            </SOAP-ENV:Body>
        </SOAP-ENV:Envelope>
    "#;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/xml")
        .body(soap_response.into())
        .unwrap())
}

#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = Router::new().route("/soap", get(soap_request_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    // Run the server on port 8000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
