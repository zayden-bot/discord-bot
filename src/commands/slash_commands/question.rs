use crate::sqlx_lib::{create_question, update_question_message_id};
use crate::utils::{respond_with_ephemeral_message, respond_with_message};
use serenity::all::{
    ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage,
};

const QUESTION_CHANNEL_ID: u64 = 829463308629180447;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let question = match interaction.data.options[0].value.as_str() {
        Some(question) => question,
        None => return respond_with_message(ctx, interaction, "Invalid question").await,
    };

    let question_channel = ChannelId::new(QUESTION_CHANNEL_ID);

    let question = match create_question(question, interaction.user.id.get() as i64).await {
        Ok(question) => question,
        Err(_) => return respond_with_message(ctx, interaction, "Error creating question").await,
    };

    let msg = question_channel
        .send_message(
            ctx,
            CreateMessage::new().add_embed(CreateEmbed::new().field(
                format!("Question ID: {}", question.id),
                question.question,
                false,
            )),
        )
        .await
        .unwrap();

    if update_question_message_id(question.id, msg.id.get() as i64)
        .await
        .is_err()
    {
        return respond_with_message(ctx, interaction, "Error updating question").await;
    }

    respond_with_ephemeral_message(ctx, interaction, "Question asked").await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("question")
        .description("Ask an anonymous question")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "question",
                "The anonymous question to ask",
            )
            .required(true),
        )
}
