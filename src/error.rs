use serde_json;

pub enum Error {
  Json(serde_json::error::Error)
}
