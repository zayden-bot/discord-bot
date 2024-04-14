use crate::Result;
use serenity::{
    all::{ActionRowComponent, ButtonKind, ChannelId, Context, CreateButton, CreateMessage},
    futures::StreamExt,
};

const CHANNEL_ID: ChannelId = ChannelId::new(919950775134847016);

pub async fn run(ctx: &Context) -> Result<()> {
    let mut messages = CHANNEL_ID.messages_iter(&ctx).boxed();
    while let Some(Ok(message)) = messages.next().await {
        println!("{:?}", message);
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

    CHANNEL_ID
        .send_message(
            ctx,
            CreateMessage::default()
                .button(CreateButton::new("support_ticket").label("Create Support Ticket")),
        )
        .await?;

    Ok(())
}
