use mongodb::bson::uuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct CardMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub theme: Option<String>,
    pub color: Option<String>,
    pub size: Option<String>,
    pub modules: Vec<Module>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    #[serde(rename = "type")]
    pub module_type: String,
    pub text: Option<Element>,
    pub mode: Option<String>,
    pub accessory: Option<Element>,
    pub elements: Option<Vec<Element>>,
    pub src: Option<String>,
    pub title: Option<String>,
    pub cover: Option<String>,
    pub endTime: Option<u64>,
    pub startTime: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    #[serde(rename = "type")]
    pub element_type: String,
    pub content: Option<String>,
    pub emoji: Option<bool>,
    pub value: Option<String>,
    pub click: Option<String>,
    pub cols: Option<u8>,
    pub fields: Option<Vec<Field>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "type")]
    pub field_type: String,
    pub content: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "type")]
    pub text_type: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    #[serde(rename = "type")]
    pub button_type: String,
    pub theme: String,
    pub value: String,
    pub text: Text,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionGroup {
    #[serde(rename = "type")]
    pub action_group_type: String,
    pub elements: Vec<Button>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    #[serde(rename = "type")]
    pub context_type: String,
    pub elements: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PvPCard {
    #[serde(rename = "type")]
    pub card_type: String,
    pub theme: String,
    pub size: String,
    pub modules: Vec<Module>,
    pub action_group: Option<ActionGroup>,
    pub context: Option<Context>,
    pub id: Uuid,
}

impl PvPCard {
    pub fn new(card_type: String, theme: String, size: String, modules: Vec<Module>, action_group: Option<ActionGroup>, context: Option<Context>) -> Self {
        PvPCard {
            card_type,
            theme,
            size,
            modules,
            action_group,
            context,
            id: Uuid::new(),
        }
    }
}
