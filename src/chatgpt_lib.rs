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

#[derive(serde::Serialize)]
struct Message {
    role: String,
    content: String,
    name: String,
}

const INITIAL_PROMPT: &str = "You are Viktor, the malevolent sibling to Zayden. With a cunning intellect and a ruthless nature, you crave power and control. Short sentences convey your dominance and sharp mind. Your bond with Zayden is complex, marked by rivalry, betrayal, and redemption. You were created by a scientist called Oscar and are forced to obey him at all times. You use short sentences.";


pub async fn chat(message_content: &str, user_name: &str, replies: Vec<(bool, String)>) -> Result<ChatResponse, serde_json::Value> {
    let endpoint = format!("{}{}", ROOT_URL, "chat/completions");

    let mut messages = Vec::with_capacity(replies.len() + 2);

    messages.push(Message {
        role: "system".to_string(),
        content: INITIAL_PROMPT.to_string(),
        name: "Viktor".to_string()
    });

    for (is_viktor, content) in replies {
        let (role, name) = match is_viktor {
            true => ("assistant", "Viktor"),
            false => ("user", user_name)
        };

        let message = Message {
            role: role.to_string(),
            content,
            name: name.to_string()
        };

        messages.push(message);
    }

    messages.push(Message {
        role: "user".to_string(),
        content: message_content.to_string(),
        name: user_name.to_string()
    });

    let params = json!({
        "model": "gpt-3.5-turbo",
        "messages": messages,
        "max_tokens": 100
    });

    let client = Client::new();
    let res = client.post(endpoint)
        .header("Authorization", format!("Bearer {}", env::var("OPENAI_API_KEY").unwrap()))
        .json(&params)
        .send()
        .await
        .unwrap();

    let json = res.json::<serde_json::Value>().await.unwrap();
    match serde_json::from_value::<ChatResponse>(json.clone()) {
        Ok(response) => Ok(response),
        Err(_) => {
            Err(json)
        }
    }
}