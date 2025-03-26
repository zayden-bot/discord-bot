use async_trait::async_trait;
use endgame_analysis::DestinyPerk;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, Ready,
    ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct Perk;

#[async_trait]
impl SlashCommand<Error, Postgres> for Perk {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        let ResolvedValue::String(perk) = options[0].value else {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Invalid perk name")
                            .ephemeral(true),
                    ),
                )
                .await
                .unwrap();

            return Ok(());
        };

        interaction.defer(ctx).await.unwrap();

        let perk = sqlx::query_as!(
            DestinyPerk,
            "SELECT * FROM destiny_perks WHERE name = $1 LIMIT 1",
            perk
        )
        .fetch_one(pool)
        .await
        .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content(format!("__{}__\n{}", perk.name, perk.description)),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("perk")
            .description("Perk information")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "perk", "Perk name")
                    .required(true),
            );

        Ok(cmd)
    }
}
