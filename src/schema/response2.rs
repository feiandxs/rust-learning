// src/schema/response2.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response2 {
    pub status: bool,
    pub result: String,
}