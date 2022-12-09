use reqwest;

#[derive(serde::Serialize)]
struct CompletionPayload {
    model: &'static str,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

pub async fn completion(prompt: String, api_key: String) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}",api_key))
        .header("Content-Type", "application/json")
        .json(&CompletionPayload {
            model: "text-davinci-003",
            prompt: prompt,
            max_tokens: 64,
            temperature: 0.9,
        })
        .send()
        .await;
    match res {
        Ok(res) => match res.text().await {
            Ok(body) => {
                println!("{}", body);
                let completion_result: Result<CompletionResponse, serde_json::Error> = serde_json::from_str(&body);
                match completion_result {
                    Ok(completion) => completion.choices[0].text.clone(),
                    Err(err) => {
                        tracing::error!("Error: {}", err);
                        "Error".to_string()
                    }
                }
            }
            Err(err) => {
                tracing::error!("Error: {}", err);
                "Error".to_string()
            }
        },
        Err(err) => {
            tracing::error!("Error: {}", err);
            "Error".to_string()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CompletionChoice {
    text: String,
    // index: u32,
    // logprobs: Option<serde_json::Value>,
    // finish_reason: String,
}

#[derive(serde::Deserialize)]
pub struct CompletionUsage {
    // prompt_tokens: u32,
    // completion_tokens: u32,
    // total_tokens: u32,
}

#[derive(serde::Deserialize)]
pub struct CompletionResponse {
    // id: String,
    // object: String,
    // created: u32,
    // model: String,
    choices: Vec<CompletionChoice>,
    // usage: CompletionUsage,
}
