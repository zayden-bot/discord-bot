use futures::{StreamExt, TryStreamExt};
use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage, EditMessage};

use crate::Result;

pub async fn send_or_update_message(
    ctx: &Context,
    channel_id: ChannelId,
    embed: CreateEmbed,
) -> Result<()> {
    let message = channel_id.messages_iter(ctx).boxed().try_next().await?;

    match message {
        Some(mut message) => {
            message.edit(&ctx, EditMessage::new().embed(embed)).await?;
        }
        None => {
            channel_id
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
