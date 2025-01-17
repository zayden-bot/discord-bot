use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ComponentInteraction, ComponentInteractionDataKind, Context, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::{guilds::college_kings::FAQ_CHANNEL_ID, Result};

pub async fn faq(ctx: &Context, interaction: &ComponentInteraction, ephemeral: bool) -> Result<()> {
    let index =
        if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
            &values[0]
        } else {
            unreachable!("Invalid interaction data kind")
        };

    let message = FAQ_CHANNEL_ID
        .messages_iter(ctx)
        .skip(index.parse::<usize>().unwrap())
        .boxed()
        .try_next()
        .await
        .unwrap()
        .unwrap();

    let mut parts: Vec<&str> = message.content.split("**").collect();
    let description = parts.pop().unwrap();
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
        .await
        .unwrap();

    Ok(())
}
