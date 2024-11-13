#![allow(dead_code)]

use serenity::all::{
    CommandInteraction, Context, CreateEmbed, CreateInteractionResponseFollowup, CreateMessage,
    EditInteractionResponse, Message, UserId,
};
use serenity::Result;

pub mod support;

pub async fn dm_user_embed(
    ctx: &Context,
    user_id: impl Into<UserId>,
    embed: CreateEmbed,
) -> Result<Message> {
    let user_id = user_id.into();

    let channel = user_id.create_dm_channel(ctx).await?;
    let message = channel
        .send_message(ctx, CreateMessage::new().embed(embed))
        .await?;

    // {
    //     Ok(_) => {}
    //     Err(serenity::Error::Http(UnsuccessfulRequest(ErrorResponse {
    //         error: DiscordJsonError { code: 50007, .. },
    //         ..
    //     }))) => {}
    //     Err(e) => return Err(e.into()),
    // }
    Ok(message)
}

pub async fn message_response(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: impl Into<String>,
) -> Result<Message> {
    let _ = interaction.defer(ctx).await;

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
    let _ = interaction.defer(ctx).await;

    let message = interaction
        .edit_response(ctx, EditInteractionResponse::new().add_embed(embed))
        .await?;

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