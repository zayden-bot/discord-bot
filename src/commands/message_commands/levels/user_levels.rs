use chrono::NaiveDateTime;
use sqlx::postgres::PgQueryResult;

use crate::sqlx_lib;

pub const LIMIT: i64 = 10;

pub struct UserLevelData {
    pub id: i64,
    pub xp: i32,
    pub level: i32,
    pub total_xp: i32,
    pub message_count: i32,
    pub last_xp: NaiveDateTime,
}

pub async fn get_user_level_data<T: TryInto<i64>>(
    user_id: T,
) -> Result<UserLevelData, sqlx::Error> {
    let pool = sqlx_lib::get_pool().await;

    let user_id: i64 = match user_id.try_into() {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::RowNotFound),
    };

    match sqlx::query_as!(UserLevelData, "SELECT * FROM levels WHERE id = $1", user_id)
        .fetch_optional(&pool)
        .await?
    {
        Some(data) => Ok(data),
        None => {
            sqlx::query_as!(
                UserLevelData,
                "INSERT INTO levels (id) VALUES ($1) RETURNING *",
                user_id,
            )
            .fetch_one(&pool)
            .await
        }
    }
}

pub async fn update_user_level_data(
    user_id: i64,
    xp: i32,
    total_xp: i32,
    level: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    let pool = sqlx_lib::get_pool().await;

    sqlx::query!(
        "UPDATE levels SET xp = $1, total_xp = $2, level = $3, message_count = message_count + 1, last_xp = now() WHERE id = $4",
        xp,
        total_xp,
        level,
        user_id
    )
    .execute(&pool)
    .await
}

pub async fn get_user_rank<T: TryInto<i64>>(user_id: T) -> Result<Option<i64>, sqlx::Error> {
    let pool = sqlx_lib::get_pool().await;

    let user_id: i64 = match user_id.try_into() {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::RowNotFound),
    };

    let rank = sqlx::query!(
        "SELECT rank FROM (SELECT id, RANK() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(rank.rank)
}

pub async fn get_user_row_number<T: TryInto<i64>>(user_id: T) -> Result<Option<i64>, sqlx::Error> {
    let pool = sqlx_lib::get_pool().await;

    let user_id: i64 = match user_id.try_into() {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::RowNotFound),
    };

    let row_number = sqlx::query!(
        "SELECT row_number FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(row_number.row_number)
}

pub async fn get_users<T: Into<i64>>(page: T) -> Result<Vec<UserLevelData>, sqlx::Error> {
    let pool = sqlx_lib::get_pool().await;

    let page: i64 = page.into();

    let offset = (page - 1) * LIMIT;

    sqlx::query_as!(
        UserLevelData,
        "SELECT * FROM levels ORDER BY total_xp DESC LIMIT $1 OFFSET $2",
        LIMIT,
        offset
    )
    .fetch_all(&pool)
    .await
}
