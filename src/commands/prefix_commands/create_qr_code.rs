use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[derive(Debug, serde::Deserialize)]
pub struct LovenseQrCodeResponse {
    pub result: bool,
    pub code: i32,
    pub message: String,
    pub data: HashMap<String, String>,
}

pub async fn run(ctx: Context, msg: Message) {
    let token = env::var("LOVENSE_TOKEN").unwrap();

    let data = json!({
        "token": token,
        "uid": &msg.author.name,
        "uname": &msg.author.name,
        "v": "2"
    });

    let client = Client::new();

    match client.post("https://api.lovense.com/api/lan/getQrCode").json(&data).send().await {
        Ok(res) => {
            let response_json = res.json::<LovenseQrCodeResponse>().await.unwrap();
            let qr_code_url = response_json.data.get("qr").unwrap();

            msg.channel_id.say(&ctx, qr_code_url).await.unwrap();
        }
        Err(_) => {}
    };
}
