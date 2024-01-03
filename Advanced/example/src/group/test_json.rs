use axum::Json;
use serde_json::{json, Value};

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
pub async fn json_test() -> Json<Value> {
    // Json(json!({ "data": 42 }));
    Json(json!(
    [
        {
            "mrbs_area": "Heidelberg",
            "mitarbeiter": [
                {"name": "Torben", "room": "S.2.124"},
                {"name": "Emil", "room": "S.2.110"}
            ]
        },
        {
            "mrbs_area": "Mannheim",
            "mrbs_room": [
                {"name" : "Patrik", "room": "004"}
            ]
        },
        {
            "mrbs_area" : "Homeoffice",
            "mitarbeiter" : [
                {"name" : "Jori", "room" : "000"}
            ]
        }
    ]
    ))
}
