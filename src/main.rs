mod chatgpt_lib;
mod commands;
pub mod components;
pub mod cron;
mod error;
mod handler;
mod image_cache;
mod infraction_type;
pub mod modals;
mod models;
mod sqlx_lib;
mod utils;

use crate::image_cache::ImageCache;
pub use error::{Error, Result};
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

pub const COLLEGE_KINGS_GUILD_ID: u64 = 745662812335898806;

pub const SERVER_IP: &str = "82.9.123.190";
pub const SERVER_URL: &str = "http://82.9.123.190";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let token = &env::var("DISCORD_TOKEN")?;

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
