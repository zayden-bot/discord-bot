#![allow(dead_code)]

use serenity::all::{
    CommandInteraction, Context, CreateEmbed, CreateInteractionResponseFollowup, CreateMessage,
    EditInteractionResponse, Message, MessageFlags, ResolvedOption, ResolvedValue,
};
use std::collections::HashMap;

async fn cancel_defer(ctx: &Context, interaction: &CommandInteraction) {
    if interaction
        .get_response(&ctx)
        .await
        .expect("Failed to get response")
        .flags
        .expect("Failed to get flags")
        .contains(MessageFlags::LOADING)
    {
        let _ = message_response(ctx, interaction, "Success").await;
    }
}

pub async fn message_response(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: impl Into<String>,
) -> Result<Message, serenity::Error> {
    interaction
        .edit_response(ctx, EditInteractionResponse::new().content(content))
        .await
}

pub async fn embed_response(
    ctx: &Context,
    interaction: &CommandInteraction,
    embed: CreateEmbed,
) -> Result<Message, serenity::Error> {
    interaction
        .edit_response(ctx, EditInteractionResponse::new().add_embed(embed))
        .await
}

pub async fn send_message(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: impl Into<String>,
) -> Result<Message, serenity::Error> {
    let channel_id = interaction
        .channel
        .as_ref()
        .expect("Only guild commands are supported")
        .id;

    tokio::join!(
        cancel_defer(ctx, interaction),
        channel_id.send_message(ctx, CreateMessage::new().content(content))
    )
    .1
}

pub async fn send_embed(
    ctx: &Context,
    interaction: &CommandInteraction,
    message_builder: CreateMessage,
) -> Result<Message, serenity::Error> {
    let channel_id = interaction
        .channel
        .as_ref()
        .expect("Only guild commands are supported")
        .id;

    tokio::join!(
        cancel_defer(ctx, interaction),
        channel_id.send_message(ctx, message_builder)
    )
    .1
}

pub async fn message_follow_up(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: impl Into<String>,
) -> Result<Message, serenity::Error> {
    interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new().content(content),
        )
        .await
}

pub async fn embed_follow_up(
    ctx: &Context,
    interaction: &CommandInteraction,
    embed: CreateEmbed,
) -> Result<Message, serenity::Error> {
    interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new().add_embed(embed),
        )
        .await
}

pub fn parse_options<'a>(
    options: &'a Vec<ResolvedOption<'_>>,
) -> HashMap<&'a str, &'a ResolvedValue<'a>> {
    let mut parsed_options = HashMap::new();

    for option in options {
        parsed_options.insert(option.name, &option.value);

        // if let ResolvedValue::SubCommand(subcommand) = option.value {
        //     for subcommand_option in subcommand {
        //         parsed_options.insert(subcommand_option.name, subcommand_option.value);
        //     }
        // }
    }

    parsed_options
}
