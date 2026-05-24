use temporalio_client::{Client, ClientOptions, Connection};
use temporalio_common::envconfig::LoadClientConfigProfileOptions;
use temporalio_sdk::{Worker, WorkerOptions};
use temporalio_sdk_core::{CoreRuntime, RuntimeOptions};
use workflow_demo::TASK_QUEUE;
use workflow_demo::activities::PrehabActivities;
use workflow_demo::workflow::PrehabWorkflow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info,workflow_demo=debug")
        .init();

    tracing::info!("Starte Prähab-Worker auf Task-Queue '{}'", TASK_QUEUE);

    let runtime = CoreRuntime::new_assume_tokio(RuntimeOptions::builder().build()?)?;

    let (conn_options, client_options) =
        ClientOptions::load_from_config(LoadClientConfigProfileOptions::default())?;

    let connection = Connection::connect(conn_options).await?;
    let client = Client::new(connection, client_options)?;

    let worker_options = WorkerOptions::new(TASK_QUEUE)
        .register_activities(PrehabActivities)
        .register_workflow::<PrehabWorkflow>()
        .build();

    Worker::new(&runtime, client, worker_options)?
        .run()
        .await?;

    Ok(())
}
