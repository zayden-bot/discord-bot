use crate::utils::message_response;
use crate::{Error, Result};
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateCommand, EditChannel, Permissions,
};

const CHANNEL_ID: ChannelId = ChannelId::new(919950775134847016);

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let current_channel = interaction
        .channel_id
        .to_channel(&ctx)
        .await?
        .guild()
        .ok_or_else(|| Error::NoGuild)?;

    if current_channel.parent_id.ok_or_else(|| Error::NoParent)? != CHANNEL_ID {
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

pub fn register() -> CreateCommand {
    CreateCommand::new("open")
        .description("Reopen a support ticket")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
}
