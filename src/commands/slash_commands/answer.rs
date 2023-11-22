use crate::sqlx_lib::update_question_answer;
use crate::utils::{respond_with_ephemeral_message, respond_with_message};
use serenity::all::{
    ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, EditMessage, Permissions,
};

const QUESTION_CHANNEL_ID: u64 = 829463308629180447;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let id = match interaction.data.options[0].value.as_i64() {
        Some(id) => id,
        None => return respond_with_message(ctx, interaction, "Invalid question ID").await,
    };

    let answer = match interaction.data.options[1].value.as_str() {
        Some(answer) => answer,
        _ => return respond_with_message(ctx, interaction, "Invalid answer").await,
    };

    let question = match update_question_answer(id as i32, answer).await {
        Ok(question) => question,
        Err(_) => return respond_with_message(ctx, interaction, "Error updating question").await,
    };

    let question_channel = ChannelId::new(QUESTION_CHANNEL_ID);

    let mut msg = question_channel
        .message(ctx, question.message_id.unwrap() as u64)
        .await
        .unwrap();
    let msg_result = msg
        .edit(
            ctx,
            EditMessage::new().embed(
                CreateEmbed::new()
                    .field(
                        format!("Question ID: {}", question.id),
                        question.question,
                        false,
                    )
                    .field(
                        format!("Answered by {}", interaction.user.name),
                        answer,
                        false,
                    ),
            ),
        )
        .await;

    if msg_result.is_err() {
        return respond_with_message(ctx, interaction, "Error updating question").await;
    }

    respond_with_ephemeral_message(ctx, interaction, "Question answered").await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("answer")
        .description("Answer a user's question")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "question_id",
                "The ID of the question to answer",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "answer",
                "The answer to the question",
            )
            .required(true),
        )
}
