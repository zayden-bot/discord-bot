mod chatgpt_lib;
mod commands;
pub mod error;
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

pub use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let token = &env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(handler::Handler)
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<ImageCache>(ImageCache::new());
    }

    client.start().await?;

    Ok(())
}
