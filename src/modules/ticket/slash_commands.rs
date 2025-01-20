use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready, ResolvedOption};
use sqlx::{PgPool, Postgres};
use ticket::slash_commands::support::SupportCommand;
use ticket::slash_commands::ticket::TicketCommand;
use zayden_core::SlashCommand;

use crate::sqlx_lib::GuildTable;
use crate::{Error, Result};

#[async_trait]
impl SlashCommand<Error, Postgres> for TicketCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        TicketCommand::run::<Postgres, GuildTable>(ctx, interaction, pool, options).await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(TicketCommand::register())
    }
}

#[async_trait]
impl SlashCommand<Error, Postgres> for SupportCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        SupportCommand::run::<Postgres, GuildTable>(ctx, interaction, pool, options).await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(SupportCommand::register())
    }
}
