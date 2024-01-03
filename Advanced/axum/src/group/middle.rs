use axum::{http::StatusCode, Extension};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub async fn middleware_message(Extension(sharedata): Extension<SharedData>) -> String {
    sharedata.message
}
// has changed not working
#[derive(Clone)]
pub struct HeaderMessage(String);

pub async fn show_custom(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

pub async fn always_errors() -> Result<(), StatusCode> {
    // Ok(())
    Err(StatusCode::IM_A_TEAPOT)
}
