use crate::utils::{respond_with_ephemeral_message, respond_with_message};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    EditChannel, Permissions,
};

const CHANGE_LOG_CHANNEL_ID: u64 = 992599169288122410;
const SUPPORT_CHANNEL_ID: u64 = 919950775134847016;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let version = interaction
        .data
        .options
        .get(0)
        .map_or("", |option| option.value.as_str().unwrap_or(""));

    let is_silent = version.is_empty();

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
            "This command can only be used in support threads",
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
        .edit(ctx, EditChannel::new().name(new_channel_name))
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

pub fn register() -> CreateCommand {
    CreateCommand::new("fixed")
        .description("Mark support ticket as fixed")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "version",
            "The version the issue was fixed in",
        ))
}
