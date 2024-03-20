// src/schema/response1.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response3 {
    pub message: String,
    pub data: String,
}
