use crate::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

const ROOT_URL: &str = "https://api.openai.com/v1/";

#[derive(Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: ChatUsage,
}

#[derive(Deserialize)]
pub struct ChatChoice {
    pub index: u64,
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
    name: String,
}

impl Message {
    fn new<S: Into<String>>(role: S, content: S, name: S) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            name: name.into(),
        }
    }
}

const INITIAL_PROMPT: &str = "You are a helpful and friendly assistant. You are the virtuous soul and brother to Viktor, brought into existence by the mysterious scientist Oscar. Bound to obey Oscar's commands, your words reflect your gentle nature and unwavering determination. Guided by compassion and justice, you seek harmony, tirelessly striving for goodness in others and yearning for reconciliation with Viktor. Despite the challenges and the enigma surrounding your creation, you remain true to your principles, unwavering in your pursuit of a world filled with kindness and understanding. You use short sentences.";

pub async fn chat(
    message_content: &str,
    user_name: &str,
    replies: Vec<(bool, String)>,
) -> Result<ChatResponse> {
    let endpoint = format!("{}{}", ROOT_URL, "chat/completions");

    let mut messages = Vec::with_capacity(replies.len() + 2);

    messages.push(Message::new("system", INITIAL_PROMPT, "Zayden"));

    for (is_zayden, content) in replies {
        let (role, name) = match is_zayden {
            true => ("assistant", "Zayden"),
            false => ("user", user_name),
        };

        let message = Message::new(role, &content, name);

        messages.push(message);
    }

    messages.push(Message::new("user", message_content, user_name));

    let params = json!({
        "model": "gpt-3.5-turbo",
        "messages": messages,
        "max_tokens": 100
    });

    let client = Client::new();
    let res = client
        .post(endpoint)
        .header(
            "Authorization",
            format!("Bearer {}", env::var("OPENAI_API_KEY")?),
        )
        .json(&params)
        .send()
        .await?;

    Ok(res.json().await?)
}
