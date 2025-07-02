use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ButtonStyle, Context, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor,
    CreateEmbedFooter, CreateMessage, EditMessage, EmbedField, GuildChannel, Message, Reaction,
    ReactionType,
};
use sqlx::{Database, Pool};

use crate::{Suggestions, SuggestionsGuildManager};

impl Suggestions {
    pub async fn reaction<Db: Database, Manager: SuggestionsGuildManager<Db>>(
        ctx: &Context,
        reaction: &Reaction,
        pool: &Pool<Db>,
    ) {
        let Some(guild_id) = reaction.guild_id else {
            return;
        };

        let Some(channel) = reaction.channel(&ctx).await.unwrap().guild() else {
            return;
        };

        let Some(row) = Manager::get(pool, guild_id).await.unwrap() else {
            return;
        };

        if channel.parent_id.is_none()
            || row.channel_id().is_none()
            || channel.parent_id != row.channel_id()
        {
            return;
        }

        let Some(review_channel_id) = row.review_channel_id() else {
            return;
        };

        let message = reaction.message(ctx).await.unwrap();

        let positive_reaction = ReactionType::from('👍');
        let negative_reaction = ReactionType::from('👎');

        let (pos_count, neg_count) = message.reactions.iter().fold((0, 0), |(pos, neg), r| {
            if r.reaction_type == positive_reaction {
                (r.count as i32, neg)
            } else if r.reaction_type == negative_reaction {
                (pos, r.count as i32)
            } else {
                (pos, neg)
            }
        });

        let mut messages = review_channel_id.messages_iter(&ctx).boxed();

        if (pos_count - neg_count) >= 20 {
            while let Some(mut msg) = messages.try_next().await.unwrap() {
                if let Some(embed) = msg.embeds.first() {
                    if embed.url == Some(message.link()) {
                        msg.edit(
                            ctx,
                            EditMessage::new()
                                .embed(create_embed(
                                    &channel,
                                    &message,
                                    &msg.embeds[0].fields,
                                    pos_count,
                                    neg_count,
                                ))
                                .components(create_components()),
                        )
                        .await
                        .unwrap();

                        return;
                    }
                }
            }

            review_channel_id
                .send_message(
                    ctx,
                    CreateMessage::new()
                        .embed(create_embed(
                            &channel,
                            &message,
                            &Vec::new(),
                            pos_count,
                            neg_count,
                        ))
                        .components(create_components()),
                )
                .await
                .unwrap();
        } else if (neg_count - pos_count) <= 15 {
            while let Some(msg) = messages.try_next().await.unwrap() {
                if msg.embeds[0].url == Some(message.link()) {
                    msg.delete(ctx).await.unwrap();

                    return;
                }
            }
        }
    }
}

fn create_embed(
    channel: &GuildChannel,
    message: &Message,
    embed_fields: &[EmbedField],
    pos_count: i32,
    neg_count: i32,
) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title(&channel.name)
        .url(message.link())
        .description(&message.content)
        .author(CreateEmbedAuthor::new(&message.author.name))
        .footer(CreateEmbedFooter::new(format!(
            "👍 {} · 👎 {}",
            pos_count, neg_count
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
    ])]
}
