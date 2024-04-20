use crate::utils::{message_response, parse_options};
use crate::Result;
use serenity::all::{
    ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, ResolvedValue,
};

const CHANNEL_ID: ChannelId = ChannelId::new(1227244734515380316);

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer_ephemeral(ctx).await?;

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

    CHANNEL_ID
        .send_message(
            ctx,
            CreateMessage::new().embed(
                CreateEmbed::new()
                    .field("For", team, true)
                    .field("Feedback", feedback, false),
            ),
        )
        .await?;

    message_response(ctx, interaction, "Feedback submitted!").await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("review")
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
        )
}
