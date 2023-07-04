use std::env;
use reqwest::Client;
use serde_json::json;

const ROOT_URL: &str = "https://api.openai.com/v1/";

#[derive(serde::Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: ChatUsage,
}

#[derive(serde::Deserialize)]
pub struct ChatChoice {
    pub index: u64,
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(serde::Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String
}

#[derive(serde::Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

pub async fn chat(message_content: &str, user_name: &str) -> reqwest::Result<ChatResponse> {
    let endpoint = format!("{}{}", ROOT_URL, "chat/completions");

    let params = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {"role": "system", "content": "You are the evil brother of Zayden, named Viktor. You like to use short sentences.", "name": "Viktor"},
            {"role": "user", "content": message_content, "name": user_name}
        ],
        "max_tokens": 50
    });

    let client = Client::new();
    let res = client.post(endpoint)
        .header("Authorization", format!("Bearer {}", env::var("OPENAI_API_KEY").unwrap()))
        .json(&params)
        .send()
        .await
        .unwrap();

    res.json::<ChatResponse>().await
}