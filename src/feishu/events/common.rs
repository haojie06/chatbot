use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct EventUserId {
    pub union_id: String,
    pub user_id: String,
    pub open_id: String,
}

#[derive(Debug, Deserialize)]
pub struct BotEvent {
    // schema: String,
    pub header: BotEventHeader,
    pub event: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct BotEventHeader {
    // event_id: String,
    // token: String,
    // create_time: String,
    pub event_type: String,
    // tenant_key: String,
    // app_id: String,
}