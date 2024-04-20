use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, ResolvedValue,
};

use crate::sqlx_lib::user_levels::{get_user_level_data, get_user_rank};
use crate::sqlx_lib::PostgresPool;
use crate::utils::{embed_response, parse_options};
use crate::{Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    match options.get("ephemeral") {
        Some(ResolvedValue::Boolean(true)) => interaction.defer_ephemeral(&ctx).await?,
        _ => interaction.defer(&ctx).await?,
    }

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => &interaction.user,
    };

    let pool = {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    };

    let level_data = get_user_level_data(&pool, user.id.get()).await?;

    let level = level_data.level;
    let xp_for_next_level = 5 * (level * level) + 50 * level + 100;
    let user_rank = get_user_rank(&pool, user.id.get())
        .await?
        .ok_or_else(|| Error::UserNotFound)?;

    embed_response(
        ctx,
        interaction,
        CreateEmbed::default()
            .title(format!("XP stats for {}", user.name))
            .description(format!(
                "Rank: #{}\nLevel: {}\nXP: {}/{} ({}%)",
                user_rank,
                level,
                level_data.xp,
                xp_for_next_level,
                (level_data.xp as f32 / xp_for_next_level as f32 * 100.0).round()
            )),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("rank")
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
        ))
}
