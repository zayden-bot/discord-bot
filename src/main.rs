use std::env;

use serenity::{
    all::{GatewayIntents, UserId},
    Client,
};

pub use error::{Error, Result};
use guild_commands::college_kings::{
    goodmorning::GoodMorningLockedUsers, goodnight::GoodNightLockedUsers,
};
use sqlx_lib::PostgresPool;

use crate::image_cache::ImageCache;

mod chatgpt_lib;
pub mod components;
pub mod cron;
mod error;
mod global_commands;
mod guild_commands;
pub mod guilds;
mod handler;
mod image_cache;
pub mod modals;
mod models;
pub mod modules;
mod sqlx_lib;
mod utils;

pub const SERVER_URL: &str = "http://145.40.184.89:8080";
pub const SUPER_USERS: [UserId; 2] = [
    UserId::new(211486447369322506),  // oscarsix
    UserId::new(1287941705861173281), // ck_oscarsix
];

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv()?;

    let token = &env::var("DISCORD_TOKEN")?;
    let pool = PostgresPool::init().await?;

    let mut client = Client::builder(token, GatewayIntents::all())
        .raw_event_handler(handler::Handler)
        .await?;

    let mut data = client.data.write().await;
    data.insert::<ImageCache>(ImageCache::new());
    data.insert::<GoodMorningLockedUsers>(Vec::new());
    data.insert::<GoodNightLockedUsers>(Vec::new());
    data.insert::<PostgresPool>(pool);
    drop(data);

    client.start().await?;

    Ok(())
}
