pub mod component;
pub mod message_command;
pub mod slash_commands;

use chrono::NaiveDateTime;
use futures::{StreamExt, TryStreamExt};
use serenity::all::{Context, CreateCommand, Ready, User, UserId};
use slash_commands::{Rank, Xp};
use sqlx::PgPool;
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub fn register(ctx: &Context, ready: &Ready) -> [CreateCommand; 3] {
    [
        Levels::register(ctx, ready).unwrap(),
        Rank::register(ctx, ready).unwrap(),
        Xp::register(ctx, ready).unwrap(),
    ]
}

pub struct Levels;

pub struct Level {
    pub id: i64,
    pub xp: i32,
    pub level: i32,
    pub total_xp: i32,
    pub message_count: i32,
    pub last_xp: NaiveDateTime,
}

pub async fn get_user_level_data(pool: &PgPool, id: UserId) -> Result<Level> {
    let user_id = id.get() as i64;

    let data = match sqlx::query_as!(Level, "SELECT * FROM levels WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await
        .unwrap()
    {
        Some(data) => data,
        None => sqlx::query_as!(
            Level,
            "INSERT INTO levels (id) VALUES ($1) RETURNING *",
            user_id,
        )
        .fetch_one(pool)
        .await
        .unwrap(),
    };

    Ok(data)
}

pub async fn update_user_level_data(
    pool: &PgPool,
    user_id: UserId,
    xp: i32,
    total_xp: i32,
    level: i32,
) -> Result<()> {
    let user_id = user_id.get() as i64;

    sqlx::query!(
        "UPDATE levels SET xp = $1, total_xp = $2, level = $3, message_count = message_count + 1, last_xp = now() WHERE id = $4",
        xp,
        total_xp,
        level,
        user_id
    )
    .execute(pool)
    .await.unwrap();

    Ok(())
}

pub async fn get_user_rank(pool: &PgPool, user_id: UserId) -> Result<Option<i64>> {
    let user_id = user_id.get() as i64;

    let data = sqlx::query!(
        "SELECT rank FROM (SELECT id, RANK() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await.unwrap();

    Ok(data.rank)
}

pub async fn get_user_row_number(pool: &PgPool, user_id: UserId) -> Result<Option<i64>> {
    let user_id = user_id.get() as i64;

    let data = sqlx::query!(
        "SELECT row_number FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await.unwrap();

    Ok(data.row_number)
}

#[allow(dead_code)]
pub struct UserLevel {
    pub user: User,
    pub xp: i32,
    pub level: i32,
    pub total_xp: i32,
    pub message_count: i32,
    pub last_xp: NaiveDateTime,
}

pub async fn get_users(
    ctx: &Context,
    pool: &PgPool,
    page: i64,
    limit: i64,
) -> Result<Vec<UserLevel>> {
    let offset = (page - 1) * limit;

    let data = sqlx::query_as!(
        Level,
        "SELECT * FROM levels ORDER BY total_xp DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch(pool)
    .then(|level_result| async move {
        let level = level_result.unwrap();

        let userlevel = UserLevel {
            user: UserId::new(level.id as u64).to_user(ctx).await.unwrap(),
            xp: level.xp,
            level: level.level,
            total_xp: level.total_xp,
            message_count: level.message_count,
            last_xp: level.last_xp,
        };

        Ok::<_, Error>(userlevel)
    })
    .try_collect()
    .await?;

    Ok(data)
}
