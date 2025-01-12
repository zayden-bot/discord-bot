use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, Ready, ResolvedValue,
};
use zayden_core::parse_options;

use crate::guilds::college_kings_team::REVIEW_CHANNEL_ID;
use crate::utils::message_response;
use crate::Result;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    let options = interaction.data.options();
    let options = parse_options(&options);

    let team = match options.get("for") {
        Some(ResolvedValue::String(team)) => *team,
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

    message_response(ctx, interaction, "Feedback submitted!")
        .await
        .unwrap();

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
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
