use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    EditChannel, Permissions, Ready, ResolvedValue,
};
use zayden_core::parse_options;

use crate::{
    guilds::{ServersTable, ServersTableError},
    sqlx_lib::PostgresPool,
    utils::message_response,
    Error, Result,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    let message = match options.get("message") {
        Some(ResolvedValue::String(message)) => *message,
        _ => "",
    };

    match message.is_empty() {
        true => interaction.defer_ephemeral(&ctx).await?,
        false => interaction.defer(&ctx).await?,
    };

    let current_channel = interaction
        .channel_id
        .to_channel(&ctx)
        .await?
        .guild()
        .ok_or_else(|| Error::NotInGuild)?;

    let pool = PostgresPool::get(ctx).await;

    let support_channel_id = ServersTable::get_row(&pool, current_channel.guild_id)
        .await?
        .ok_or(ServersTableError::ServerNotFound)?
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

    let new_channel_name: String = format!("{} - {}", "[Closed]", current_channel.name)
        .chars()
        .take(100)
        .collect();

    interaction
        .channel_id
        .edit(&ctx, EditChannel::new().name(new_channel_name))
        .await?;

    message_response(
        ctx,
        interaction,
        format!("Ticket marked as closed\n\n{}", message),
    )
    .await?;

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("close")
        .description("Mark support ticket as closed")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "message",
            "The message to send to the ticket",
        ));

    Ok(command)
}
