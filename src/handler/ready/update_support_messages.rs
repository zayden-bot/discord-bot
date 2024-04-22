use futures::{StreamExt, TryStreamExt};
use serenity::all::{ActionRowComponent, ButtonKind, Context, CreateButton, CreateMessage};

use crate::{guilds::college_kings::SUPPORT_CHANNEL_ID, Result};

pub async fn run(ctx: &Context) -> Result<()> {
    let mut messages = SUPPORT_CHANNEL_ID.messages_iter(&ctx).boxed();
    while let Some(message) = messages.try_next().await? {
        if let Some(ActionRowComponent::Button(b)) = message
            .components
            .first()
            .and_then(|c| c.components.first())
        {
            if let ButtonKind::NonLink { custom_id, .. } = &b.data {
                if custom_id == "support_ticket" {
                    message.delete(ctx).await?;
                    break;
                }
            }
        }
    }

    SUPPORT_CHANNEL_ID
        .send_message(
            ctx,
            CreateMessage::default()
                .button(CreateButton::new("support_ticket").label("Create Support Ticket")),
        )
        .await?;

    Ok(())
}
