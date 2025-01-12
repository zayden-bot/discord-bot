mod update_information_message;
mod update_render_requests_messages;
mod update_support_messages;

use crate::Result;

use futures::{StreamExt, TryStreamExt};
use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage, EditMessage};

pub async fn update_messages(ctx: &Context) -> Result<()> {
    tokio::try_join!(
        update_information_message::run(ctx),
        update_support_messages::run(ctx),
        update_render_requests_messages::run(ctx),
    )?;

    println!("Messages updated!");

    Ok(())
}

pub async fn send_or_update_message(
    ctx: &Context,
    channel_id: ChannelId,
    embed: CreateEmbed,
) -> Result<()> {
    let message = channel_id
        .messages_iter(ctx)
        .boxed()
        .try_next()
        .await
        .unwrap();

    match message {
        Some(mut message) => {
            message
                .edit(&ctx, EditMessage::new().embed(embed))
                .await
                .unwrap();
        }
        None => {
            channel_id
                .send_message(ctx, CreateMessage::new().embed(embed))
                .await
                .unwrap();
        }
    }

    Ok(())
}
