use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryRelatedData {
    id: i32,
    area_name: String,
    rooms: Vec<RoomsRelatedData>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct RoomsRelatedData {
    id: i32,
    room_name: String,
    description: String,
    capacity: i32,
    entry_id: i32,
    registration_limit: i32,
    participants: Vec<String>,
}
/*
pub  fn mrbs_test() -> Vec<EntryRelatedData> {
    let request = format!("http://localhost:7878/today");
    let response ;
        // .json::<Vec<EntryRelatedData>>();
    // let area: Vec<Area> = response.json().await.unwrap();
    // println!("{:?}", response);
    response
}
*/