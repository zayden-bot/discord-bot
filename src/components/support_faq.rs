use futures::TryStreamExt;
use serenity::{
    all::{
        ComponentInteraction, ComponentInteractionDataKind, Context, CreateEmbed,
        CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    futures::StreamExt,
};

use crate::{guilds::college_kings::SUPPORT_FAQ_CHANNEL_ID, Error, Result};

pub async fn support_faq(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let index = match &interaction.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => &values[0],
        _ => unreachable!("Invalid interaction data kind"),
    };

    let message = SUPPORT_FAQ_CHANNEL_ID
        .messages_iter(ctx)
        .skip(index.parse::<usize>()?)
        .boxed()
        .try_next()
        .await?
        .ok_or_else(|| Error::FaqMessageNotFound(index.to_string()))?;

    let mut parts: Vec<&str> = message.content.split("**").collect();
    let description = parts
        .pop()
        .ok_or_else(|| Error::FaqMessageNotFound(index.to_string()))?
        .trim();
    let title = parts.join("");

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(
                    CreateEmbed::new()
                        .title(title.trim())
                        .description(description),
                ),
            ),
        )
        .await?;

    Ok(())
}
