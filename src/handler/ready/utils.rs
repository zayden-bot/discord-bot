use serenity::{
    all::{ChannelId, Context, CreateEmbed, CreateMessage, EditMessage},
    futures::StreamExt,
};

use crate::Result;

pub async fn send_or_update_message(
    ctx: &Context,
    channel_id: u64,
    embed: CreateEmbed,
) -> Result<()> {
    let message = ChannelId::new(channel_id)
        .messages_iter(ctx)
        .boxed()
        .next()
        .await;

    match message {
        Some(message) => {
            message?.edit(&ctx, EditMessage::new().embed(embed)).await?;
        }
        None => {
            ChannelId::new(channel_id)
                .send_message(ctx, CreateMessage::new().embed(embed))
                .await?;
        }
    }

    Ok(())
}

/*

let mut message = ctx
    .http
    .get_message(
        ChannelId::new(INFORMATION_CHANNEL_ID),
        MessageId::new(INFORMATION_MESSAGE_ID),
    )
    .await?;
message.edit(&ctx, EditMessage::new().embed(embed)).await?;
 */
