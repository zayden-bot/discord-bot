use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, Ready, ResolvedValue,
};
use zayden_core::parse_options;

use crate::sqlx_lib::user_levels::get_user_level_data;
use crate::sqlx_lib::PostgresPool;
use crate::utils::embed_response;
use crate::Result;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    match options.get("ephemeral") {
        Some(ResolvedValue::Boolean(true)) => interaction.defer_ephemeral(&ctx).await.unwrap(),
        _ => interaction.defer(&ctx).await.unwrap(),
    }

    let pool = PostgresPool::get(ctx).await;

    let level_data = get_user_level_data(&pool, interaction.user.id).await?;

    embed_response(
        ctx,
        interaction,
        CreateEmbed::default().title("XP").description(format!(
            "Current XP: {}\nLevel: {}\nTotal XP: {}",
            level_data.xp, level_data.level, level_data.total_xp
        )),
    )
    .await
    .unwrap();

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("xp")
        .description("Get your current xp")
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            "ephemeral",
            "Whether the response should be ephemeral",
        ));

    Ok(command)
}
