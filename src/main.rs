mod chat_context;
mod completion;
mod feishu;
use std::{collections::HashMap, env, str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, Server,
};
use chat_context::ChatContextMap;
use dotenvy::dotenv;
use feishu::{
    auth::get_access_token_periodically,
    events::{common::BotEvent, EventType},
};
use tokio::task;
use tracing::{log::warn};

use crate::{
    completion::completion,
    feishu::{
        events::im_message::{IMMessageReceiveEvent, IMMessageText},
        message::reply_message,
    },
};

pub struct BotState {
    pub openai_key: String,
    pub access_token: String,
    pub chat_context_map: ChatContextMap,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt().pretty().init();
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let openai_key = env::var("OPENAI_KEY").unwrap();
    let bot_state = Arc::new(RwLock::new(BotState {
        openai_key,
        access_token: "".to_string(), // 之后在定时任务中更新
        chat_context_map: RwLock::new(HashMap::new()),
    }));
    // 周期性地获取 access token
    let access_token_task = task::spawn(get_access_token_periodically(
        app_id.clone(),
        app_secret.clone(),
        bot_state.clone(),
    ));
    tokio::spawn(access_token_task);
    // 储存上下文会话的map
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/bot", post(bot))
        .with_state(bot_state);

    Server::bind(&format!("{}:{}", host, port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn bot(
    State(state): State<Arc<RwLock<BotState>>>,
    extract::Json(bot_event): extract::Json<BotEvent>,
) -> impl IntoResponse {
    let et = bot_event.header.event_type;
    if let Ok(event_type) = EventType::from_str(et.as_str()) {
        match event_type {
            EventType::IMMessageReceive => {
                let e: IMMessageReceiveEvent = serde_json::from_value(bot_event.event).unwrap();
                let text_message: IMMessageText =
                    serde_json::from_str(e.message.content.as_str()).unwrap();
                let bot_state = state.read().await;
                let mut chat_context_map = bot_state.chat_context_map.write().await;
                let c_message: String;
                if !chat_context_map.contains_key(&e.sender.sender_id.user_id) {
                    let mut chat_context = chat_context::ChatContext::new();
                    c_message = text_message.text.clone();
                    chat_context.add_message(
                        chat_context::MessageSender::Human,
                        text_message.text.clone(),
                    );
                    chat_context_map.insert(e.sender.sender_id.user_id.clone(), chat_context);
                } else {
                    let chat_context = chat_context_map
                        .get_mut(&e.sender.sender_id.user_id)
                        .unwrap();
                    // 如果用户不是在回复消息，则清空上下文
                    if e.message.parent_id == "" || e.message.parent_id != chat_context.current_message_id {
                        chat_context.clear();
                    }
                    // c_message = format!("{}\n{}", chat_context.messages.clone(), text_message.text.clone());
                    chat_context.add_message(chat_context::MessageSender::Human, text_message.text);
                    c_message = chat_context.messages.clone();
                    // info!("Chat context:\n{}", chat_context.messages);
                }

                let openai_key = bot_state.openai_key.clone();
                let access_token = bot_state.access_token.clone();
                let c_task = task::spawn(completion_chat(
                    e.message.message_id,
                    e.sender.sender_id.user_id.clone(),
                    c_message,
                    openai_key,
                    access_token,
                    state.clone(),
                ));
                tokio::spawn(c_task);
            }
        }
    } else {
        tracing::warn!("Unknown event type: {}", et);
    }
    (StatusCode::OK, "OK")
}

async fn completion_chat(
    message_id: String,
    user_id: String,
    message_content: String,
    openai_key: String,
    access_token: String,
    state: Arc<RwLock<BotState>>,
) {
    let mut completion_result = completion(message_content.clone(), openai_key).await;
    if completion_result == "" {
        warn!("Completion result is empty");
        completion_result = "我不知道你在说什么(ChatGPT返回空值)".to_string();
    }
    let message_id = reply_message(message_id, &completion_result, access_token).await;
    state
        .read()
        .await
        .chat_context_map
        .write()
        .await
        .get_mut(&user_id)
        .unwrap()
        .add_message_with_id(
            chat_context::MessageSender::AI,
            completion_result.clone(),
            message_id,
        );
}

// #[derive(Debug, Deserialize)]
// struct ChallengeRequest {
// challenge: String,
// token: String,
// #[serde(rename = "type")]
// type_: String,
// }

// #[derive(Debug, Serialize)]
// struct ChallengeResponse {
//     challenge: String,
// }
