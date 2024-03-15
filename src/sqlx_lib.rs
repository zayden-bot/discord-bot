use crate::infraction_type::InfractionType;
use crate::models::*;
use chrono::Utc;
use serenity::all::User;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use std::env;

pub async fn get_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("Expected a database url in the environment"))
        .await
        .expect("Failed to connect to database")
}

pub async fn get_support_thead_id(server_id: i64) -> Result<i32, Error> {
    let pool = get_pool().await;

    let result = sqlx::query!(
        "SELECT support_thread_id FROM servers WHERE id = $1",
        server_id
    )
    .fetch_one(&pool)
    .await?;

    pool.close().await;
    Ok(result.support_thread_id)
}

pub async fn post_support_thread_id(server_id: i64, thread_id: i32) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!(
        "INSERT INTO servers (id, support_thread_id) VALUES ($1, $2)",
        server_id,
        thread_id
    )
    .execute(&pool)
    .await?;

    pool.close().await;
    Ok(())
}

pub async fn update_support_thread_id(server_id: i64, thread_id: i32) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!(
        "UPDATE servers SET support_thread_id = $1 WHERE id = $2",
        thread_id,
        server_id
    )
    .execute(&pool)
    .await?;

    pool.close().await;
    Ok(())
}

pub async fn get_support_channel_ids(server_id: i64) -> Result<Vec<i64>, Error> {
    let pool = get_pool().await;

    let results = sqlx::query!(
        "SELECT id FROM channels WHERE guild_id = $1 AND category = 'support'",
        server_id
    )
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>();

    pool.close().await;
    Ok(results)
}

pub async fn get_spoiler_channel_ids(server_id: i64) -> Result<Vec<i64>, Error> {
    let pool = get_pool().await;

    let results = sqlx::query!(
        "SELECT id FROM channels WHERE guild_id = $1 AND category = 'spoiler'",
        server_id
    )
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>();

    pool.close().await;
    Ok(results)
}

pub async fn get_support_role_ids(server_id: i64) -> Result<Vec<i64>, Error> {
    let pool = get_pool().await;

    let results = sqlx::query!(
        "SELECT id FROM roles WHERE guild_id = $1 AND category = 'support'",
        server_id
    )
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>();

    pool.close().await;
    Ok(results)
}

pub async fn get_gold_stars<T>(user_id: T) -> Result<GoldStar, Error>
where
    T: Into<i64>,
{
    let user_id: i64 = user_id.into();

    let pool = get_pool().await;

    let result = sqlx::query_as!(GoldStar, "SELECT * FROM gold_stars WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await?;

    pool.close().await;
    Ok(result)
}

pub async fn create_user(
    user_id: i64,
    given_stars: i32,
    received_stars: i32,
) -> Result<GoldStar, Error> {
    let last_free_star = match given_stars {
        0 => None,
        _ => Some(Utc::now().naive_utc()),
    };

    let pool = get_pool().await;

    let result = sqlx::query_as!(GoldStar, "INSERT INTO gold_stars (id, number_of_stars, given_stars, received_stars, last_free_star) VALUES ($1, $2, $3, $4, $5) RETURNING *", user_id, received_stars, given_stars, received_stars, last_free_star)
        .fetch_one(&pool)
        .await?;

    pool.close().await;
    Ok(result)
}

pub async fn remove_star_from_author(
    user_id: i64,
    stars_to_add: i32,
    last_free_star: bool,
) -> Result<(), Error> {
    let pool = get_pool().await;

    if last_free_star {
        sqlx::query!("UPDATE gold_stars SET given_stars = given_stars + $2, last_free_star = $3 WHERE id = $1", user_id, stars_to_add, Utc::now().naive_utc())
            .execute(&pool)
            .await?;
    } else {
        sqlx::query!("UPDATE gold_stars SET number_of_stars = number_of_stars - $2, given_stars = given_stars + $2 WHERE id = $1", user_id, stars_to_add)
            .execute(&pool)
            .await?;
    }

    pool.close().await;
    Ok(())
}

pub async fn add_star_to_user(user_id: i64, stars_to_add: i32) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!("UPDATE gold_stars SET number_of_stars = number_of_stars + $2, received_stars = received_stars + $2 WHERE id = $1", user_id, stars_to_add)
        .execute(&pool)
        .await?;

    pool.close().await;
    Ok(())
}

