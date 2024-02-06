use crate::utils::{message_response, send_message};
use serenity::all::{
    CommandInteraction, Context, CreateCommand, EditChannel, Message, Permissions,
};

const SUPPORT_CHANNEL_ID: u64 = 919950775134847016;

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let current_channel = interaction
        .channel_id
        .to_channel(&ctx)
        .await
        .unwrap()
        .guild()
        .unwrap();

    if current_channel.parent_id.unwrap().get() != SUPPORT_CHANNEL_ID {
        return message_response(
            &ctx,
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
        .edit(&ctx, EditChannel::new().name(new_channel_name))
        .await
        .expect("Failed to edit channel name");

    send_message(&ctx, interaction, "Ticket reopened").await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("open")
        .description("Reopen a support ticket")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
}
