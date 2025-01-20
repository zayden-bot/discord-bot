use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateButton, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateEmbedFooter, EditInteractionResponse, Ready,
    ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{parse_options, SlashCommand};

use crate::{Error, Result};

use super::{get_user_level_data, get_user_rank, get_users, Levels};

#[async_trait]
impl SlashCommand<Error, Postgres> for Levels {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(&ctx).await.unwrap();

        let page_number = 1;

        let fields = get_users(ctx, pool, page_number, 10)
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

        let embed = CreateEmbed::new()
            .title("Leaderboard")
            .fields(fields)
            .footer(CreateEmbedFooter::new(format!("Page {}", page_number)));

        interaction
            .edit_response(
                &ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .button(CreateButton::new("levels_previous").label("<"))
                    .button(CreateButton::new("levels_user").emoji('🎯'))
                    .button(CreateButton::new("levels_next").label(">")),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("levels").description("Get the leaderboard");

        Ok(command)
    }
}

pub struct Rank;

#[async_trait]
impl SlashCommand<Error, Postgres> for Rank {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        let mut options = parse_options(options);

        match options.remove("ephemeral") {
            Some(ResolvedValue::Boolean(true)) => interaction.defer_ephemeral(&ctx).await.unwrap(),
            _ => interaction.defer(&ctx).await.unwrap(),
        }

        let user = match options.remove("user") {
            Some(ResolvedValue::User(user, _)) => user,
            _ => &interaction.user,
        };

        let level_data = get_user_level_data(pool, user.id).await?;

        let level = level_data.level;
        let xp_for_next_level = 5 * (level * level) + 50 * level + 100;
        let user_rank = get_user_rank(pool, user.id).await?.unwrap();

        let embed = CreateEmbed::new()
            .title(format!("XP stats for {}", user.name))
            .description(format!(
                "Rank: #{}\nLevel: {}\nXP: {}/{} ({}%)",
                user_rank,
                level,
                level_data.xp,
                xp_for_next_level,
                (level_data.xp as f32 / xp_for_next_level as f32 * 100.0).round()
            ));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("rank")
            .description("Get your rank or another member's rank")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to get the xp of",
            ))
            .add_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "ephemeral",
                "Whether the response should be ephemeral",
            ));

        Ok(command)
    }
}

pub struct Xp;

#[async_trait]
impl SlashCommand<Error, Postgres> for Xp {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        let mut options = parse_options(options);

        match options.remove("ephemeral") {
            Some(ResolvedValue::Boolean(true)) => interaction.defer_ephemeral(&ctx).await.unwrap(),
            _ => interaction.defer(&ctx).await.unwrap(),
        }

        let level_data = get_user_level_data(pool, interaction.user.id).await?;

        let embed = CreateEmbed::default().title("XP").description(format!(
            "Current XP: {}\nLevel: {}\nTotal XP: {}",
            level_data.xp, level_data.level, level_data.total_xp
        ));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("xp")
            .description("Get your current xp")
            .add_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "ephemeral",
                "Whether the response should be ephemeral",
            ));

        Ok(command)
    }
}
