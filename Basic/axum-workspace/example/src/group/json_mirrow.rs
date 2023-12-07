use serde
struct Room {
    name: String,
    room: String,
}
struct Area{
    mrbs_area: String,
    room: Vec<Room>
}

pub async json_mirrow() {}
