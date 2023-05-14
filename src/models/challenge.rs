use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ChallengeRequest {
    pub s: u8,
    pub d: ChallengeData,
}

#[derive(Deserialize)]
pub struct ChallengeData {
    #[serde(rename = "type")]
    pub type_: u8,
    pub channel_type: String,
    pub challenge: String,
    pub verify_token: String,
}

#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    pub challenge: String,
}

#[derive(Deserialize)]
pub struct EncryptedMessage {
    pub encrypt: String,
}