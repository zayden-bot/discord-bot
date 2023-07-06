use crate::models::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use std::env;
use chrono::Utc;

async fn get_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("Expected a database url in the environment"))
        .await
        .expect("Failed to connect to database")
}

async fn fetch_images(query: &str) -> Vec<Image> {
    let pool = get_pool().await;

    let results = sqlx::query_as::<_, Image>(query)
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch images");

    pool.close().await;
    results
}

pub async fn get_good_morning_images() -> Vec<Image> {
    fetch_images("SELECT * FROM good_morning_images").await
}

pub async fn get_good_night_images() -> Vec<Image> {
    fetch_images("SELECT * FROM good_night_images").await
}

pub async fn get_support_thead_id(server_id: i64) -> Result<i32, Error> {
    let pool = get_pool().await;

    let result = sqlx::query!("SELECT support_thread_id FROM servers WHERE id = $1", server_id)
        .fetch_one(&pool)
        .await?;

    pool.close().await;
    Ok(result.support_thread_id)
}

pub async fn post_support_thread_id(server_id: i64, thread_id: i32) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!("INSERT INTO servers (id, support_thread_id) VALUES ($1, $2)", server_id, thread_id)
        .execute(&pool)
        .await?;

    pool.close().await;
    Ok(())
}

pub async fn update_support_thread_id(server_id: i64, thread_id: i32) -> Result<(), Error> {
    let pool = get_pool().await;

    sqlx::query!("UPDATE servers SET support_thread_id = $1 WHERE id = $2", thread_id, server_id)
        .execute(&pool)
        .await?;

    pool.close().await;
    Ok(())
}

pub async fn get_support_channel_ids(server_id: &i64) -> Result<Vec<i64>, Error> {
    let pool = get_pool().await;

    let results = sqlx::query!("SELECT id FROM channels WHERE guild_id = $1 AND category = 'support'", server_id)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|record| record.id)
        .collect::<Vec<i64>>();

    pool.close().await;
    Ok(results)
}

pub async fn get_support_role_ids(server_id: &i64) -> Result<Vec<i64>, Error> {
    let pool = get_pool().await;

    let results = sqlx::query!("SELECT id FROM roles WHERE guild_id = $1 AND category = 'support'", server_id)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|record| record.id)
        .collect::<Vec<i64>>();

    pool.close().await;
    Ok(results)
}

pub async fn get_gold_stars(user_id: i64) -> Result<GoldStar, Error> {
    let pool = get_pool().await;

    let result = sqlx::query!("SELECT * FROM gold_stars WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await?;

    pool.close().await;
    Ok(GoldStar {id: result.id, number_of_stars: result.number_of_stars, given_stars: result.given_stars, received_stars: result.received_stars, last_free_star: result.last_free_star})
}

pub async fn create_user(user_id: i64, given_stars: i32, received_stars: i32) -> Result<GoldStar, Error> {
    let last_free_star = match given_stars {
        0 => None,
        _ => Some(Utc::now().naive_utc()),
    } ;

    let gold_star = GoldStar {id: user_id, number_of_stars: received_stars, given_stars, received_stars, last_free_star };

    let pool = get_pool().await;

    sqlx::query!("INSERT INTO gold_stars (id, number_of_stars, given_stars, received_stars, last_free_star) VALUES ($1, $2, $3, $4, $5)", &gold_star.id, &gold_star.number_of_stars, &gold_star.given_stars, &gold_star.received_stars, gold_star.last_free_star)
        .execute(&pool)
        .await?;

    pool.close().await;
    Ok(gold_star)
}

pub async fn remove_star_from_author(user_id: i64, stars_to_add: i32, last_free_star: bool) -> Result<(), Error> {
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