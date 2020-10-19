use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct BadRequest {
    pub error: String,
    pub status: i32,
    pub message: String,
}
