use std::env;

use serenity::all::{ClientBuilder, GatewayIntents, UserId};
use serenity::prelude::TypeMap;

pub use error::{Error, Result};
use guild_commands::college_kings::{
    goodmorning::GoodMorningLockedUsers, goodnight::GoodNightLockedUsers,
};
use sqlx_lib::PostgresPool;

use crate::image_cache::ImageCache;

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
    dotenvy::dotenv().unwrap();

    let pool = PostgresPool::init().await.unwrap();

    let mut type_map = TypeMap::new();
    type_map.insert::<ImageCache>(ImageCache::new());
    type_map.insert::<GoodMorningLockedUsers>(Vec::new());
    type_map.insert::<GoodNightLockedUsers>(Vec::new());
    type_map.insert::<PostgresPool>(pool);

    let token = &env::var("DISCORD_TOKEN")?;

    let mut client = ClientBuilder::new(token, GatewayIntents::all())
        .type_map(type_map)
        .raw_event_handler(handler::Handler)
        .await
        .unwrap();

    client.start().await.unwrap();

    Ok(())
}
