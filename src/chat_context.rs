use std::{collections::HashMap, fmt::Display};
use tokio::sync::RwLock;

// 为每个用户建立一个map，key是用户id，value是用户上下文，只要用户输入的信息是有parent_id的，就把上下文一起发给机器人，如果用户输入了不包含parent_id的信息，就把上下文清空
pub enum MessageSender {
    Human,
    AI,
}

impl Display for MessageSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageSender::Human => write!(f, "Human"),
            MessageSender::AI => write!(f, "AI"),
        }
    }
}
pub struct ChatContext {
    pub current_message_id: String, // bot 发送的最近一条消息的id
    pub messages: String,
}

impl ChatContext {
    pub fn new() -> Self {
        Self {
            current_message_id: "".to_string(),
            messages: "The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.".to_string(),
        }
    }

    pub fn add_message(&mut self, sender: MessageSender, message: String) {
        self.messages = format!("{}\n{}: {}", self.messages, sender, message)
    }

    pub fn add_message_with_id(&mut self, sender: MessageSender, message: String, message_id: String) {
        self.current_message_id = message_id;
        self.messages = format!("{}\n{}: {}", self.messages, sender, message)
    }

    pub fn clear(&mut self) {
        self.current_message_id = "".to_string();
        self.messages = "".to_string();
    }
}

pub type ChatContextMap = RwLock<HashMap<String, ChatContext>>;
