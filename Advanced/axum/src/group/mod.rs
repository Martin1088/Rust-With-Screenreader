mod json_mirror;
mod middle;
mod mirror_string;
mod state;
mod test_html;

use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use json_mirror::json_mirror;
use middle::middleware_message;
use mirror_string::mirror_string;
use state::show_state;
use test_html::test_html;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use self::{
    json_mirror::{json_mrbs, json_mrbs_full},
    middle::{always_errors, show_custom, SharedData},
    state::AppState,
    test_html::{param_var, path_var, show_header},
};


#[derive(OpenApi)]
#[openapi(
    paths(),
    components(),
    tags(),
)]
struct ApiDoc;
#[tokio::main()]
pub async fn run_server() {
    // let state_test = AppState {};
    // let app = create_routes();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shareddata = SharedData {
        message: "Axum is okay".to_owned(),
    };

    let app: Router = Router::new()
        .merge(SwaggerUi::new("/doc").url("/api/openapi.json", ApiDoc::openapi()))
        .route("/html", get(test_html))
        .route("/mirror", post(mirror_string))
        .route("/mirrow_json", post(json_mirror))
        .route("/mrbs", post(json_mrbs))
        .route("/mrbsfull", post(json_mrbs_full))
        .route("/path_var/:id", post(path_var))
        .route("/param", post(param_var))
        .route("/header", get(show_header))
        .route("/message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shareddata))
        .route("/error", get(always_errors));

    // .route("/state", get(show_state))
    // .with_state(state_test);

    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
