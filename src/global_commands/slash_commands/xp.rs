use crate::sqlx_lib::get_pool;
use crate::sqlx_lib::user_levels::get_user_level_data;
use crate::utils::{embed_response, parse_options};
use crate::Result;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, ResolvedValue,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    match options.get("ephemeral") {
        Some(ResolvedValue::Boolean(true)) => interaction.defer_ephemeral(&ctx).await?,
        _ => interaction.defer(&ctx).await?,
    }

    let pool = get_pool(ctx).await?;

    let level_data = get_user_level_data(&pool, interaction.user.id.get()).await?;

    embed_response(
        ctx,
        interaction,
        CreateEmbed::default().title("XP").description(format!(
            "Current XP: {}\nLevel: {}\nTotal XP: {}",
            level_data.xp, level_data.level, level_data.total_xp
        )),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("xp")
        .description("Get your current xp")
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            "ephemeral",
            "Whether the response should be ephemeral",
        ))
}
