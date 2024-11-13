use serenity::all::{CommandInteraction, Context, CreateCommand, EditChannel, Permissions, Ready};

use crate::guilds::ServersTable;
use crate::sqlx_lib::PostgresPool;
use crate::utils::message_response;
use crate::{Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let current_channel = interaction
        .channel_id
        .to_channel(&ctx)
        .await?
        .guild()
        .ok_or_else(|| Error::NotInGuild)?;

    let pool = PostgresPool::get(ctx).await;

    let support_channel_id = ServersTable::get_row(&pool, current_channel.guild_id)
        .await?
        .ok_or(crate::guilds::ServersTableError::ServerNotFound)?
        .get_support_channel_id()?;

    if current_channel.parent_id.ok_or_else(|| Error::NoParent)? != support_channel_id {
        message_response(
            ctx,
            interaction,
            "This command can only be used in support channels",
        )
        .await?;
        return Ok(());
    }

    let new_channel_name = current_channel
        .name
        .replace("[Fixed] - ", "")
        .replace("[Closed] - ", "");

    interaction
        .channel_id
        .edit(&ctx, EditChannel::new().name(new_channel_name))
        .await?;

    message_response(ctx, interaction, "Ticket reopened").await?;

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("open")
        .description("Reopen a support ticket")
        .default_member_permissions(Permissions::MANAGE_MESSAGES);

    Ok(command)
}
