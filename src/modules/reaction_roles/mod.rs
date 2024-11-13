use async_trait::async_trait;
use reaction_roles::reaction_roles_manager::ReactionRole;
use reaction_roles::ReactionRolesManager;
use serenity::all::{Context, CreateCommand, Ready};
use sqlx::any::AnyQueryResult;
use sqlx::{Pool, Postgres};
use zayden_core::SlashCommand;

pub use slash_command::ReactionRoleCommand;

pub mod reaction;
pub mod slash_command;

use crate::Result;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    Ok(vec![ReactionRoleCommand::register(ctx, ready)?])
}

struct ReactionRolesTable;

#[async_trait]
impl ReactionRolesManager<Postgres> for ReactionRolesTable {
    async fn create_row(
        pool: &Pool<Postgres>,
        guild_id: impl Into<i64> + Send,
        channel_id: impl Into<i64> + Send,
        message_id: impl Into<i64> + Send,
        role_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!("INSERT INTO reaction_roles (guild_id, channel_id, message_id, role_id, emoji) VALUES ($1, $2, $3, $4, $5)",
        guild_id.into(),
        channel_id.into(),
        message_id.into(),
        role_id.into(),
        emoji)
            .execute(pool)
            .await?;

        let result = result.into();

        Ok(result)
    }

    async fn get_guild_rows(
        pool: &Pool<Postgres>,
        guild_id: impl Into<i64> + Send,
    ) -> sqlx::Result<Vec<ReactionRole>> {
        let reaction_roles = sqlx::query_as!(
            ReactionRole,
            "SELECT * FROM reaction_roles WHERE guild_id = $1",
            guild_id.into()
        )
        .fetch_all(pool)
        .await?;

        Ok(reaction_roles)
    }

    async fn get_row(
        pool: &Pool<Postgres>,
        message_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<Option<ReactionRole>> {
        let reaction_role = sqlx::query_as!(
            ReactionRole,
            "SELECT * FROM reaction_roles WHERE message_id = $1 AND emoji = $2",
            message_id.into(),
            emoji
        )
        .fetch_optional(pool)
        .await?;

        Ok(reaction_role)
    }

    async fn delete_row(
        pool: &Pool<Postgres>,
        guild_id: impl Into<i64> + Send,
        channel_id: impl Into<i64> + Send,
        message_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!("DELETE FROM reaction_roles WHERE guild_id = $1 AND channel_id = $2 AND message_id = $3 AND emoji = $4", guild_id.into(), channel_id.into(), message_id.into(), emoji)
            .execute(pool)
            .await?;

        Ok(result.into())
    }
}
