use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WebhookEvent {
    pub s: u8,
    pub d: EventBody,
}

#[derive(Deserialize)]
pub enum MessageType {
    Text,
    Image,
    Video,
    File,
    Audio,
    KMarkdown,
    Card,
    System,
    Other(u8),
}

#[derive(Deserialize)]
pub enum ChannelType {
    Group,
    Person,
    Broadcast,
}

#[derive(Deserialize)]
pub struct ChallengeData {
    #[serde(rename = "type")]
    pub type_: u8,
    pub channel_type: String,
    pub challenge: String,
    pub verify_token: String,
}

#[derive(Deserialize)]
pub struct EventBody {
    #[serde(rename = "type")]
    pub type_: u8,
    pub channel_type: String,
    pub challenge: Option<String>,
    pub verify_token: Option<String>,
    pub target_id: String,
    pub author_id: String,
    pub content: String,
    pub msg_id: String,
    pub msg_timestamp: i64,
    pub nonce: String,
    pub extra: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    pub challenge: String,
}

#[derive(Deserialize)]
pub struct EncryptedMessage {
    pub encrypt: String,
}
