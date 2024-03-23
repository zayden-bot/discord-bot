use crate::Result;
use serenity::{
    all::{ActionRowComponent, ButtonKind, ChannelId, Context, CreateButton, CreateMessage},
    futures::StreamExt,
};

const CHANNEL_ID: u64 = 919950775134847016;

pub async fn run(ctx: &Context) -> Result<()> {
    let message = ChannelId::new(CHANNEL_ID)
        .messages_iter(ctx)
        .filter_map(|m| async move {
            match m {
                Ok(m) => {
                    if let Some(ActionRowComponent::Button(b)) =
                        m.components.first().and_then(|c| c.components.first())
                    {
                        if let ButtonKind::NonLink { custom_id, .. } = &b.data {
                            if custom_id == "support_ticket" {
                                return Some(m);
                            }
                        }
                    }
                    None
                }
                _ => None,
            }
        })
        .boxed()
        .next()
        .await;

    if let Some(message) = message {
        message.delete(ctx).await?;
    }

    ChannelId::new(CHANNEL_ID)
        .send_message(
            ctx,
            CreateMessage::default()
                .button(CreateButton::new("support_ticket").label("Create Support Ticket")),
        )
        .await?;

    Ok(())
}
