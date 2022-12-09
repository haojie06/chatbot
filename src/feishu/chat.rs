use serde::Serialize;

#[derive(Serialize)]
struct ReplyMessagePaylod {
  content: String,
  msg_type: String,
}

pub async fn reply_message(message_id: String, content: String, access_token: String) {
  tracing::warn!("Replying message: {}", access_token);
  let client = reqwest::Client::new();
  let res = client
    .post(format!("https://open.feishu.cn/open-apis/im/v1/messages/{}/reply", message_id))
    .header("Authorization", format!("Bearer {}", access_token))
    .json(&ReplyMessagePaylod {
      content: content,
      msg_type: "text".to_string(),
    })
    .send().await;
  match res {
    Ok(res) => match res.text().await {
      Ok(body) => {
        tracing::info!("{}", body);
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