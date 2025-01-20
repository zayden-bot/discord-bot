use async_trait::async_trait;
use gold_star::{GiveStar, Stars};
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready, ResolvedOption};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::GoldStarTable;

pub struct GiveStarCommand;

#[async_trait]
impl SlashCommand<Error, Postgres> for GiveStarCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        GiveStar::run::<Postgres, GoldStarTable>(ctx, interaction, options, pool).await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(GiveStar::register())
    }
}

pub struct StarsCommand;

#[async_trait]
impl SlashCommand<Error, Postgres> for StarsCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        Stars::run::<Postgres, GoldStarTable>(ctx, interaction, options, pool).await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Stars::register())
    }
}
