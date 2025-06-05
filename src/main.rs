use std::env;

use endgame_analysis::endgame_analysis::EndgameAnalysisSheet;
use modules::destiny2::endgame_analysis::DestinyWeaponTable;
use modules::destiny2::endgame_analysis::database_manager::DestinyDatabaseManager;
use serenity::all::{ClientBuilder, GatewayIntents, GuildId, UserId};
use serenity::prelude::TypeMap;

pub use error::{Error, Result};
use sqlx::Postgres;
use sqlx_lib::PostgresPool;

mod cron;
mod error;
mod handler;
pub mod modules;
mod sqlx_lib;

pub const SUPER_USERS: [UserId; 1] = [
    UserId::new(211486447369322506), // oscarsix
];
pub const BRADSTER_GUILD: GuildId = GuildId::new(1255957182457974875);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();

    let pool = PostgresPool::new().await.unwrap();

    if !cfg!(debug_assertions) {
        DestinyDatabaseManager::update_dbs(&pool.pool)
            .await
            .unwrap();
        EndgameAnalysisSheet::update::<Postgres, DestinyWeaponTable>(&pool.pool)
            .await
            .unwrap();
    }

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
