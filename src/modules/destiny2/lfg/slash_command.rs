use async_trait::async_trait;
use serenity::all::{
    AutocompleteOption, CommandInteraction, Context, CreateCommand, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{Autocomplete, SlashCommand};

use crate::{Error, Result};

use super::{LfgGuildTable, LfgPostTable, UsersTable};

pub struct LfgCommand;

#[async_trait]
impl SlashCommand<Error, Postgres> for LfgCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        lfg::LfgCommand::run::<Postgres, LfgGuildTable, LfgPostTable, UsersTable>(
            ctx,
            interaction,
            pool,
        )
        .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(lfg::LfgCommand::register())
    }
}

#[async_trait]
impl Autocomplete<Error, Postgres> for LfgCommand {
    async fn autocomplete(
        ctx: &Context,
        interaction: &CommandInteraction,
        option: AutocompleteOption<'_>,
        _pool: &PgPool,
    ) -> Result<()> {
        lfg::LfgCommand::autocomplete(ctx, interaction, option).await?;

        Ok(())
    }
}
