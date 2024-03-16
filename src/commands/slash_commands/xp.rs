use crate::utils::{embed_response, message_response, parse_options};
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

    let level_data = match get_user_level_data(interaction.user.id.get()).await {
        Ok(data) => data,
        Err(_) => {
            return message_response(&ctx, interaction, "Cannot get user level data").await;
        }
    };

    embed_response(
        &ctx,
        interaction,
        CreateEmbed::default().title("XP").description(format!(
            "Current XP: {}\nLevel: {}\nTotal XP: {}",
            level_data.xp, level_data.level, level_data.total_xp
        )),
    )
    .await
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
