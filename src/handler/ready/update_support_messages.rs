use crate::Result;
use serenity::{
    all::{ActionRowComponent, ButtonKind, ChannelId, Context, CreateButton, CreateMessage},
    futures::StreamExt,
};

const CHANNEL_ID: ChannelId = ChannelId::new(919950775134847016);

pub async fn run(ctx: &Context) -> Result<()> {
    let mut message = None;
    while let Some(Ok(msg)) = CHANNEL_ID.messages_iter(ctx).boxed().next().await {
        if let Some(ActionRowComponent::Button(b)) =
            msg.components.first().and_then(|c| c.components.first())
        {
            if let ButtonKind::NonLink { custom_id, .. } = &b.data {
                if custom_id == "support_ticket" {
                    message = Some(msg);
                    break;
                }
            }
        }
    }

    if let Some(message) = message {
        message.delete(ctx).await?;
    }

    CHANNEL_ID
        .send_message(
            ctx,
            CreateMessage::default()
                .button(CreateButton::new("support_ticket").label("Create Support Ticket")),
        )
        .await?;

    Ok(())
}
