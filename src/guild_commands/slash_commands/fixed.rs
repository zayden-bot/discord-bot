use crate::{
    utils::{message_response, parse_options},
    Error, Result, COLLEGE_KINGS_GUILD_ID,
};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    EditChannel, GuildId, Permissions, ResolvedValue,
};

const CHANGE_LOG_CHANNEL_ID: u64 = 992599169288122410;
const SUPPORT_CHANNEL_ID: u64 = 919950775134847016;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    let version = match options.get("version") {
        Some(ResolvedValue::String(message)) => *message,
        _ => "",
    };

    match version.is_empty() {
        true => interaction.defer_ephemeral(&ctx).await?,
        false => interaction.defer(&ctx).await?,
    };

    let current_channel = interaction
        .channel_id
        .to_channel(&ctx)
        .await?
        .guild()
        .ok_or_else(|| Error::NoGuild)?;

    if current_channel
        .parent_id
        .ok_or_else(|| Error::NoParent)?
        .get()
        != SUPPORT_CHANNEL_ID
    {
        message_response(
            ctx,
            interaction,
            "This command can only be used in support channels",
        )
        .await?;
        return Ok(());
    }

    let new_channel_name = format!("{} - {}", "[Fixed]", current_channel.name)
        .chars()
        .take(100)
        .collect::<String>();

    interaction
        .channel_id
        .edit(ctx, EditChannel::new().name(new_channel_name))
        .await?;

    if version.is_empty() {
        message_response(ctx, interaction, "Ticket marked as fixed").await?;
    } else {
        message_response(
            ctx,
            interaction,
            format!(
                "Fixed in {}. Check <#{}> for more details",
                version, CHANGE_LOG_CHANNEL_ID
            ),
        )
        .await?;
    }

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    GuildId::new(COLLEGE_KINGS_GUILD_ID)
        .create_command(
            ctx,
            CreateCommand::new("fixed")
                .description("Mark support ticket as fixed")
                .default_member_permissions(Permissions::MANAGE_MESSAGES)
                .add_option(CreateCommandOption::new(
                    CommandOptionType::String,
                    "version",
                    "The version the issue was fixed in",
                )),
        )
        .await?;

    Ok(())
}
