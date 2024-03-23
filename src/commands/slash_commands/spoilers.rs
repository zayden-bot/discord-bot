use crate::sqlx_lib::{get_spoiler_channel_ids, get_support_channel_ids};
use crate::utils::send_message;
use crate::{Error, Result};
use serenity::all::{CommandInteraction, Context, CreateCommand};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let support_thread_ids = get_support_channel_ids(guild_id.get() as i64).await?;
    let support_thread_id = support_thread_ids
        .first()
        .ok_or_else(|| Error::NoSupportThread)?;

    let spoiler_thread_ids = get_spoiler_channel_ids(guild_id.get() as i64).await?;
    let spoiler_thread_id = spoiler_thread_ids
        .first()
        .ok_or_else(|| Error::NoSpoilerThread)?;

    send_message(ctx, interaction, &format!("Please keep all conversations about the new update to <#{}>\nIf you have any bugs or questions please post them in <#{}>", spoiler_thread_id, support_thread_id)).await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("spoilers").description("Disclaimer about spoilers")
}
