use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready, ResolvedOption};
use sqlx::Postgres;
use ticket::slash_commands::support::SupportCommand;
use ticket::slash_commands::ticket::TicketCommand;
use zayden_core::SlashCommand;

use crate::sqlx_lib::{GuildTable, PostgresPool};
use crate::{Error, Result};

#[async_trait]
impl SlashCommand<Error> for TicketCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        TicketCommand::run::<Postgres, GuildTable>(ctx, interaction, &pool, options).await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(TicketCommand::register())
    }
}

#[async_trait]
impl SlashCommand<Error> for SupportCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        SupportCommand::run::<Postgres, GuildTable>(ctx, interaction, &pool, options).await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(SupportCommand::register())
    }
}
