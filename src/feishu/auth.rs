use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GetAccessTokenRequest {
    app_id: String,
    app_secret: String,
}

#[derive(Debug, Deserialize)]
struct GetAccessTokenResponse {
    code: i32,
    msg: String,
    app_access_token: String,
    // expire: i32,
}

pub async fn get_access_token(app_id: String, app_secret: String) -> Option<String> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal")
        .header("Content-Type", "application/json; charset=utf-8")
        .json(&GetAccessTokenRequest {
            app_id: app_id,
            app_secret: app_secret,
        })
        .send()
        .await;
    match res {
        Ok(res) => match res.text().await {
            Ok(body) => {
                let access_token_result: Result<GetAccessTokenResponse, serde_json::Error> =
                    serde_json::from_str(&body);
                match access_token_result {
                    Ok(access_token) => {
                        tracing::info!(
                            "Got access token, code: {}, msg: {}",
                            access_token.code,
                            access_token.msg
                        );
                        Some(access_token.app_access_token)
                    }
                    Err(err) => {
                        tracing::error!("Error: {}", err);
                        None
                    }
                }
            }
            Err(err) => {
                tracing::error!("Error: {}", err);
                None
            }
        },
        Err(err) => {
            tracing::error!("Error: {}", err);
            None
        }
    }
}

// 周期性地获取 access token
pub async fn get_access_token_periodically(app_id: String, app_secret: String) -> Option<String> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    loop {
        let access_token = get_access_token(app_id.clone(), app_secret.clone()).await;
        if access_token.is_some() {
            tracing::info!("Refresh token {}", access_token.unwrap());
        }
        interval.tick().await;
    }
}
