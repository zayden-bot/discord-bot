use async_trait::async_trait;
use gambling::Commands;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::GameTable;
use super::goals::GoalsTable;

pub struct HigherLower;

#[async_trait]
impl SlashCommand<Error, Postgres> for HigherLower {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::higher_lower::<Postgres, GoalsTable, GameTable>(ctx, interaction, pool).await?;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_higher_lower())
    }
}
