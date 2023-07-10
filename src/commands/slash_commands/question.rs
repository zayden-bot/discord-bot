use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use crate::sqlx_lib::{create_question, update_question_message_id};
use crate::utils::{respond_with_ephemeral_message, respond_with_message};

const QUESTION_CHANNEL_ID: u64 = 829463308629180447;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let question = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::String(question)) => question,
        _ => return respond_with_message(ctx, interaction, "Invalid question").await,
    };

    let question_channel = ChannelId(QUESTION_CHANNEL_ID);

    let question = match create_question(question, interaction.user.id.0 as i64).await {
        Ok(question) => question,
        Err(_) => return respond_with_message(ctx, interaction, "Error creating question").await,
    };

    let msg = question_channel.send_message(ctx, |message| {
        message.embed(|e| {
            e.field(format!("Question ID: {}", question.id), question.question, false)
        })
    }).await.unwrap();

    if update_question_message_id(question.id, msg.id.0 as i64).await.is_err() {
        return respond_with_message(ctx, interaction, "Error updating question").await;
    }

    respond_with_ephemeral_message(ctx, interaction, "Question asked").await
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