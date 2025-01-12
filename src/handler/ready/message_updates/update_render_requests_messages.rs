use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ActionRowComponent, ButtonKind, Context, CreateButton, CreateEmbed, CreateMessage,
};

use crate::{guilds::college_kings::RENDER_REQUESTS_CHANNEL_ID, Result};

pub async fn run(ctx: &Context) -> Result<()> {
    let mut messages = RENDER_REQUESTS_CHANNEL_ID.messages_iter(&ctx).boxed();
    while let Some(message) = messages.try_next().await.unwrap() {
        if let Some(ActionRowComponent::Button(b)) = message
            .components
            .first()
            .and_then(|c| c.components.first())
        {
            if let ButtonKind::NonLink { custom_id, .. } = &b.data {
                if custom_id == "render_request" {
                    message.delete(ctx).await.unwrap();
                    break;
                }
            }
        }
    }

    RENDER_REQUESTS_CHANNEL_ID
        .send_message(
            ctx,
            CreateMessage::new()
                .embed(CreateEmbed::new().description(
                    "Custom renders are only available to President ($50) and King ($100) tier patrons.",
                ))
                .button(CreateButton::new("render_request").label("Request Render")),
        )
        .await.unwrap();

    Ok(())
}
