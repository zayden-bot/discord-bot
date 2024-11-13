use std::env;

use futures::TryStreamExt;
use serenity::all::Context;
use serenity::all::RoleId;
use serenity::prelude::TypeMapKey;
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::Postgres;
use sqlx::{PgPool, Pool};

use crate::Error;
use crate::Result;

pub mod user_levels;

pub struct PostgresPool;

impl PostgresPool {
    pub async fn init() -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .min_connections(3)
            .connect(&env::var("DATABASE_URL")?)
            .await?;

        Ok(pool)
    }

    pub async fn get(ctx: &Context) -> PgPool {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    }
}

impl TypeMapKey for PostgresPool {
    type Value = PgPool;
}

pub async fn get_support_thead_id(
    pool: &Pool<Postgres>,
    guild_id: impl TryInto<i64>,
) -> Result<i32> {
    let guild_id: i64 = guild_id.try_into().map_err(|_| Error::ConversionError)?;

    let result = sqlx::query!(
        "SELECT support_thread_id FROM servers WHERE id = $1",
        guild_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.support_thread_id)
}

pub async fn update_support_thread_id(
    pool: &Pool<Postgres>,
    guild_id: impl TryInto<i64>,
    thread_id: i32,
) -> Result<PgQueryResult> {
    let result = sqlx::query!(
        "INSERT INTO servers (id, support_thread_id) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET support_thread_id = $2",
        guild_id.try_into().map_err(|_| Error::ConversionError)?,
        thread_id
    ).execute(pool).await?;

    Ok(result)
}

pub async fn get_support_role_ids(
    pool: &Pool<Postgres>,
    guild_id: impl TryInto<i64>,
) -> Result<Vec<RoleId>> {
    let ids = sqlx::query!(
        "SELECT id FROM roles WHERE guild_id = $1 AND category = 'support'",
        guild_id.try_into().map_err(|_| Error::ConversionError)?
    )
    .fetch(pool)
    .map_ok(|x| RoleId::new(x.id as u64))
    .try_collect()
    .await?;

    Ok(ids)
}

pub async fn get_rule(
    pool: &Pool<Postgres>,
    rule_id: &str,
    guild_id: impl TryInto<i64>,
) -> Result<String> {
    todo!()
}
