mod commands;
mod handler;
mod models;
mod sqlx_lib;
mod chatgpt_lib;

use dotenvy::dotenv;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = &env::var("ZAYDEN_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
