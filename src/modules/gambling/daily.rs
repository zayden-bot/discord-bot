use async_trait::async_trait;
use chrono::Utc;
use serenity::all::{
    Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, EditInteractionResponse,
    ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::{COIN, GamblingRow, GamblingTable, START_AMOUNT};

pub struct Daily;

#[async_trait]
impl SlashCommand<Error, Postgres> for Daily {
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

        let today = Utc::now().naive_utc().date();

        if row.daily == today {
            return Err(Error::DailyClaimed);
        }

        row.cash += START_AMOUNT;
        row.daily = today;

        GamblingTable::save(pool, row).await.unwrap();

        let embed = CreateEmbed::new()
            .description(format!("Collected {START_AMOUNT} <:coin:{COIN}>",))
            .colour(Colour::GOLD);

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("daily").description("Collect your daily cash");

        Ok(cmd)
    }
}
