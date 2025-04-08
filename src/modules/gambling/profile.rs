use async_trait::async_trait;
use serenity::all::{
    Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, EditInteractionResponse,
    ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::modules::levels::level_up_xp;
use crate::{Error, Result};

use super::{COIN, GamblingAndLevelsRow};

pub struct Profile;

#[async_trait]
impl SlashCommand<Error, Postgres> for Profile {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let row = GamblingAndLevelsRow::from_table(pool, interaction.user.id).await;

        let xp_for_next_rank = level_up_xp(row.level);

        let embed = CreateEmbed::new()
            .title(interaction.user.display_name())
            .field(format!("Cash <:coin:{COIN}>"), row.cash.to_string(), false)
            .field(
                format!("Level {}", row.level),
                format!("{} / {} xp", row.xp, xp_for_next_rank),
                false,
            )
            .colour(Colour::TEAL)
            .thumbnail(interaction.user.avatar_url().unwrap());

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("profile").description("Show your cash, level and items");

        Ok(cmd)
    }
}
