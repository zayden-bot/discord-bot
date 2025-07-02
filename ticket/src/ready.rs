use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ActionRowComponent, ButtonKind, Context, CreateButton, CreateMessage, GuildId,
};
use sqlx::{Database, Pool};

use crate::{Result, TicketGuildManager};

pub async fn run<Db: Database, GuildManager: TicketGuildManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    guild_id: GuildId,
) -> Result<()> {
    let row = match GuildManager::get(pool, guild_id).await.unwrap() {
        Some(row) => row,
        None => return Ok(()),
    };

    let channel_id = match row.channel_id() {
        Some(channel_id) => channel_id,
        None => return Ok(()),
    };

    let mut messages = channel_id.messages_iter(&ctx).boxed();
    while let Some(message) = messages.try_next().await.unwrap() {
        if let Some(ActionRowComponent::Button(b)) = message
            .components
            .first()
            .and_then(|c| c.components.first())
        {
            if let ButtonKind::NonLink { custom_id, .. } = &b.data {
                if custom_id == "ticket_create" {
                    message.delete(ctx).await.unwrap();
                    break;
                }
            }
        }
    }

    channel_id
        .send_message(
            ctx,
            CreateMessage::default()
                .button(CreateButton::new("ticket_create").label("Create Support Ticket")),
        )
        .await
        .unwrap();

    Ok(())
}
