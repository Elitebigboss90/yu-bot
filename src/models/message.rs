use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MessageRequest {
    #[serde(rename = "type")]
    pub message_type: Option<u8>,
    pub target_id: String,
    pub content: String,
    pub quote: Option<String>,
    pub nonce: Option<String>,
    pub temp_target_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub msg_id: String,
    pub msg_timestamp: i64,
    pub nonce: String,
}