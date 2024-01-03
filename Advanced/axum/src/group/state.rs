use axum::extract::State;

#[derive(Clone, Debug)]
pub struct AppState {}

pub async fn show_state(State(test): State<AppState>) {
    dbg!(test);
    todo!();
}
