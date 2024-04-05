mod chatgpt_lib;
pub mod components;
pub mod cron;
mod error;
mod global_commands;
mod guild_commands;
pub mod guilds;
mod handler;
mod image_cache;
mod infraction_type;
pub mod modals;
mod models;
mod sqlx_lib;
mod utils;

use serenity::{all::GatewayIntents, Client};
use std::env;

use crate::image_cache::ImageCache;
pub use error::{Error, Result};
pub use guilds::college_kings;

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
