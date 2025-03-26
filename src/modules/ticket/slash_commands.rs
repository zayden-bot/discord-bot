use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready, ResolvedOption};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::sqlx_lib::GuildTable;
use crate::{Error, Result};

use super::TicketTable;

pub struct TicketCommand;

#[async_trait]
impl SlashCommand<Error, Postgres> for TicketCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        ticket::TicketCommand::run::<Postgres, GuildTable, TicketTable>(
            ctx,
            interaction,
            pool,
            options,
        )
        .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(ticket::TicketCommand::register())
    }
}

pub struct SupportCommand;

#[async_trait]
impl SlashCommand<Error, Postgres> for SupportCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        ticket::SupportCommand::run::<Postgres, GuildTable>(ctx, interaction, pool, options)
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(ticket::SupportCommand::register())
    }
}
