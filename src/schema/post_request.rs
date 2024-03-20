use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub(crate) content: String,
}