use async_trait::async_trait;
use serenity::all::{Context, CreateCommand, GuildId, MessageId, Ready};
use slash_commands::{SupportCommand, TicketCommand};
use sqlx::{PgPool, Postgres};
use ticket::{
    support_guild_manager::TicketGuildRow,
    ticket_manager::{TicketManager, TicketRow},
    TicketGuildManager,
};
use zayden_core::SlashCommand;

use crate::sqlx_lib::GuildTable;
use crate::Result;

pub mod components;
pub mod message_commands;
pub mod slash_commands;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![
        TicketCommand::register(ctx, ready)?,
        SupportCommand::register(ctx, ready)?,
    ];

    Ok(commands)
}

pub struct Ticket;

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

pub struct TicketTable;

#[async_trait]
impl TicketManager<Postgres> for TicketTable {
    async fn get(pool: &PgPool, id: impl Into<MessageId> + Send) -> sqlx::Result<TicketRow> {
        let row = sqlx::query_as!(
            TicketRow,
            "SELECT * FROM tickets WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}
