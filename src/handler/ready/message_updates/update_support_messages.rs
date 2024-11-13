use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ActionRowComponent, ButtonKind, ChannelId, Context, CreateButton, CreateMessage,
};

use crate::{guilds::ServersTable, sqlx_lib::PostgresPool, Result};

pub async fn run(ctx: &Context) -> Result<()> {
    let pool = PostgresPool::get(ctx).await;

    let support_channel_ids = ServersTable::get_support_channel_ids(&pool).await?;

    for support_channel_id in support_channel_ids {
        update_support_message(ctx, support_channel_id).await?;
    }

    Ok(())
}

pub async fn update_support_message(ctx: &Context, support_channel_id: ChannelId) -> Result<()> {
    let mut messages = support_channel_id.messages_iter(&ctx).boxed();
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

    support_channel_id
        .send_message(
            ctx,
            CreateMessage::default()
                .button(CreateButton::new("support_ticket").label("Create Support Ticket")),
        )
        .await
        .unwrap();

    Ok(())
}
