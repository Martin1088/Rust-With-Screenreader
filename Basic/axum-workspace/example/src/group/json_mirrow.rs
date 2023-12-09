use axum::Json;
use serde::Serialize;

struct Room {
    name: String,
    room: String,
}
struct Area {
    mrbs_area: String,
    room: Vec<Room>,
}

pub async fn json_mrbs() {}

pub async fn json_mirrow() {}
