pub mod component;
pub mod message_command;
pub mod slash_commands;

use chrono::NaiveDateTime;
use serenity::all::{Context, CreateCommand, User, UserId};
use sqlx::{PgPool, any::AnyQueryResult};
use zayden_core::SlashCommand;

#[inline(always)]
pub const fn level_up_xp(level: i32) -> i32 {
    (5 * level * level) + (50 * level) + 100
}

pub fn register(ctx: &Context) -> CreateCommand {
    Levels::register(ctx).unwrap()
}

pub struct Levels;

pub struct LevelsTable;

impl LevelsTable {
    pub async fn get_user_rank(
        pool: &PgPool,
        user_id: impl Into<UserId>,
    ) -> sqlx::Result<Option<i64>> {
        let user_id = user_id.into().get() as i64;

        let data = sqlx::query!(
            "SELECT rank FROM (SELECT id, RANK() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(data.rank)
    }

    pub async fn get_user_row_number(
        pool: &PgPool,
        user_id: impl Into<UserId>,
    ) -> sqlx::Result<Option<i64>> {
        let user_id = user_id.into().get() as i64;

        let data = sqlx::query!(
            "SELECT row_number FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(data.row_number)
    }

    pub async fn get_users(pool: &PgPool, page: i64, limit: i64) -> sqlx::Result<Vec<LevelsRow>> {
        let offset = (page - 1) * limit;

        let data = sqlx::query_as!(
            LevelsRow,
            "SELECT * FROM levels ORDER BY total_xp DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }
}

pub struct LevelsRow {
    pub id: i64,
    pub xp: i32,
    pub level: i32,
    pub total_xp: i32,
    pub message_count: i32,
    pub last_xp: NaiveDateTime,
}

impl LevelsRow {
    pub fn new(id: impl Into<UserId>) -> Self {
        let id = id.into().get() as i64;

        Self {
            id,
            xp: 0,
            level: 1,
            total_xp: 0,
            message_count: 0,
            last_xp: NaiveDateTime::default(),
        }
    }

    pub async fn from_table(pool: &PgPool, id: impl Into<UserId>) -> sqlx::Result<Self> {
        let id = id.into().get() as i64;

        let row = sqlx::query_as!(LevelsRow, "SELECT * FROM levels WHERE id = $1", id)
            .fetch_optional(pool)
            .await?
            .unwrap_or_else(|| LevelsRow::new(id as u64));

        Ok(row)
    }

    pub fn id(&self) -> UserId {
        UserId::new(self.id as u64)
    }

    pub async fn as_user(&self, ctx: &Context) -> serenity::Result<User> {
        self.id().to_user(ctx).await
    }

    pub fn update_level(&mut self) -> bool {
        let next_level_xp = level_up_xp(self.level);

        let rand_xp = rand::random_range(15..25);
        self.total_xp += rand_xp;
        self.xp += rand_xp;

        if self.xp >= next_level_xp {
            self.xp -= next_level_xp;
            self.level += 1;
            return true;
        };

        false
    }

    pub async fn save(self, pool: &PgPool) -> sqlx::Result<AnyQueryResult> {
        let r = sqlx::query!(
            "UPDATE levels SET xp = $2, total_xp = $3, level = $4, message_count = message_count + 1, last_xp = now() WHERE id = $1",
            self.id,
            self.xp,
            self.total_xp,
            self.level,
        )
        .execute(pool)
        .await?;

        Ok(r.into())
    }
}
