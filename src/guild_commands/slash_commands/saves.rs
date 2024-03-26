use crate::sqlx_lib::get_support_channel_ids;
use crate::utils::send_message;
use crate::{Error, Result, COLLEGE_KINGS_GUILD_ID};
use serenity::all::{CommandInteraction, Context, CreateCommand, GuildId};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let support_thread_ids = get_support_channel_ids(guild_id.get() as i64).await?;
    let support_thread_id = support_thread_ids
        .first()
        .ok_or_else(|| Error::NoSupportThread)?;

    send_message(ctx, interaction, format!("We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#{}>", support_thread_id)).await?;

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    GuildId::new(COLLEGE_KINGS_GUILD_ID)
        .create_command(
            ctx,
            CreateCommand::new("saves").description("Get saves disclaimer"),
        )
        .await?;

    Ok(())
}
