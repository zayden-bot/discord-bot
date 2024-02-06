use crate::sqlx_lib::{create_reaction_role, delete_reaction_role};
use crate::utils::message_response;
use serenity::all::{
    ChannelId, CommandDataOption, CommandDataOptionValue, CommandInteraction, CommandOptionType,
    Context, CreateCommand, CreateCommandOption, GuildId, Message, MessageId, Permissions,
    ReactionType,
};

async fn add(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[CommandDataOption],
    guild_id: GuildId,
    channel_id: ChannelId,
    message_id: MessageId,
    emoji: &str,
) -> Result<Message, serenity::Error> {
    let role_id = match options[3].value.as_role_id() {
        Some(role_id) => role_id,
        _ => return message_response(ctx, interaction, "Please provide a valid role").await,
    };

    let message = match channel_id.message(ctx, message_id).await {
        Ok(message) => message,
        Err(_) => {
            return message_response(ctx, interaction, "Please provide a valid message id").await
        }
    };

    if create_reaction_role(
        guild_id.get() as i64,
        channel_id.get() as i64,
        message_id.get() as i64,
        role_id.get() as i64,
        emoji,
    )
    .await
    .is_err()
    {
        return message_response(ctx, interaction, "Error adding reaction role").await;
    }

    message
        .react(ctx, ReactionType::Unicode(emoji.to_string()))
        .await?;
    message_response(ctx, interaction, "Reaction role added").await
}

async fn remove(
    ctx: &Context,
    interaction: &CommandInteraction,
    channel_id: ChannelId,
    guild_id: GuildId,
    message_id: MessageId,
    emoji: &str,
) -> Result<Message, serenity::Error> {
    let message = match channel_id.message(ctx, message_id).await {
        Ok(message) => message,
        Err(_) => {
            return message_response(ctx, interaction, "Please provide a valid message id").await
        }
    };

    if delete_reaction_role(
        guild_id.get() as i64,
        channel_id.get() as i64,
        message_id.get() as i64,
        emoji,
    )
    .await
    .is_err()
    {
        return message_response(ctx, interaction, "Error deleting reaction role").await;
    }

    match message
        .delete_reaction_emoji(ctx, ReactionType::Unicode(emoji.to_string()))
        .await
    {
        Ok(_) => message_response(ctx, interaction, "Reaction Role removed").await,
        Err(_) => message_response(ctx, interaction, "Error deleting reaction").await,
    }
}

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return message_response(
                &ctx,
                interaction,
                "This command can only be used in a server",
            )
            .await
        }
    };

    let command = &interaction.data.options[0];

    let options = match &command.value {
        CommandDataOptionValue::SubCommand(options) => options,
        _ => return message_response(&ctx, interaction, "Invalid subcommand").await,
    };

    let channel_id = match options[0].value.as_channel_id() {
        Some(channel) => channel,
        _ => return message_response(&ctx, interaction, "Please provide a valid channel").await,
    };

    let message_id = match options[1]
        .value
        .as_str()
        .and_then(|message_id| message_id.parse::<u64>().ok())
    {
        Some(message_id) => MessageId::new(message_id),
        _ => return message_response(&ctx, interaction, "Please provide a valid message id").await,
    };

    let emoji = match options[2].value.as_str() {
        Some(emoji) => emoji,
        _ => return message_response(&ctx, interaction, "Please provide a valid emoji").await,
    };

    match command.name.as_str() {
        "add" => {
            add(
                &ctx,
                interaction,
                options,
                guild_id,
                channel_id,
                message_id,
                emoji,
            )
            .await
        }
        "remove" => remove(&ctx, interaction, channel_id, guild_id, message_id, emoji).await,
        _ => message_response(&ctx, interaction, "Invalid subcommand").await,
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reaction_role")
        .description("Adds or removes a reaction role")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "add", "Adds a reaction role")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Channel,
                        "channel",
                        "The channel the message is in",
                    )
                    .required(true),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "message_id",
                        "The message id of the reaction role message",
                    )
                    .required(true),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "emoji",
                        "The emoji of the reaction role",
                    )
                    .required(true),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Role,
                        "role",
                        "The role to add when the emoji is reacted to",
                    )
                    .required(true),
                ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "remove",
                "Removes a reaction role",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "The channel the message is in",
                )
                .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "message_id",
                    "The message id of the reaction role message",
                )
                .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "emoji",
                    "The emoji of the reaction role",
                )
                .required(true),
            ),
        )
}
