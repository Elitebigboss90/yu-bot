use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterQuote {
    pub name: &'static str,
    pub quote: &'static str,
}