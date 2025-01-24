use async_trait::async_trait;
use serenity::all::GuildId;
use sqlx::{PgPool, Postgres};
use ticket::{support_guild_manager::TicketGuildRow, TicketGuildManager};

use crate::sqlx_lib::GuildTable;

pub mod components;
pub mod message_commands;
pub mod slash_commands;

#[async_trait]
impl TicketGuildManager<Postgres> for GuildTable {
    async fn get(
        pool: &PgPool,
        id: impl Into<GuildId> + Send,
    ) -> sqlx::Result<Option<TicketGuildRow>> {
        let row = sqlx::query_as!(
                TicketGuildRow,
                r#"SELECT id, thread_id, support_channel_id, support_role_ids, faq_channel_id FROM guilds WHERE id = $1"#,
                id.into().get() as i64
            )
            .fetch_optional(pool)
            .await.unwrap();

        Ok(row)
    }

    async fn update_thread_id(pool: &PgPool, id: impl Into<GuildId> + Send) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE guilds SET thread_id = thread_id + 1 WHERE id = $1",
            id.into().get() as i64,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
