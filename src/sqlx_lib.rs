use std::env;

use serenity::all::Context;
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::Result;

pub struct PostgresPool;

impl PostgresPool {
    pub async fn init() -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .connect(&env::var("DATABASE_URL").unwrap())
            .await?;

        Ok(pool)
    }

    pub async fn get(ctx: &Context) -> PgPool {
        let data = ctx.data.read().await;
        match data.get::<PostgresPool>() {
            Some(pool) => pool.clone(),
            None => {
                let pool = Self::init().await.unwrap();
                drop(data);
                let mut data = ctx.data.write().await;
                data.insert::<PostgresPool>(pool.clone());
                pool
            }
        }
    }
}

impl TypeMapKey for PostgresPool {
    type Value = PgPool;
}

pub struct GuildTable;
