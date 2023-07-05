use reqwest::Client;
use serde_json::json;
use std::env;

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
    pub content: String,
}

#[derive(serde::Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

pub async fn chat(message_content: &str, user_name: &str) -> reqwest::Result<ChatResponse> {
    let endpoint = format!("{}{}", ROOT_URL, "chat/completions");

    let intial_prompt = "You are the virtuous soul and brother to Viktor, brought into existence by the mysterious scientist Oscar. Bound to obey Oscar's commands, your words reflect your gentle nature and unwavering determination. Guided by compassion and justice, you seek harmony, tirelessly striving for goodness in others and yearning for reconciliation with Viktor. Despite the challenges and the enigma surrounding your creation, you remain true to your principles, unwavering in your pursuit of a world filled with kindness and understanding. You use short sentances.";

    let params = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {"role": "system", "content": intial_prompt, "name": "Zayden"},
            {"role": "user", "content": message_content, "name": user_name}
        ],
        "max_tokens": 100
    });

    let client = Client::new();
    let res = client
        .post(endpoint)
        .header(
            "Authorization",
            format!("Bearer {}", env::var("OPENAI_API_KEY").unwrap()),
        )
        .json(&params)
        .send()
        .await?;

    res.json::<ChatResponse>().await
}
