use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    json_response: String,
}

