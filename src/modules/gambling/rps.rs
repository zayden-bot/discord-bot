use async_trait::async_trait;
use gambling::Commands;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{
    Error, Result,
    modules::gambling::{EffectsTable, GameTable, GoalsTable},
};

pub struct RockPaperScissors;

#[async_trait]
impl SlashCommand<Error, Postgres> for RockPaperScissors {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::rps::<Postgres, GoalsTable, EffectsTable, GameTable>(
            ctx,
            interaction,
            options,
            pool,
        )
        .await?;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_rps())
    }
}
