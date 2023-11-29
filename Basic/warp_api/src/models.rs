use std::u64;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}
