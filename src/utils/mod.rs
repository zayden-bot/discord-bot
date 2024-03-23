#![allow(dead_code)]

pub mod support;

use serenity::all::{
    CommandInteraction, Context, CreateEmbed, CreateInteractionResponseFollowup, CreateMessage,
    EditInteractionResponse, Message, MessageFlags, ResolvedOption, ResolvedValue,
};
use std::collections::HashMap;

use crate::{Error, Result};

async fn cancel_defer(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    if let Some(flags) = interaction.get_response(&ctx).await?.flags {
        if flags.contains(MessageFlags::LOADING) {
            message_response(ctx, interaction, "Success").await?;
        }
    }

    Ok(())
}

pub async fn message_response(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: impl Into<String>,
) -> Result<Message> {
    let message = interaction
        .edit_response(ctx, EditInteractionResponse::new().content(content))
        .await?;

    Ok(message)
}

pub async fn embed_response(
    ctx: &Context,
    interaction: &CommandInteraction,
    embed: CreateEmbed,
) -> Result<Message> {
    let message = interaction
        .edit_response(ctx, EditInteractionResponse::new().add_embed(embed))
        .await?;

    Ok(message)
}

pub async fn send_message(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: impl Into<String>,
) -> Result<Message> {
    let channel_id = interaction
        .channel
        .as_ref()
        .ok_or_else(|| Error::NoGuild)?
        .id;

    let message = tokio::join!(
        cancel_defer(ctx, interaction),
        channel_id.send_message(ctx, CreateMessage::new().content(content))
    )
    .1?;

    Ok(message)
}

pub async fn send_embed(
    ctx: &Context,
    interaction: &CommandInteraction,
    message_builder: CreateMessage,
) -> Result<Message> {
    let channel_id = interaction
        .channel
        .as_ref()
        .ok_or_else(|| Error::NoGuild)?
        .id;

    let message = tokio::join!(
        cancel_defer(ctx, interaction),
        channel_id.send_message(ctx, message_builder)
    )
    .1?;

    Ok(message)
}

pub async fn message_follow_up(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: impl Into<String>,
) -> Result<Message> {
    let message = interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new().content(content),
        )
        .await?;

    Ok(message)
}

pub async fn embed_follow_up(
    ctx: &Context,
    interaction: &CommandInteraction,
    embed: CreateEmbed,
) -> Result<Message> {
    let message = interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new().add_embed(embed),
        )
        .await?;

    Ok(message)
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
