use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::sqlx_lib::update_question_answer;
use crate::utils::respond_with_message;

const QUESTION_CHANNEL_ID: u64 = 829463308629180447;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let id = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::Integer(id)) => id,
        _ => return respond_with_message(ctx, interaction, "Invalid question ID").await,
    };

    let answer = match interaction.data.options[1].resolved.as_ref() {
        Some(CommandDataOptionValue::String(answer)) => answer,
        _ => return respond_with_message(ctx, interaction, "Invalid answer").await,
    };

    let question = match update_question_answer(*id as i32, answer).await {
        Ok(question) => question,
        Err(_) => return respond_with_message(ctx, interaction, "Error updating question").await,
    };

    let guild_channels = guild_id.channels(&ctx).await.unwrap();
    let question_channel = guild_channels.values().find(|channel| channel.id.0 == QUESTION_CHANNEL_ID).unwrap();

    let mut msg = question_channel.message(ctx, question.message_id.unwrap() as u64).await.unwrap();
    let msg_result = msg.edit(ctx, |message| {
        message.embed(|e| {
            e.field(format!("Question ID: {}", question.id), question.question, false)
                .field(format!("Answered by {}", interaction.user.name), answer, false)
        })
    }).await;

    if msg_result.is_err() {
        return respond_with_message(ctx, interaction, "Error updating question").await;
    }

    respond_with_message(ctx, interaction, "Question answered").await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("answer")
        .description("Answer a user's question")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .create_option(|option| {
            option.name("question_id")
                .description("The ID of the question to answer")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option.name("answer")
                .description("The answer to the question")
                .kind(CommandOptionType::String)
                .required(true)
        })
}