pub async fn get_support_answer(server_id: i64, support_id: &str) -> Result<String, Error> {
    let pool = get_pool().await;

    let result = sqlx::query!(
        "SELECT answer FROM support_faq WHERE id = $1 AND guild_id = $2",
        support_id,
        server_id
    )
    .fetch_one(&pool)
    .await?;

    pool.close().await;
    Ok(result.answer)
}

pub async fn get_all_support_faq(server_id: i64) -> Result<Vec<SupportFAQ>, Error> {
    let pool = get_pool().await;

    let results = sqlx::query_as!(
        SupportFAQ,
        "SELECT * FROM support_faq WHERE guild_id = $1",
        server_id
    )
    .fetch_all(&pool)
    .await?;

    pool.close().await;
    Ok(results)
}

pub async fn create_support_faq(
    server_id: i64,
    support_id: &str,
    answer: &str,
) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!(
        "INSERT INTO support_faq (id, answer, guild_id) VALUES ($1, $2, $3)",
        support_id,
        answer,
        server_id
    )
    .execute(&pool)
    .await?;

    pool.close().await;
    Ok(())
}

pub async fn delete_support_faq(server_id: i64, support_id: &str) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!(
        "DELETE FROM support_faq WHERE id = $1 AND guild_id = $2",
        support_id,
        server_id
    )
    .execute(&pool)
    .await?;

    pool.close().await;
    Ok(())
}

pub async fn get_rule(rule_id: &str, guild_id: i64) -> Result<String, Error> {
    let pool = get_pool().await;

    let result = sqlx::query!(
        "SELECT rule_text FROM server_rules WHERE rule_id = $1 AND guild_id = $2",
        rule_id,
        guild_id
    )
    .fetch_one(&pool)
    .await?;

    pool.close().await;
    Ok(result.rule_text)
}

pub async fn get_reaction_roles(guild_id: i64) -> Result<Vec<ReactionRole>, Error> {
    let pool = get_pool().await;

    let results = sqlx::query_as!(
        ReactionRole,
        "SELECT * FROM reaction_roles WHERE guild_id = $1",
        guild_id
    )
    .fetch_all(&pool)
    .await?;

    pool.close().await;
    Ok(results)
}

pub async fn create_reaction_role(
    guild_id: i64,
    channel_id: i64,
    message_id: i64,
    role_id: i64,
    emoji: &str,
) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!("INSERT INTO reaction_roles (guild_id, channel_id, message_id, role_id, emoji) VALUES ($1, $2, $3, $4, $5)", guild_id, channel_id, message_id, role_id, emoji)
        .execute(&pool)
        .await?;

    pool.close().await;
    Ok(())
}

pub async fn delete_reaction_role(
    guild_id: i64,
    channel_id: i64,
    message_id: i64,
    emoji: &str,
) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!("DELETE FROM reaction_roles WHERE guild_id = $1 AND channel_id = $2 AND message_id = $3 AND emoji = $4", guild_id, channel_id, message_id, emoji)
        .execute(&pool)
        .await?;

    pool.close().await;
    Ok(())
}

pub async fn get_user_infractions(user_id: i64) -> Result<Vec<Infraction>, Error> {
    let pool = get_pool().await;

    let results = sqlx::query_as!(
        Infraction,
        "SELECT * FROM infractions WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    pool.close().await;
    Ok(results)
}

pub async fn create_user_infraction(
    user_id: i64,
    username: &str,
    guild_id: i64,
    infraction_type: InfractionType,
    moderator: User,
    points: i32,
    reason: &str,
) -> Result<(), Error> {
    let pool = get_pool().await;

    let moderator_id = moderator.id.get() as i64;
    let moderator_name = moderator.name;

    sqlx::query!("INSERT INTO infractions (user_id, username, guild_id, infraction_type, moderator_id, moderator_username, points, reason) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", user_id, username, guild_id, infraction_type.to_string(), moderator_id, moderator_name, points, reason)
        .execute(&pool)
        .await?;

    pool.close().await;
    Ok(())
}
