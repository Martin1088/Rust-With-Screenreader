use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    name: String,
    room: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    mrbs_area: String,
    mitarbeiter: Vec<Room>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Overview {
    today: Vec<Area>,
}

// with Json<Area> only one ist possible like in mtest.json
// here mrbs.json all Area
pub async fn json_mrbs(Json(area): Json<Area>) -> (StatusCode, Response) {
    // dbg!(area);
    // todo!();
    // Json(area)
    (StatusCode::CREATED, Json(area).into_response())
}

pub async fn json_mrbs_full(Json(area): Json<Vec<Area>>) -> (StatusCode, Response) {
    // dbg!(area);
    // todo!();
    // Json(area)
    (StatusCode::CREATED, Json(area).into_response())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    text: String,
    name: String,
}

pub async fn json_mirror(Json(message): Json<Message>) -> Json<Message> {
    // dbg!(message);
    // todo!();
    Json(message)
}
