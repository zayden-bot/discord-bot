use crate::sqlx_lib::{create_reaction_role, delete_reaction_role, PostgresPool};
use crate::utils::{message_response, parse_options};
use crate::{Error, Result};
use serenity::all::{
    Command, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, GuildId, Mentionable, MessageId, PartialChannel, Permissions,
    ReactionType, ResolvedValue, Role,
};
use sqlx::{Pool, Postgres};

async fn add(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Postgres>,
    channel: &PartialChannel,
    message_id: Option<MessageId>,
    emoji: ReactionType,
    role: &Role,
) -> Result<()> {
    let guild_id = role.guild_id;

    let message = match message_id {
        Some(message_id) => channel.id.message(ctx, message_id).await?,
        None => {
            channel
                .id
                .send_message(
                    ctx,
                    CreateMessage::default().embed(CreateEmbed::default().description(format!(
                        "{} | {}",
                        emoji,
                        role.mention()
                    ))),
                )
                .await?
        }
    };

    create_reaction_role(
        pool,
        guild_id.get(),
        channel.id.get(),
        message.id.get(),
        role.id.get(),
        &emoji.to_string(),
    )
    .await?;

    message.react(ctx, emoji).await?;
    message_response(ctx, interaction, "Reaction role added").await?;

    Ok(())
}

async fn remove(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Postgres>,
    channel: &PartialChannel,
    guild_id: GuildId,
    message_id: MessageId,
    emoji: ReactionType,
) -> Result<()> {
    let message = channel.id.message(ctx, message_id).await?;

    delete_reaction_role(
        pool,
        guild_id.get(),
        channel.id.get(),
        message_id.get(),
        &emoji.to_string(),
    )
    .await?;

    message.delete_reaction_emoji(ctx, emoji).await?;
    message_response(ctx, interaction, "Reaction Role removed").await?;

    Ok(())
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let command = &interaction.data.options()[0];

    let options = match &command.value {
        ResolvedValue::SubCommand(options) => options,
        _ => unreachable!("Subcommand is required"),
    };
    let options = parse_options(options);

    let channel = match options.get("channel") {
        Some(ResolvedValue::Channel(channel)) => *channel,
        _ => unreachable!("Channel is required"),
    };

    let reaction = match options.get("emoji") {
        Some(ResolvedValue::String(emoji)) => ReactionType::try_from(*emoji)?,
        _ => unreachable!("Emoji is required"),
    };

    let data = ctx.data.read().await;
    let pool = data
        .get::<PostgresPool>()
        .expect("PostgresPool should exist in data.");

    match command.name {
        "add" => {
            let role = match options.get("role") {
                Some(ResolvedValue::Role(role)) => *role,
                _ => unreachable!("Role is required"),
            };

            let message_id = match options.get("message_id") {
                Some(ResolvedValue::String(message_id)) => {
                    Some(MessageId::new(message_id.parse()?))
                }
                _ => None,
            };

            add(ctx, interaction, pool, channel, message_id, reaction, role).await?;
        }
        "remove" => {
            let message_id = match options.get("message_id") {
                Some(ResolvedValue::String(message_id)) => MessageId::new(message_id.parse()?),
                _ => unreachable!("Message ID is required"),
            };

            remove(
                ctx,
                interaction,
                pool,
                channel,
                guild_id,
                message_id,
                reaction,
            )
            .await?;
        }
        _ => unreachable!("Invalid subcommand name"),
    };

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    Command::create_global_command(
        ctx,
        CreateCommand::new("reaction_role")
            .description("Adds or removes a reaction role")
            .default_member_permissions(Permissions::MANAGE_MESSAGES)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "add",
                    "Adds a reaction role",
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
                )
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::String,
                    "message_id",
                    "The message id of the reaction role message",
                )),
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
            ),
    )
    .await?;

    Ok(())
}
