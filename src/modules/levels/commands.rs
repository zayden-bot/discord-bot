use async_trait::async_trait;
use levels::Commands;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::LevelsTable;

pub struct Levels;

#[async_trait]
impl SlashCommand<Error, Postgres> for Levels {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        levels::Commands::levels::<Postgres, LevelsTable>(ctx, interaction, pool).await;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        unimplemented!()
    }
}

pub struct Rank;

#[async_trait]
impl SlashCommand<Error, Postgres> for Rank {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::rank::<Postgres, LevelsTable>(ctx, interaction, options, pool).await;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        unimplemented!()
    }
}

pub struct Xp;

#[async_trait]
impl SlashCommand<Error, Postgres> for Xp {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::xp::<Postgres, LevelsTable>(ctx, interaction, options, pool).await;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        unimplemented!()
    }
}
