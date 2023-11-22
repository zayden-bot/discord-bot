use crate::utils::{respond_with_ephemeral_message, respond_with_message};
use serenity::all::{CommandInteraction, Context, CreateCommand, EditChannel, Permissions};

const SUPPORT_CHANNEL_ID: u64 = 919950775134847016;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let current_channel = interaction
        .channel_id
        .to_channel(ctx)
        .await
        .unwrap()
        .guild()
        .unwrap();

    if current_channel.parent_id.unwrap().get() != SUPPORT_CHANNEL_ID {
        return respond_with_message(
            ctx,
            interaction,
            "This command can only be used in support channels",
        )
        .await;
    }

    let new_channel_name = current_channel
        .name
        .replace("[Fixed] - ", "")
        .replace("[Closed] - ", "");

    interaction
        .channel_id
        .edit(ctx, EditChannel::new().name(new_channel_name))
        .await
        .expect("Failed to edit channel name");

    respond_with_ephemeral_message(ctx, interaction, "Ticket reopened").await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("open")
        .description("Reopen a support ticket")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
}
