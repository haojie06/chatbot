use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct EventUserId {
    pub union_id: String,
    pub user_id: String,
    pub open_id: String,
}
