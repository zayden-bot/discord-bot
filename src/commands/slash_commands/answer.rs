use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::Permissions;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::sqlx_lib::update_question_answer;

const QUESTION_CHANNEL_ID: u64 = 829463308629180447;

pub async fn run<'a>(ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            response.interaction_response_data(|message| message.content("This command can only be used in a server"));
            return response;
        }
    };

    let id = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::Integer(id)) => id,
        _ => {
            response.interaction_response_data(|message| message.content("Invalid question ID"));
            return response;
        }
    };

    let answer = match interaction.data.options[1].resolved.as_ref() {
        Some(CommandDataOptionValue::String(answer)) => answer,
        _ => {
            response.interaction_response_data(|message| message.content("Invalid answer"));
            return response;
        }
    };

    let question = match update_question_answer(*id as i32, answer).await {
        Ok(quesiton) => quesiton,
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error updating question"));
            return response;
        }
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
        response.interaction_response_data(|message| message.content("Error updating question message"));
        return response;
    }

    response.interaction_response_data(|message| message.content("Question updated"));
    response
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