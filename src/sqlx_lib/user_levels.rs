use chrono::NaiveDateTime;
use futures::{StreamExt, TryStreamExt};
use serenity::all::{Context, User, UserId};
use sqlx::{Pool, Postgres};

use crate::{Error, Result};
use crate::sqlx_lib::PostgresPool;

pub struct Level {
    pub id: i64,
    pub xp: i32,
    pub level: i32,
    pub total_xp: i32,
    pub message_count: i32,
    pub last_xp: NaiveDateTime,
}

pub async fn get_user_level_data(
    pool: &Pool<Postgres>,
    user_id: impl TryInto<i64>,
) -> Result<Level> {
    let user_id: i64 = user_id.try_into().map_err(|_| Error::ConversionError)?;

    let data = match sqlx::query_as!(Level, "SELECT * FROM levels WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await?
    {
        Some(data) => data,
        None => {
            sqlx::query_as!(
                Level,
                "INSERT INTO levels (id) VALUES ($1) RETURNING *",
                user_id,
            )
            .fetch_one(pool)
            .await?
        }
    };

    Ok(data)
}

pub async fn update_user_level_data(
    pool: &Pool<Postgres>,
    user_id: impl TryInto<i64>,
    xp: i32,
    total_xp: i32,
    level: i32,
) -> Result<()> {
    sqlx::query!(
        "UPDATE levels SET xp = $1, total_xp = $2, level = $3, message_count = message_count + 1, last_xp = now() WHERE id = $4",
        xp,
        total_xp,
        level,
        user_id.try_into().map_err(|_| Error::ConversionError)?
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_user_rank(
    pool: &Pool<Postgres>,
    user_id: impl TryInto<i64>,
) -> Result<Option<i64>> {
    let data = sqlx::query!(
        "SELECT rank FROM (SELECT id, RANK() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
        user_id.try_into().map_err(|_| Error::ConversionError)?
    )
    .fetch_one(pool)
    .await?;

    Ok(data.rank)
}

pub async fn get_user_row_number(
    pool: &Pool<Postgres>,
    user_id: impl TryInto<i64>,
) -> Result<Option<i64>> {
    let data = sqlx::query!(
        "SELECT row_number FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
        user_id.try_into().map_err(|_| Error::ConversionError)?
    )
    .fetch_one(pool)
    .await?;

    Ok(data.row_number)
}

pub struct UserLevel {
    pub user: User,
    pub xp: i32,
    pub level: i32,
    pub total_xp: i32,
    pub message_count: i32,
    pub last_xp: NaiveDateTime,
}

pub async fn get_users(ctx: &Context, page: i64, limit: i64) -> Result<Vec<UserLevel>> {
    let pool = PostgresPool::get(ctx).await;

    let offset = (page - 1) * limit;

    let data = sqlx::query_as!(
        Level,
        "SELECT * FROM levels ORDER BY total_xp DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch(&pool)
    .then(|level_result| async move {
        let level = level_result?;

        let userlevel = UserLevel {
            user: UserId::new(level.id as u64).to_user(ctx).await?,
            xp: level.xp,
            level: level.level,
            total_xp: level.total_xp,
            message_count: level.message_count,
            last_xp: level.last_xp,
        };

        Ok::<UserLevel, Error>(userlevel)
    })
    .try_collect()
    .await?;

    Ok(data)
}
