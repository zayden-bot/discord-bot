use crate::models::Image;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
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

    sqlx::query_as::<_, Image>(query)
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch images")
}

pub async fn get_good_morning_images() -> Vec<Image> {
    fetch_images("SELECT * FROM good_morning_images").await
}

pub async fn get_good_night_images() -> Vec<Image> {
    fetch_images("SELECT * FROM good_night_images").await
}
