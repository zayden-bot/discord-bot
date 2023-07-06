use crate::models::Image;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use std::env;

async fn get_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("Expected a database url in the environment"))
        .await
        .expect("Failed to connect to database")
}

async fn fetch_images(query: &str) -> Vec<Image> {
    let pool = get_pool().await;

    let result = sqlx::query_as::<_, Image>(query)
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch images");

    pool.close().await;
    result
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