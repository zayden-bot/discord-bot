use std::env;
use std::ops::Deref;

use serenity::all::Context;
use serenity::prelude::TypeMapKey;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::Result;

#[derive(Clone)]
pub struct PostgresPool {
    pub pool: PgPool,
}

impl PostgresPool {
    pub async fn new() -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .connect(&env::var("DATABASE_URL").unwrap())
            .await?;

        Ok(Self { pool })
    }

    pub async fn get(ctx: &Context) -> PgPool {
        let data = ctx.data.read().await;
        match data.get::<PostgresPool>() {
            Some(pool) => pool.pool.clone(),
            None => {
                drop(data);
                let pool = Self::new().await.unwrap();
                let mut data = ctx.data.write().await;
                data.insert::<PostgresPool>(pool.clone());
                pool.pool
            }
        }
    }
}

impl Deref for PostgresPool {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl TypeMapKey for PostgresPool {
    type Value = PostgresPool;
}

pub struct GuildTable;
