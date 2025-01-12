use serenity::all::{
    ComponentInteraction, Context, CreateEmbed, CreateEmbedFooter, EditInteractionResponse,
    MessageInteractionMetadata,
};

use crate::sqlx_lib::user_levels::{get_user_row_number, get_users};
use crate::sqlx_lib::PostgresPool;
use crate::{Error, Result};

const LIMIT: i64 = 10;

pub async fn levels(ctx: &Context, interaction: &ComponentInteraction, action: &str) -> Result<()> {
    interaction.defer(ctx).await.unwrap();

    if let Some(MessageInteractionMetadata::Component(metadata)) =
        interaction.message.interaction_metadata.as_deref()
    {
        if metadata.user != interaction.user {
            return Err(Error::NotInteractionAuthor);
        }
    }

    let pool = PostgresPool::get(ctx).await;

    let mut old_embed = interaction.message.embeds[0].clone();

    let mut page_number: i64 = old_embed
        .footer
        .clone()
        .unwrap()
        .text
        .strip_prefix("Page ")
        .unwrap()
        .parse()
        .unwrap();

    old_embed.fields = Vec::new();
    let mut new_embed: CreateEmbed = old_embed.into();

    match action {
        "previous" => {
            page_number = (page_number - 1).max(1);
        }
        "user" => {
            let row_number = get_user_row_number(&pool, interaction.user.id)
                .await
                .unwrap()
                .unwrap();

            page_number = row_number / LIMIT + 1;
        }
        "next" => {
            page_number += 1;
        }
        _ => unreachable!(),
    };

    let fields = get_users(ctx, page_number, LIMIT)
        .await?
        .into_iter()
        .map(|level_data| {
            (
                level_data.user.name,
                format!(
                    "Messages: {} | Total XP: {} | Level: {}",
                    level_data.message_count, level_data.xp, level_data.level
                ),
                false,
            )
        });

    new_embed = new_embed.footer(CreateEmbedFooter::new(format!("Page {}", page_number)));
    new_embed = new_embed.fields(fields);

    interaction
        .edit_response(ctx, EditInteractionResponse::new().embed(new_embed))
        .await
        .unwrap();

    Ok(())
}
