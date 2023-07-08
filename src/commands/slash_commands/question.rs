use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::sqlx_lib::{create_question, update_question_message_id};

const QUESTION_CHANNEL_ID: u64 = 829463308629180447;

pub async fn run<'a>(ctx: &Context, interaction: &ApplicationCommandInteraction, mut response: CreateInteractionResponse<'a>) -> CreateInteractionResponse<'a> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            response.interaction_response_data(|message| message.content("This command can only be used in a server"));
            return response;
        }
    };

    let question = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::String(question)) => question,
        _ => {
            response.interaction_response_data(|message| message.content("Invalid question"));
            return response;
        }
    };

    let guild_channels = guild_id.channels(&ctx).await.unwrap();
    let question_channel = match guild_channels.values().find(|channel| channel.id.0 == QUESTION_CHANNEL_ID) {
        Some(question_channel) => question_channel,
        None => {
            response.interaction_response_data(|message| message.content("Error retrieving question channel"));
            return response;
        }
    };

    let question = match create_question(question, interaction.user.id.0 as i64).await {
        Ok(question) => question,
        Err(_) => {
            response.interaction_response_data(|message| message.content("Error creating question"));
            return response;
        }
    };

    let msg = question_channel.send_message(ctx, |message| {
        message.embed(|e| {
            e.field(format!("Question ID: {}", question.id), question.question, false)
        })
    }).await.unwrap();

    if update_question_message_id(question.id, msg.id.0 as i64).await.is_err() {
        response.interaction_response_data(|message| message.content("Error updating question's message ID"));
        return response;
    }

    response.interaction_response_data(|message| message.content("Question created").ephemeral(true));
    response
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("question")
        .description("Ask an anonymous question")
        .create_option(|option| {
            option.name("question")
                .description("The anonymous question to ask")
                .kind(CommandOptionType::String)
                .required(true)
        })
}