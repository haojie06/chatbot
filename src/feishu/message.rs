use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ReplyMessagePaylod {
    content: String,
    msg_type: String,
}

#[derive(Debug, Serialize)]
struct ReplyMessageTextContent {
    text: String,
}

#[derive(Deserialize)]
struct ReplyMessageResponse {
    code: i32,
    msg: String,
}

pub async fn reply_message(message_id: String, content: String, access_token: String) {
    let content = serde_json::to_string(&ReplyMessageTextContent {
        text: content.clone(),
    })
    .unwrap();
    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "https://open.feishu.cn/open-apis/im/v1/messages/{}/reply",
            message_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&ReplyMessagePaylod {
            content: content,
            msg_type: "text".to_string(),
        })
        .send()
        .await;
    match res {
        Ok(res) => match res.text().await {
            Ok(body) => {
                let reply_message_result: Result<ReplyMessageResponse, serde_json::Error> =
                    serde_json::from_str(&body);
                if let Ok(reply_message) = reply_message_result {
                    (reply_message.code != 0).then(|| tracing::warn!("Reply message failed: {}", reply_message.msg));
                } else {
                    tracing::error!("Error: {}", reply_message_result.err().unwrap());
                }
            }
            Err(err) => {
                tracing::error!("Error: {}", err);
            }
        },
        Err(err) => {
            tracing::error!("Error: {}", err);
        }
    }
}
