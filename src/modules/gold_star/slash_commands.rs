use async_trait::async_trait;
use gold_star::commands::{GiveStar, GoldStarCommand, Stars};
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, EditInteractionResponse, Mentionable,
    Ready, ResolvedOption,
};
use sqlx::Postgres;
use zayden_core::SlashCommand;

use crate::sqlx_lib::PostgresPool;
use crate::{Error, Result};

use super::GoldStarTable;

pub struct GiveStarCommand;

#[async_trait]
impl SlashCommand<Error> for GiveStarCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let pool = PostgresPool::get(ctx).await;

        let res = GiveStar::run::<Postgres, GoldStarTable>(interaction, &pool).await?;

        let mut description = format!(
            "{} received a golden star from {} for a total of **{}** stars.",
            res.target_user.mention(),
            interaction.user.mention(),
            res.target_user_stars
        );

        if let Some(reason) = res.reason {
            description.push_str(&format!("\nReason: {}", reason));
        }

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().embed(
                    CreateEmbed::new()
                        .title("⭐ NEW GOLDEN STAR ⭐")
                        .description(description),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(GiveStar::register())
    }
}

pub struct StarsCommand;

#[async_trait]
impl SlashCommand<Error> for StarsCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let pool = PostgresPool::get(ctx).await;

        let (username, row) = Stars::run::<Postgres, GoldStarTable>(interaction, &pool).await?;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().embed(
                    CreateEmbed::new()
                        .title(format!("{}'s Stars", username))
                        .field("Number of Stars", row.number_of_stars.to_string(), true)
                        .field("Given Stars", row.given_stars.to_string(), true)
                        .field("Received Stars", row.received_stars.to_string(), true),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Stars::register())
    }
}
