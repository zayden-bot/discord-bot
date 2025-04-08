use async_trait::async_trait;
use serenity::all::{
    Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, EditInteractionResponse,
    Mentionable, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::GamblingTable;

pub struct Leaderboard;

#[async_trait]
impl SlashCommand<Error, Postgres> for Leaderboard {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let rows = GamblingTable::guild_leaderboard(pool, 1).await.unwrap();

        let desc = rows
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let place = if i == 0 {
                    "🥇".to_string()
                } else if i == 1 {
                    "🥈".to_string()
                } else if i == 2 {
                    "🥉".to_string()
                } else {
                    format!("#{}", i + 1)
                };

                format!("{} - {} - {}", place, row.user_id().mention(), row.cash)
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let embed = CreateEmbed::new()
            .title("🏁 Leaderboard")
            .description(desc)
            .colour(Colour::TEAL);

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("leaderboard").description("Total Cash Leaderboard");

        Ok(cmd)
    }
}
