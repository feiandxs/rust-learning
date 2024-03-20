// src/schema/response1.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response1 {
    pub message: String,
    pub data: Vec<i32>,
}