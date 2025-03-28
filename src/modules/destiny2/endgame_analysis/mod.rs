pub mod database_manager;
pub mod slash_commands;

use async_trait::async_trait;
use endgame_analysis::{DestinyPerk, DestinyPerkManager, DestinyWeapon, DestinyWeaponManager};
use sqlx::{PgPool, Postgres};

pub struct DestinyWeaponTable;

#[async_trait]
impl DestinyWeaponManager<Postgres> for DestinyWeaponTable {
    async fn get(pool: &PgPool, name: &str) -> sqlx::Result<DestinyWeapon> {
        sqlx::query_as!(
            DestinyWeapon,
            "SELECT * FROM destiny_weapons WHERE name = $1",
            name
        )
        .fetch_one(pool)
        .await
    }

    async fn get_by_prefix(pool: &PgPool, name: &str) -> sqlx::Result<Vec<DestinyWeapon>> {
        sqlx::query_as!(
            DestinyWeapon,
            "SELECT * FROM destiny_weapons WHERE name LIKE $1 || '%'",
            name
        )
        .fetch_all(pool)
        .await
    }
}

pub struct DestinyPerkTable;

#[async_trait]
impl DestinyPerkManager<Postgres> for DestinyPerkTable {
    async fn get(pool: &PgPool, name: &str) -> sqlx::Result<DestinyPerk> {
        sqlx::query_as!(
            DestinyPerk,
            "SELECT * FROM destiny_perks WHERE name = $1",
            name
        )
        .fetch_one(pool)
        .await
    }

    async fn get_all(pool: &PgPool, names: &[String]) -> sqlx::Result<Vec<DestinyPerk>> {
        sqlx::query_as!(
            DestinyPerk,
            "SELECT * FROM destiny_perks WHERE name = ANY($1)",
            names
        )
        .fetch_all(pool)
        .await
    }
}
