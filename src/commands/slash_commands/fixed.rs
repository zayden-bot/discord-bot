use crate::utils::{respond_with_ephemeral_message, respond_with_message};
use serde_json::Value;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Permissions;
use serenity::prelude::Context;
use std::collections::HashMap;

const CHANGE_LOG_CHANNEL_ID: u64 = 992599169288122410;
const SUPPORT_CHANNEL_ID: u64 = 919950775134847016;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let options: HashMap<&str, &Value> = interaction
        .data
        .options
        .iter()
        .filter_map(|option| {
            if let Some(value) = &option.value {
                return Some((option.name.as_str(), value));
            }
            None
        })
        .collect();

    let version = match options.get("version") {
        Some(value) => value.as_str().unwrap(),
        None => "latest",
    };

    let is_silent = match options.get("silent") {
        Some(value) => value.as_bool().unwrap(),
        None => false,
    };

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

    if is_silent {
        respond_with_ephemeral_message(ctx, interaction, "Ticket marked as fixed").await
    } else {
        respond_with_message(
            ctx,
            interaction,
            &format!(
                "Fixed in {}. Check <#{}> for more details",
                version, CHANGE_LOG_CHANNEL_ID
            ),
        )
        .await
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("fixed")
        .description("Mark support ticket as fixed")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .create_option(|option| {
            option
                .name("version")
                .description("The version the issue was fixed in")
                .kind(CommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("silent")
                .description("Whether the fix should be announced in the thread")
                .kind(CommandOptionType::Boolean)
        })
}
