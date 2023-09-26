use crate::utils::{respond_with_ephemeral_message, respond_with_message};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::Permissions;
use serenity::prelude::Context;

const SUPPORT_CHANNEL_ID: u64 = 919950775134847016;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let current_channel = interaction
        .channel_id
        .to_channel(ctx)
        .await
        .unwrap()
        .guild()
        .unwrap();

    if current_channel.parent_id.unwrap().0 != SUPPORT_CHANNEL_ID {
        return respond_with_message(
            ctx,
            interaction,
            "This command can only be used in support channels",
        )
        .await;
    }

    let current_channel_name = current_channel.name;

    let new_channel_name = format!("{} - {}", "[Fixed]", current_channel_name)
        .chars()
        .take(100)
        .collect::<String>();

    interaction
        .channel_id
        .edit(ctx, |c| c.name(new_channel_name))
        .await
        .expect("Failed to edit channel name");

    respond_with_ephemeral_message(ctx, interaction, "Ticket reopened").await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("open")
        .description("Reopen a support ticket")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
}
