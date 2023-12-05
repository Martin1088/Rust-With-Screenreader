use axum::body;
use axum::http::{Request, Response, StatusCode};
use axum::{body::Body, response::Json, routing::get, Router};
use reqwest::blocking::Client;
use serde_json::{json, Value};

pub async fn soap_test() -> String {
    let client = Client::new();
    let soap_response = r#"
        <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/">
            <SOAP-ENV:Body>
                <Response>Data from SOAP request</Response>
            </SOAP-ENV:Body>
        </SOAP-ENV:Envelope>
    "#;
    let response = client
        .post("http://exammple.com/soap_endpoint")
        .header("Content-Type", "text/xml")
        .body(soap_response.to_string())
        .send()
        .unwrap();
    response.text().unwrap()
}

// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
pub async fn plain_text() -> &'static str {
    "Welcome to Rust with plain text"
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
pub async fn json_test() -> Json<Value> {
    Json(json!({ "data": 42 }));
    Json(json!(
    [
        {
            "mrbs_area": "Heidelberg",
            "mitarbeiter": [
                {"name": "Torben", "room": "S.2.124"},
                {"name": "Emil", "room": "S.2.110"}
            ]
        },
        {
            "mrbs_area": "Mannheim",
            "mrbs_room": [
                {"name" : "Patrik", "room": "004"}
            ]
        },
        {
            "mrbs_area" : "Homeoffice",
            "mitarbeiter" : [
                {"name" : "Jori", "room" : "000"}
            ]
        }
    ]
    ))
}

/*
pub async fn soap_request_handler(req: Request) -> Result<Response, StatusCode> {
    let body = req.into_body().into_bytes().await?;
    let soap_request = String::from_utf8_lossy(&body);

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
*/
