use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ImageMessageContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageUrl {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageMessage {
    pub role: Role,
    pub content: Vec<ImageMessageContent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Message {
    TextMessage(TextMessage),
    ImageMessage(ImageMessage),
}
