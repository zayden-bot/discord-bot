mod chatgpt_lib;
mod commands;
mod handler;
mod infraction_type;
mod models;
mod sqlx_lib;
mod utils;

use dotenvy::dotenv;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");

    let token = &env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
