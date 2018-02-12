use serde_json;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::error::Error),
}
