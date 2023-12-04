mod chatgpt_lib;
mod commands;
mod handler;
mod image_cache;
mod infraction_type;
mod models;
mod sqlx_lib;
mod utils;

use crate::image_cache::ImageCache;
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
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ImageCache>(ImageCache::new());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
