use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::post,
};
use serde::Deserialize;
use std::sync::Arc;
use temporalio_client::{Client, ClientOptions, Connection, WorkflowSignalOptions, WorkflowStartOptions};
use temporalio_common::envconfig::LoadClientConfigProfileOptions;
use temporalio_sdk_core::{CoreRuntime, RuntimeOptions};
use workflow_demo::{
    EnrollPatient, PromResponse, ReviewDecision, TASK_QUEUE,
};
use workflow_demo::workflow::PrehabWorkflow;

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let _runtime = CoreRuntime::new_assume_tokio(RuntimeOptions::builder().build()?)?;

    let (conn_options, client_options) =
        ClientOptions::load_from_config(LoadClientConfigProfileOptions::default())?;

    let connection = Connection::connect(conn_options).await?;
    let client = Arc::new(Client::new(connection, client_options)?);

    let state = AppState { client };

    let app = Router::new()
        .route("/enroll", post(enroll))
        .route("/prom", post(prom))
        .route("/coordinator-review", post(coordinator_review))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await?;
    tracing::info!("HTTP API auf http://localhost:9000");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn enroll(
    State(state): State<AppState>,
    Json(input): Json<EnrollPatient>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let workflow_id = format!("prehab-{}", input.pseudonym);

    let handle = state
        .client
        .start_workflow(
            PrehabWorkflow::run,
            input.clone(),
            WorkflowStartOptions::new(TASK_QUEUE, workflow_id.clone()).build(),
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "workflow_id": workflow_id,
        "run_id": handle.run_id(),
        "ui_url": format!("http://localhost:8233/namespaces/default/workflows/{}", workflow_id),
    })))
}

#[derive(Deserialize)]
struct PromInput {
    pseudonym: String,
    questionnaire: String,
    score: i32,
}

async fn prom(
    State(state): State<AppState>,
    Json(input): Json<PromInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let workflow_id = format!("prehab-{}", input.pseudonym);
    let handle = state.client.get_workflow_handle::<PrehabWorkflow>(&workflow_id);

    handle
        .signal(
            PrehabWorkflow::on_prom_completed,
            PromResponse {
                questionnaire: input.questionnaire.clone(),
                score: input.score,
            },
            WorkflowSignalOptions::default(),
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "status": "signal_sent",
        "workflow_id": workflow_id,
        "questionnaire": input.questionnaire,
    })))
}

#[derive(Deserialize)]
struct ReviewInput {
    pseudonym: String,
    proceed: bool,
    note: Option<String>,
}

async fn coordinator_review(
    State(state): State<AppState>,
    Json(input): Json<ReviewInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let workflow_id = format!("prehab-{}", input.pseudonym);
    let handle = state.client.get_workflow_handle::<PrehabWorkflow>(&workflow_id);

    handle
        .signal(
            PrehabWorkflow::on_coordinator_review,
            ReviewDecision {
                proceed: input.proceed,
                reviewer_note: input.note.unwrap_or_else(|| "(keine Notiz)".into()),
            },
            WorkflowSignalOptions::default(),
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "status": "review_recorded",
        "workflow_id": workflow_id,
        "proceed": input.proceed,
    })))
}
