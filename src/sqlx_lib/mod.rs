pub mod user_levels;

use chrono::Utc;
use futures::TryStreamExt;
use serenity::all::ChannelId;
use serenity::all::Context;
use serenity::all::RoleId;
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgQueryResult;
use sqlx::Pool;
use sqlx::Postgres;

use crate::models::*;
use crate::Error;
use crate::Result;

pub struct PostgresPool;

impl TypeMapKey for PostgresPool {
    type Value = Pool<Postgres>;
}

pub async fn get_pool(ctx: &Context) -> Result<Pool<Postgres>> {
    let data = ctx.data.read().await;
    let pool = data
        .get::<PostgresPool>()
        .expect("PostgresPool should exist in data.")
        .clone();
    drop(data);

    Ok(pool)
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

pub async fn get_support_channel_ids(
    pool: &Pool<Postgres>,
    guild_id: impl TryInto<i64>,
) -> Result<Vec<ChannelId>> {
    let ids = sqlx::query!(
        "SELECT id FROM channels WHERE guild_id = $1 AND category = 'support'",
        guild_id.try_into().map_err(|_| Error::ConversionError)?
    )
    .fetch(pool)
    .map_ok(|x| ChannelId::new(x.id as u64))
    .try_collect()
    .await?;

    Ok(ids)
}

pub async fn get_spoiler_channel_ids(
    pool: &Pool<Postgres>,
    guild_id: impl TryInto<i64>,
) -> Result<Vec<ChannelId>> {
    let ids = sqlx::query!(
        "SELECT id FROM channels WHERE guild_id = $1 AND category = 'spoiler'",
        guild_id.try_into().map_err(|_| Error::ConversionError)?
    )
    .fetch(pool)
    .map_ok(|x| ChannelId::new(x.id as u64))
    .try_collect()
    .await?;

    Ok(ids)
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

pub async fn get_gold_stars(pool: &Pool<Postgres>, user_id: impl TryInto<i64>) -> Result<GoldStar> {
    let user_id: i64 = user_id.try_into().map_err(|_| Error::ConversionError)?;

    let data = sqlx::query_as!(GoldStar, "SELECT * FROM gold_stars WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await?
        .unwrap_or(create_user(pool, user_id, 0, 0).await?);

    Ok(data)
}

pub async fn create_user(
    pool: &Pool<Postgres>,
    user_id: impl TryInto<i64>,
    given_stars: i32,
    received_stars: i32,
) -> Result<GoldStar> {
    let last_free_star = match given_stars {
        0 => None,
        _ => Some(Utc::now().naive_utc()),
    };

    let data = sqlx::query_as!(GoldStar, "INSERT INTO gold_stars (id, number_of_stars, given_stars, received_stars, last_free_star) VALUES ($1, $2, $3, $4, $5) RETURNING *", user_id.try_into().map_err(|_| Error::ConversionError)?, received_stars, given_stars, received_stars, last_free_star)
        .fetch_one(pool)
        .await?;

    Ok(data)
}

pub async fn remove_star_from_author(
    pool: &Pool<Postgres>,
    user_id: impl TryInto<i64>,
    stars_to_add: i32,
    last_free_star: bool,
) -> Result<PgQueryResult> {
    let user_id: i64 = user_id.try_into().map_err(|_| Error::ConversionError)?;

    let result = if last_free_star {
        sqlx::query!("UPDATE gold_stars SET given_stars = given_stars + $2, last_free_star = $3 WHERE id = $1", user_id, stars_to_add, Utc::now().naive_utc())
            .execute(pool)
            .await?
    } else {
        sqlx::query!("UPDATE gold_stars SET number_of_stars = number_of_stars - $2, given_stars = given_stars + $2 WHERE id = $1", user_id, stars_to_add)
            .execute(pool)
            .await?
    };

    Ok(result)
}

pub async fn add_star_to_user(
    pool: &Pool<Postgres>,
    user_id: impl TryInto<i64>,
    stars_to_add: i32,
) -> Result<PgQueryResult> {
    let result = sqlx::query!("UPDATE gold_stars SET number_of_stars = number_of_stars + $2, received_stars = received_stars + $2 WHERE id = $1", user_id.try_into().map_err(|_| Error::ConversionError)?, stars_to_add)
        .execute(pool)
        .await?;

    Ok(result)
}

pub async fn get_rule(
    pool: &Pool<Postgres>,
    rule_id: &str,
    guild_id: impl TryInto<i64>,
) -> Result<String> {
    let data = sqlx::query!(
        "SELECT rule_text FROM server_rules WHERE rule_id = $1 AND guild_id = $2",
        rule_id,
        guild_id.try_into().map_err(|_| Error::ConversionError)?
    )
    .fetch_one(pool)
    .await?;

    Ok(data.rule_text)
}

pub async fn create_reaction_role(
    pool: &Pool<Postgres>,
    guild_id: impl TryInto<i64>,
    channel_id: impl TryInto<i64>,
    message_id: impl TryInto<i64>,
    role_id: impl TryInto<i64>,
    emoji: &str,
) -> Result<PgQueryResult> {
    let result = sqlx::query!("INSERT INTO reaction_roles (guild_id, channel_id, message_id, role_id, emoji) VALUES ($1, $2, $3, $4, $5)",
    guild_id.try_into().map_err(|_| Error::ConversionError)?,
    channel_id.try_into().map_err(|_| Error::ConversionError)?,
    message_id.try_into().map_err(|_| Error::ConversionError)?,
    role_id.try_into().map_err(|_| Error::ConversionError)?,
    emoji)
        .execute(pool)
        .await?;

    Ok(result)
}

pub async fn delete_reaction_role(
    pool: &Pool<Postgres>,
    guild_id: impl TryInto<i64>,
    channel_id: impl TryInto<i64>,
    message_id: impl TryInto<i64>,
    emoji: &str,
) -> Result<PgQueryResult> {
    let result = sqlx::query!("DELETE FROM reaction_roles WHERE guild_id = $1 AND channel_id = $2 AND message_id = $3 AND emoji = $4", guild_id.try_into().map_err(|_| Error::ConversionError)?, channel_id.try_into().map_err(|_| Error::ConversionError)?, message_id.try_into().map_err(|_| Error::ConversionError)?, emoji)
        .execute(pool)
        .await?;

    Ok(result)
}

pub async fn get_user_infractions(
    pool: &Pool<Postgres>,
    user_id: impl TryInto<i64>,
    recent: bool,
) -> Result<Vec<Infraction>> {
    let user_id = user_id.try_into().map_err(|_| Error::ConversionError)?;

    let infractions = if recent {
        sqlx::query_as!(
            Infraction,
            "SELECT * FROM infractions WHERE user_id = $1 AND created_at > CURRENT_DATE - INTERVAL '6 months'",
            user_id
        ).fetch_all(pool).await?
    } else {
        sqlx::query_as!(
            Infraction,
            "SELECT * FROM infractions WHERE user_id = $1",
            user_id
        )
        .fetch_all(pool)
        .await?
    };

    Ok(infractions)
}

pub async fn create_user_infraction(
    pool: &Pool<Postgres>,
    infraction: Infraction,
) -> Result<PgQueryResult> {
    let result = sqlx::query!("INSERT INTO infractions (user_id, username, guild_id, infraction_type, moderator_id, moderator_username, points, reason) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", infraction.user_id, infraction.username, infraction.guild_id, infraction.infraction_type, infraction.moderator_id, infraction.moderator_username, infraction.points, infraction.reason)
        .execute(pool)
        .await?;

    Ok(result)
}
