mod components;
mod modal;
mod slash_command;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use chrono_tz::Tz;
use lfg::timezone_manager::LOCALE_TO_TIMEZONE;
use lfg::{LfgGuildManager, LfgGuildRow, LfgPostManager, LfgPostRow, TimezoneManager};
use serenity::all::{ChannelId, GuildId, MessageId, RoleId, UserId};
use sqlx::any::AnyQueryResult;
use sqlx::{PgPool, Pool, Postgres};

pub use components::LfgComponents;
pub use modal::{LfgCreateModal, LfgEditModal};
pub use slash_command::LfgCommand;

struct LfgGuildTable;

#[async_trait]
impl LfgGuildManager<Postgres> for LfgGuildTable {
    async fn get(
        pool: &PgPool,
        id: impl Into<GuildId> + Send,
    ) -> sqlx::Result<Option<LfgGuildRow>> {
        let guild = sqlx::query_as!(
            LfgGuildRow,
            "SELECT * FROM lfg_guilds WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_optional(pool)
        .await?;

        Ok(guild)
    }

    async fn save(
        pool: &PgPool,
        id: impl Into<GuildId> + Send,
        channel: impl Into<ChannelId> + Send,
        role: Option<impl Into<RoleId> + Send>,
    ) -> sqlx::Result<AnyQueryResult> {
        let id = id.into().get() as i64;
        let channel = channel.into().get() as i64;
        let role = role.map(|r| r.into().get() as i64);

        let result = sqlx::query!(
            "INSERT INTO lfg_guilds (id, channel_id, role_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (id)
            DO UPDATE SET channel_id = EXCLUDED.channel_id, role_id = EXCLUDED.role_id;",
            id,
            channel,
            role
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }
}

struct LfgPostTable;

#[async_trait]
impl LfgPostManager<Postgres> for LfgPostTable {
    async fn get(
        pool: &Pool<Postgres>,
        id: impl Into<MessageId> + Send,
    ) -> sqlx::Result<LfgPostRow> {
        let post = sqlx::query_as!(
            LfgPostRow,
            "SELECT * FROM lfg_posts WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(post)
    }

    async fn save(
        pool: &Pool<Postgres>,
        id: impl Into<i64> + Send,
        owner: impl Into<i64> + Send,
        activity: &str,
        timestamp: NaiveDateTime,
        timezone: &str,
        description: &str,
        fireteam_size: impl Into<i16> + Send,
        fireteam: &[i64],
        alternatives: &[i64],
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            "INSERT INTO lfg_posts (id, owner_id, activity, timestamp, timezone, description, fireteam_size, fireteam, alternatives)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id)
            DO UPDATE SET owner_id = EXCLUDED.owner_id,
                          activity = EXCLUDED.activity,
                          timestamp = EXCLUDED.timestamp,
                          timezone = EXCLUDED.timezone,
                          description = EXCLUDED.description,
                          fireteam_size = EXCLUDED.fireteam_size,
                          fireteam = EXCLUDED.fireteam,
                          alternatives = EXCLUDED.alternatives;",
            id.into(),
            owner.into(),
            activity,
            timestamp,
            timezone,
            description,
            fireteam_size.into(),
            fireteam,
            alternatives
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }

    async fn delete(
        pool: &Pool<Postgres>,
        id: impl Into<MessageId> + Send,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            "DELETE FROM lfg_posts WHERE id = $1",
            id.into().get() as i64
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }
}

struct UsersTable;

#[async_trait]
impl TimezoneManager<Postgres> for UsersTable {
    async fn get(
        pool: &Pool<Postgres>,
        id: impl Into<UserId> + Send,
        local: &str,
    ) -> sqlx::Result<Tz> {
        let tz = sqlx::query!(
            "SELECT timezone FROM lfg_users WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_optional(pool)
        .await?;

        match tz {
            Some(tz) => Ok(tz.timezone.parse().unwrap_or(chrono_tz::UTC)),
            None => Ok(LOCALE_TO_TIMEZONE
                .get(local)
                .unwrap_or(&chrono_tz::UTC)
                .to_owned()),
        }
    }

    async fn save(
        pool: &Pool<Postgres>,
        id: impl Into<UserId> + Send,
        tz: Tz,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            "INSERT INTO lfg_users (id, timezone) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET timezone = $2",
            id.into().get() as i64,
            tz.name()
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }
}
