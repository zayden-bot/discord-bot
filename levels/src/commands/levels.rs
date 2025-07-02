use std::time::Duration;

use futures::StreamExt;
use serenity::all::{
    CommandInteraction, ComponentInteraction, Context, CreateButton, CreateEmbed,
    CreateEmbedFooter, EditInteractionResponse, GuildId, Mentionable,
};
use sqlx::{Database, Pool};
use zayden_core::cache::GuildMembersCache;

use crate::{LeaderboardRow, LevelsManager, LevelsRow};

use super::Commands;

impl Commands {
    pub async fn levels<Db: Database, Manager: LevelsManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) {
        interaction.defer(ctx).await.unwrap();

        let embed = create_embed::<Db, Manager>(ctx, pool, interaction.guild_id.unwrap(), 1).await;

        let msg = interaction
            .edit_response(
                &ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .button(CreateButton::new("previous").label("<"))
                    .button(CreateButton::new("user").emoji('🎯'))
                    .button(CreateButton::new("next").label(">")),
            )
            .await
            .unwrap();

        let mut stream = msg
            .await_component_interactions(ctx)
            .timeout(Duration::from_secs(120))
            .stream();

        while let Some(component) = stream.next().await {
            run_components::<Db, Manager>(ctx, component, pool).await
        }
    }
}

async fn run_components<Db: Database, Manager: LevelsManager<Db>>(
    ctx: &Context,
    mut interaction: ComponentInteraction,
    pool: &Pool<Db>,
) {
    interaction.defer(ctx).await.unwrap();

    let Some(embed) = interaction.message.embeds.pop() else {
        unreachable!("Embed must be present")
    };

    let page_number = match interaction.data.custom_id.as_str() {
        "previous" => {
            embed
                .footer
                .unwrap()
                .text
                .strip_prefix("Page ")
                .unwrap()
                .parse::<i64>()
                .unwrap()
                - 1
        }
        "user" => {
            let Some(row_number) = Manager::user_rank(pool, interaction.user.id).await.unwrap()
            else {
                return;
            };

            row_number / 10 + 1
        }
        "next" => {
            embed
                .footer
                .unwrap()
                .text
                .strip_prefix("Page ")
                .unwrap()
                .parse::<i64>()
                .unwrap()
                + 1
        }
        _ => unreachable!(),
    }
    .max(1);

    let embed =
        create_embed::<Db, Manager>(ctx, pool, interaction.guild_id.unwrap(), page_number + 1)
            .await;

    interaction
        .edit_response(ctx, EditInteractionResponse::new().embed(embed))
        .await
        .unwrap();
}

async fn create_embed<Db: Database, Manager: LevelsManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    guild_id: GuildId,
    page_number: i64,
) -> CreateEmbed {
    let users = {
        let data = ctx.data.read().await;
        let cache = data.get::<GuildMembersCache>().unwrap();
        cache
            .get(&guild_id)
            .unwrap()
            .iter()
            .map(|id| id.get() as i64)
            .collect::<Vec<_>>()
    };

    let rows = Manager::leaderboard(pool, &users, page_number)
        .await
        .unwrap();

    let desc = rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| row_as_desc(&row, i + (page_number as usize - 1) * 10))
        .collect::<Vec<_>>()
        .join("\n\n");

    CreateEmbed::new()
        .title("Leaderboard")
        .description(desc)
        .footer(CreateEmbedFooter::new(format!("Page {}", page_number)))
}

fn row_as_desc(row: &LeaderboardRow, i: usize) -> String {
    let place = if i == 0 {
        "🥇".to_string()
    } else if i == 1 {
        "🥈".to_string()
    } else if i == 2 {
        "🥉".to_string()
    } else {
        format!("#{}", i + 1)
    };

    let data = format!(
        "{}\n(Messages: {} | Total XP: {})",
        row.level(),
        row.message_count(),
        row.xp(),
    );

    format!("{place} - {} - {data}", row.user_id().mention())
}
