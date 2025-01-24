use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready, ResolvedOption};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{sqlx_lib::GuildTable, Error, Result};

pub struct FetchSuggestions;

#[async_trait]
impl SlashCommand<Error, Postgres> for FetchSuggestions {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        suggestions::FetchSuggestions::run::<Postgres, GuildTable>(ctx, interaction, options, pool)
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = suggestions::FetchSuggestions::register();

        Ok(command)
    }
}
