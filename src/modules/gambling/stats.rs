use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, EditInteractionResponse,
    ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::sqlx_lib::GuildTable;
use crate::{Error, Result};

use super::COIN;

pub struct Stats;

#[async_trait]
impl SlashCommand<Error, Postgres> for Stats {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let row = GuildTable::get(pool, interaction.guild_id.unwrap())
            .await
            .unwrap()
            .unwrap();

        let embed = CreateEmbed::new().description(format!(
            "Total gained coins: {} <:coin:{COIN}>\nTotal lost coins: {} <:coin:{COIN}>\nNET: {:+}",
            row.gambling_gain,
            row.gambling_lost,
            row.gambling_gain - row.gambling_lost
        ));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("stats").description("See gambling stats");

        Ok(cmd)
    }
}
