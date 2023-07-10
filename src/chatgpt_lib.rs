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

#[derive(serde::Serialize)]
struct Message {
    role: String,
    content: String,
    name: String,
}

const INITIAL_PROMPT: &str = "You are a helpful and friendly assistant. You are the virtuous soul and brother to Viktor, brought into existence by the mysterious scientist Oscar. Bound to obey Oscar's commands, your words reflect your gentle nature and unwavering determination. Guided by compassion and justice, you seek harmony, tirelessly striving for goodness in others and yearning for reconciliation with Viktor. Despite the challenges and the enigma surrounding your creation, you remain true to your principles, unwavering in your pursuit of a world filled with kindness and understanding. You use short sentences.";

pub async fn chat(
    message_content: &str,
    user_name: &str,
    replies: Vec<(bool, String)>,
) -> Result<ChatResponse, serde_json::Value> {
    let endpoint = format!("{}{}", ROOT_URL, "chat/completions");

    let mut messages = Vec::with_capacity(replies.len() + 2);

    messages.push(Message {
        role: "system".to_string(),
        content: INITIAL_PROMPT.to_string(),
        name: "Zayden".to_string(),
    });

    for (is_zayden, content) in replies {
        let (role, name) = match is_zayden {
            true => ("assistant", "Zayden"),
            false => ("user", user_name),
        };

        let message = Message {
            role: role.to_string(),
            content,
            name: name.to_string(),
        };

        messages.push(message);
    }

    messages.push(Message {
        role: "user".to_string(),
        content: message_content.to_string(),
        name: user_name.to_string(),
    });

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
            format!("Bearer {}", env::var("OPENAI_API_KEY").unwrap()),
        )
        .json(&params)
        .send()
        .await
        .unwrap();

    let json = res.json::<serde_json::Value>().await.unwrap();
    match serde_json::from_value::<ChatResponse>(json.clone()) {
        Ok(response) => Ok(response),
        Err(_) => Err(json),
    }
}