use async_trait::async_trait;
use futures::future;
use serenity::all::{
    ComponentInteraction, Context, CreateEmbed, CreateEmbedFooter, EditInteractionResponse,
    MessageInteractionMetadata,
};
use sqlx::{PgPool, Postgres};
use zayden_core::Component;

use crate::{Error, Result};

use super::{LevelsRow, LevelsTable};

const LIMIT: i64 = 10;

#[async_trait]
impl Component<Error, Postgres> for LevelsRow {
    async fn run(ctx: &Context, interaction: &ComponentInteraction, pool: &PgPool) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let action = interaction.data.custom_id.split_once('_').unwrap().1;

        if let Some(MessageInteractionMetadata::Component(metadata)) =
            interaction.message.interaction_metadata.as_deref()
        {
            if metadata.user != interaction.user {
                return Err(Error::NotInteractionAuthor);
            }
        }

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
                let row_number = LevelsTable::get_user_row_number(pool, interaction.user.id)
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

        let iter = LevelsTable::get_users(pool, page_number, LIMIT)
            .await?
            .into_iter()
            .map(|row| async move {
                (
                    row.as_user(ctx).await.unwrap().name,
                    format!(
                        "Messages: {} | Total XP: {} | Level: {}",
                        row.message_count, row.xp, row.level
                    ),
                    false,
                )
            });

        let fields = future::join_all(iter).await;

        new_embed = new_embed.footer(CreateEmbedFooter::new(format!("Page {}", page_number)));
        new_embed = new_embed.fields(fields);

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(new_embed))
            .await
            .unwrap();

        Ok(())
    }
}
