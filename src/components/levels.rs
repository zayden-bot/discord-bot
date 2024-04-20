use serenity::all::{
    ComponentInteraction, Context, CreateEmbed, CreateEmbedFooter, EditInteractionResponse,
    EditMessage, UserId,
};

use crate::{
    sqlx_lib::{
        user_levels::{get_user_row_number, get_users},
        PostgresPool,
    },
    Error, Result,
};

const LIMIT: i64 = 10;

pub async fn levels(ctx: &Context, interaction: &ComponentInteraction, action: &str) -> Result<()> {
    interaction.defer(ctx).await?;

    let pool = {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    };

    let mut old_embed = interaction.message.embeds[0].clone();

    let mut page_number: i64 = old_embed
        .footer
        .clone()
        .unwrap()
        .text
        .strip_prefix("Page ")
        .unwrap()
        .parse()?;

    old_embed.fields = Vec::new();
    let mut new_embed: CreateEmbed = old_embed.into();

    match action {
        "previous" => {
            page_number = (page_number - 1).max(1);
        }
        "user" => {
            let row_number = get_user_row_number(&pool, interaction.user.id.get())
                .await?
                .ok_or_else(|| Error::UserNotFound)?;

            page_number = row_number / LIMIT + 1;
        }
        "next" => {
            page_number += 1;
        }
        _ => unreachable!(),
    };

    let mut fields = Vec::new();
    for level_data in get_users(&pool, page_number, 10).await? {
        let user = UserId::new(level_data.id as u64).to_user(ctx).await?;

        fields.push((
            user.name,
            format!(
                "Messages: {} | Total XP: {} | Level: {}",
                level_data.message_count, level_data.xp, level_data.level
            ),
            false,
        ));
    }

    new_embed = new_embed.footer(CreateEmbedFooter::new(format!("Page {}", page_number)));
    new_embed = new_embed.fields(fields);

    interaction
        .message
        .clone()
        .edit(ctx, EditMessage::new().embed(new_embed))
        .await?;

    interaction
        .edit_response(ctx, EditInteractionResponse::new())
        .await?;

    Ok(())
}
