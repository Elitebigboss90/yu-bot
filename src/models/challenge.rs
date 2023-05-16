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
    pub target_id: Option<String>,
    pub author_id: Option<String>,
    pub content: Option<String>,
    pub msg_id: Option<String>,
    pub msg_timestamp: Option<i64>,
    pub nonce: Option<String>,
    pub extra: Option<MessageExtra>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageExtra {
    #[serde(rename = "type")]
    pub msg_type: i32, // Assuming "type" is a reserved keyword, so I use msg_type instead
    pub guild_id: String,
    pub channel_name: String,
    pub mention: Vec<String>,
    pub mention_all: bool,
    pub mention_roles: Vec<String>,
    pub mention_here: bool,
    pub author: User, // Replace User with the actual User struct
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub identify_num: String,
    pub online: bool,
    pub bot: bool,
    pub status: i32,
    pub avatar: String,
    pub vip_avatar: String,
    pub mobile_verified: Option<bool>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    pub challenge: String,
}

#[derive(Deserialize)]
pub struct EncryptedMessage {
    pub encrypt: String,
}
