use std::env;

use serenity::all::{ClientBuilder, GatewayIntents, UserId};
use serenity::prelude::TypeMap;

pub use error::{Error, Result};
use sqlx_lib::PostgresPool;

mod error;
mod handler;
pub mod modules;
mod sqlx_lib;

pub const SUPER_USERS: [UserId; 1] = [
    UserId::new(211486447369322506), // oscarsix
];

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();

    let pool = PostgresPool::init().await.unwrap();

    let mut type_map = TypeMap::new();
    type_map.insert::<PostgresPool>(pool);

    let token = &env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

    let mut client = ClientBuilder::new(token, GatewayIntents::all())
        .type_map(type_map)
        .raw_event_handler(handler::Handler)
        .await
        .unwrap();

    client.start().await.unwrap();

    Ok(())
}
