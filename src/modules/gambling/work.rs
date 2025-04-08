use async_trait::async_trait;
use chrono::{Duration, Utc};
use serenity::all::{
    Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, EditInteractionResponse,
    ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::{COIN, GamblingRow, GamblingTable};

pub struct Work;

#[async_trait]
impl SlashCommand<Error, Postgres> for Work {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut row = GamblingTable::get(pool, interaction.user.id)
            .await
            .unwrap()
            .unwrap_or_else(|| GamblingRow::new(interaction.user.id));

        let now = Utc::now().naive_utc();
        let break_over = row.work + Duration::minutes(10);

        if break_over >= now {
            return Err(Error::work_claimed(break_over.and_utc().timestamp()));
        }

        let amount = rand::random_range(100..=500);
        row.cash += amount;
        row.work = now;

        GamblingTable::save(pool, row).await.unwrap();

        let embed = CreateEmbed::new()
            .description(format!("Collected {amount} <:coin:{COIN}> for working"))
            .colour(Colour::GOLD);

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("work").description("Do some work and get some quick cash");

        Ok(cmd)
    }
}
