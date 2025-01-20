use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, EditInteractionResponse, Ready, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{parse_options, SlashCommand};

use crate::guilds::college_kings_team::REVIEW_CHANNEL_ID;
use crate::{Error, Result};

pub struct Review;

#[async_trait]
impl SlashCommand<Error, Postgres> for Review {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let mut options = parse_options(options);

        let team = match options.remove("for") {
            Some(ResolvedValue::String(team)) => team,
            _ => unreachable!("Team is required"),
        };

        let feedback = match options.get("feedback") {
            Some(ResolvedValue::String(feedback)) => *feedback,
            _ => unreachable!("Feedback is required"),
        };

        REVIEW_CHANNEL_ID
            .send_message(
                ctx,
                CreateMessage::new().embed(
                    CreateEmbed::new()
                        .field("For", team, true)
                        .field("Feedback", feedback, false),
                ),
            )
            .await
            .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Feedback submitted!"),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("review")
            .description("Submit feedback for a team")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "for", "The team to review")
                    .add_string_choice("Art", "Art")
                    .add_string_choice("Engineering", "Engineering")
                    .add_string_choice("Leadership", "Leadership")
                    .add_string_choice("Narrative", "Narrative")
                    .add_string_choice("Marketing", "Marketing")
                    .add_string_choice("Transcribing", "Transcribing")
                    .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "feedback",
                    "Feedback for the team",
                )
                .description("Feedback for the team")
                .required(true),
            );

        Ok(command)
    }
}
