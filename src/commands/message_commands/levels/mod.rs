mod user_level_data;

use chrono::{Duration, Utc};
use rand::Rng;
use serenity::all::{Context, Message};
use sqlx::postgres::PgQueryResult;

use crate::sqlx_lib;

use user_level_data::UserLevelData;

const BLOCKED_CHANNEL_IDS: [u64; 1] = [776139754408247326];

pub async fn run(_ctx: &Context, msg: &Message) {
    if msg.guild_id.is_none() {
        return;
    }

    let channel_id = msg.channel_id;

    if BLOCKED_CHANNEL_IDS.contains(&channel_id.get()) {
        return;
    }

    let level_data = match get_user_level_data(msg.author.id.get()).await {
        Ok(data) => data,
        Err(why) => {
            println!("Cannot get user level data: {}", why);
            return;
        }
    };

    if level_data.last_xp >= (Utc::now().naive_utc() - Duration::minutes(1)) {
        return;
    }

    let xp_to_add = rand::thread_rng().gen_range(15..25);

    if let Err(why) = update_user_level_data(level_data.id, xp_to_add).await {
        println!("Cannot update user level data: {}", why);
    }
}

async fn get_user_level_data<T: TryInto<i64>>(user_id: T) -> Result<UserLevelData, sqlx::Error> {
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

async fn update_user_level_data(
    user_id: i64,
    xp_to_add: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    let pool = sqlx_lib::get_pool().await;

    sqlx::query!(
        "UPDATE levels SET total_xp = total_xp + $1, last_xp = now() WHERE id = $2",
        xp_to_add,
        user_id
    )
    .execute(&pool)
    .await
}
// XP for next level=5×(level)2+50×(level)+100
