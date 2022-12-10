use serde::Deserialize;

use super::common::EventUserId;

#[derive(Debug, Deserialize)]
pub struct IMMessageSender {
    pub sender_id: EventUserId,
    pub sender_type: String,
    pub tenant_key: String,
}

#[derive(Debug, Deserialize)]
pub struct IMMessageMention {
    pub key: String,
    pub id: EventUserId,
    pub name: String,
    pub tenant_key: String,
}

#[derive(Debug, Deserialize)]
pub struct IMMessageContent {
    pub message_id: String,
    #[serde(default)]
    pub root_id: String,
    #[serde(default)]
    pub parent_id: String,
    pub create_time: String,
    pub chat_id: String,
    pub chat_type: String,
    pub message_type: String,
    pub content: String,
    #[serde(default)]
    pub mentions: Vec<IMMessageMention>,
}

#[derive(Debug, Deserialize)]
pub struct IMMessageText {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct IMMessageReceiveEvent {
    pub sender: IMMessageSender,
    pub message: IMMessageContent,
}
