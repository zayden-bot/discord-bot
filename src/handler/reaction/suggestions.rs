use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ButtonStyle, Context, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor,
    CreateEmbedFooter, CreateMessage, EditMessage, EmbedField, GuildChannel, Message, Reaction,
    ReactionType,
};

use crate::{guilds::college_kings_team::SUGGESTION_CHANNEL_ID, Result};

const POSITIVE_REACTION: &str = "👍";
const NEGATIVE_REACTION: &str = "👎";

pub async fn suggestion(ctx: &Context, reaction: &Reaction, channel: GuildChannel) -> Result<()> {
    let message = reaction.message(ctx).await?;

    let mut positive_count: i32 = 0;
    let mut negative_count: i32 = 0;

    for reaction in &message.reactions {
        if reaction.reaction_type == ReactionType::Unicode(POSITIVE_REACTION.to_string()) {
            positive_count = reaction.count as i32;
        } else if reaction.reaction_type == ReactionType::Unicode(NEGATIVE_REACTION.to_string()) {
            negative_count = reaction.count as i32;
        }
    }

    let mut messages = SUGGESTION_CHANNEL_ID.messages_iter(&ctx).boxed();

    if (positive_count - negative_count) >= 20 {
        while let Some(mut msg) = messages.try_next().await? {
            if msg.embeds[0].url == Some(message.link()) {
                msg.edit(
                    ctx,
                    EditMessage::new()
                        .embed(create_embed(
                            &channel,
                            &message,
                            &msg.embeds[0].fields,
                            positive_count,
                            negative_count,
                        ))
                        .components(create_components()),
                )
                .await?;
                return Ok(());
            }
        }

        SUGGESTION_CHANNEL_ID
            .send_message(
                ctx,
                CreateMessage::new()
                    .embed(create_embed(
                        &channel,
                        &message,
                        &Vec::new(),
                        positive_count,
                        negative_count,
                    ))
                    .components(create_components()),
            )
            .await?;
    } else if (negative_count - positive_count) <= 15 {
        while let Some(msg) = messages.try_next().await? {
            if msg.embeds[0].url == Some(message.link()) {
                msg.delete(ctx).await?;
                return Ok(());
            }
        }
    }

    Ok(())
}

fn create_embed(
    channel: &GuildChannel,
    message: &Message,
    embed_fields: &[EmbedField],
    positive_count: i32,
    negative_count: i32,
) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title(&channel.name)
        .url(message.link())
        .description(&message.content)
        .author(CreateEmbedAuthor::new(&message.author.name))
        .footer(CreateEmbedFooter::new(format!(
            "{} {} · {} {}",
            POSITIVE_REACTION, positive_count, NEGATIVE_REACTION, negative_count
        )));

    if let Some(team_response) = embed_fields.first() {
        embed = embed.field(
            &team_response.name,
            &team_response.value,
            team_response.inline,
        );
    }

    embed
}

fn create_components() -> Vec<CreateActionRow> {
    vec![CreateActionRow::Buttons(vec![
        CreateButton::new("suggestions_accept")
            .label("Accept")
            .style(ButtonStyle::Success),
        CreateButton::new("suggestions_reject")
            .label("Reject")
            .style(ButtonStyle::Danger),
        CreateButton::new("suggestions_added")
            .label("Already Added")
            .style(ButtonStyle::Primary),
    ])]
}
