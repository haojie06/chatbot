mod chat;
mod feishu;
use std::{env, str::FromStr};

use axum::{
    extract,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router, Server,
};
use dotenvy::dotenv;
use feishu::{events::EventType, auth::get_access_token};
use serde::{Deserialize, Serialize};

use crate::{
    chat::completion,
    feishu::{chat::reply_message, events::im_message::IMMessageReceiveEvent},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let openai_key = env::var("OPENAI_KEY").unwrap();
    let access_token = get_access_token(app_id, app_secret).await.unwrap();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/bot", post(bot))
        .layer(Extension((openai_key, access_token)));

    Server::bind(&format!("{}:{}", host, port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct ChallengeRequest {
    // challenge: String,
    // token: String,
    // #[serde(rename = "type")]
    // type_: String,
}

#[derive(Debug, Serialize)]
struct ChallengeResponse {
    challenge: String,
}

async fn bot(
    Extension((openai_key, access_token)): Extension<(String, String)>,
    extract::Json(bot_event): extract::Json<BotEvent>,
) -> impl IntoResponse {

    let et = bot_event.header.event_type;
    if let Ok(event_type) = EventType::from_str(et.as_str()) {
        // tracing::info!("Event type: {:?}", event_type);
        match event_type {
            EventType::IMMessageReceive => {
                let e: IMMessageReceiveEvent = serde_json::from_value(bot_event.event).unwrap();
                tracing::info!("Chat message: {:?}", e.message.message_id);
                let completion_result = completion(e.message.content.clone(), openai_key).await;
                tracing::info!("Completion result: {}", completion_result);
                reply_message(
                    e.message.message_id,
                    e.message.content.clone(),
                    access_token,
                )
                .await;
            }
        }
    } else {
        tracing::warn!("Unknown event type: {}", et);
    }
    (StatusCode::OK, "OK")
}

#[derive(Debug, Deserialize)]
struct BotEvent {
    // schema: String,
    header: BotEventHeader,
    event: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct BotEventHeader {
    // event_id: String,
    // token: String,
    // create_time: String,
    event_type: String,
    // tenant_key: String,
    // app_id: String,
}
