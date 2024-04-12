use serenity::{
    all::{
        ComponentInteraction, ComponentInteractionDataKind, Context, CreateEmbed,
        CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    futures::StreamExt,
};

use crate::{guild_commands::slash_commands::faq::FAQ_CHANNEL_ID, Error, Result};

pub async fn faq(ctx: &Context, interaction: &ComponentInteraction, ephemeral: bool) -> Result<()> {
    let index =
        if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
            &values[0]
        } else {
            unreachable!("Invalid interaction data kind")
        };

    let message = FAQ_CHANNEL_ID
        .messages_iter(ctx)
        .skip(index.parse::<usize>()?)
        .filter_map(|msg| async { msg.ok() })
        .boxed()
        .next()
        .await
        .ok_or_else(|| Error::FaqMessageNotFound(index.to_string()))?;

    let mut parts: Vec<&str> = message.content.split("**").collect();
    let description = parts
        .pop()
        .ok_or_else(|| Error::FaqMessageNotFound(index.to_string()))?;
    let title = parts.join("");

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default()
                    .ephemeral(ephemeral)
                    .embed(
                        CreateEmbed::default()
                            .title(title.trim())
                            .description(description.trim()),
                    ),
            ),
        )
        .await?;

    Ok(())
}
