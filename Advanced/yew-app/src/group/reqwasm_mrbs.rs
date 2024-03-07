use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use web_sys::{console, js_sys::JsString};

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
pub fn call_mrbs_test() {
    wasm_bindgen_futures::spawn_local(async move{
        let data_endpoint = format!("http://127.0.0.1:7878/today");
        let data_fetch = Request::get(&data_endpoint)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        console::log_1(&JsString::from(data_fetch));
    });
}