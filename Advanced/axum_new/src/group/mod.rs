mod test_html;
use axum::{routing::get, Router};
use test_html::test_html;
use tokio::net::TcpListener;

#[tokio::main()]
pub async fn run_server() {
    // let app = create_routes();
    let app: Router = Router::new().route("/", get(test_html));
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
