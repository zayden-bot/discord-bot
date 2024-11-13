use serenity::all::{CommandInteraction, Context, CreateCommand, Mentionable, Ready};

use crate::guilds::{ServersTable, ServersTableError};
use crate::sqlx_lib::PostgresPool;
use crate::utils::message_response;
use crate::{Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(ctx).await?;

    let guild_id = interaction.guild_id.ok_or_else(|| Error::NotInGuild)?;

    let pool = PostgresPool::get(ctx).await;

    let row = ServersTable::get_row(&pool, guild_id.get())
        .await?
        .ok_or(ServersTableError::ServerNotFound)?;
    let support_channel_id = row.get_support_channel_id()?;
    let spoiler_channel_id = row.get_spoiler_channel_id()?;

    message_response(ctx, interaction, &format!(
"Spoilers are defined as any content that has not been released on all supported platforms for at least 2 weeks.
Please keep all conversations about spoilers to {}.
If you have any bugs or questions please post them in {}",
support_channel_id.mention(), spoiler_channel_id.mention())).await?;

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("spoilers").description("Disclaimer about spoilers");

    Ok(command)
}
