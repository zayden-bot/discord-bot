use crate::{
    commands::message_commands::levels::user_levels::get_user_rank,
    utils::{embed_response, message_response, parse_options},
};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Message, ResolvedValue,
};

use crate::commands::message_commands::levels::user_levels::get_user_level_data;

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    match options.get("ephemeral") {
        Some(ResolvedValue::Boolean(ephemeral)) => {
            if *ephemeral {
                interaction.defer_ephemeral(&ctx).await?;
            }
        }
        _ => interaction.defer(&ctx).await?,
    }

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => &interaction.user,
    };

    let level_data = match get_user_level_data(user.id.get()).await {
        Ok(data) => data,
        Err(_) => {
            return message_response(&ctx, interaction, "Cannot get user level data").await;
        }
    };

    let level = level_data.level;
    let xp_for_next_level = 5 * (level * level) + 50 * level + 100;
    let user_rank = get_user_rank(user.id.get()).await.unwrap_or(None);

    embed_response(
        &ctx,
        interaction,
        CreateEmbed::default()
            .title(format!("XP stats for {}", user.name))
            .description(format!(
                "Rank: #{}\nLevel: {}\nXP: {}/{} ({}%)",
                user_rank.unwrap_or(-1),
                level,
                level_data.xp,
                xp_for_next_level,
                (level_data.xp as f32 / xp_for_next_level as f32 * 100.0).round()
            )),
    )
    .await
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
