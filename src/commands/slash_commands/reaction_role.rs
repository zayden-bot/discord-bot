use serenity::builder::CreateApplicationCommand;
use serenity::model::id::ChannelId;
use serenity::model::Permissions;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::{GuildId, ReactionType};
use serenity::prelude::Context;
use crate::sqlx_lib::{create_reaction_role, delete_reaction_role};
use crate::utils::respond_with_message;

async fn add(ctx: &Context, interaction: &ApplicationCommandInteraction, subcommand: &CommandDataOption, guild_id: GuildId, channel: ChannelId, message_id: &i64, emoji: &str) -> Result<(), serenity::Error> {
    let role = match subcommand.options[3].resolved.as_ref() {
        Some(CommandDataOptionValue::Role(role)) => role,
        _ => return respond_with_message(ctx, interaction, "Please provide a valid role").await,
    };

    let message = match channel.message(ctx, *message_id as u64).await {
        Ok(message) => message,
        Err(_) => return respond_with_message(ctx, interaction, "Please provide a valid message id").await,
    };

    match create_reaction_role(guild_id.0 as i64, channel.0 as i64, message_id, role.id.0 as i64, emoji).await {
        Ok(_) => {},
        Err(_) => return respond_with_message(ctx, interaction, "Error adding reaction role").await,
    }

    message.react(ctx, ReactionType::Unicode(emoji.to_string())).await?;
    respond_with_message(ctx, interaction, "Reaction role added").await
}

async fn remove(ctx: &Context, interaction: &ApplicationCommandInteraction, guild_id: GuildId, channel: ChannelId, message_id: &i64, emoji: &str) -> Result<(), serenity::Error> {
    let message = match channel.message(ctx, *message_id as u64).await {
        Ok(message) => message,
        Err(_) => return respond_with_message(ctx, interaction, "Please provide a valid message id").await,
    };

    match delete_reaction_role(guild_id.0 as i64, channel.0 as i64, message_id, emoji).await {
        Ok(_) => {},
        Err(_) => return respond_with_message(ctx, interaction, "Error deleting reaction role").await,
    }

    match message.delete_reaction_emoji(ctx, ReactionType::Unicode(emoji.to_string())).await {
        Ok(_) => respond_with_message(ctx, interaction, "Reaction Role removed").await,
        Err(_) => respond_with_message(ctx, interaction, "Error deleting reaction").await,
    }
}

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let subcommand = &interaction.data.options[0];

    let channel = match subcommand.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::Channel(channel)) => channel.id,
        _ => return respond_with_message(ctx, interaction, "Please provide a valid channel").await,
    };

    let message_id = match subcommand.options[1].resolved.as_ref() {
        Some(CommandDataOptionValue::Integer(message_id)) => message_id,
        _ => return respond_with_message(ctx, interaction, "Please provide a valid message id").await,
    };

    let emoji = match subcommand.options[2].resolved.as_ref() {
        Some(CommandDataOptionValue::String(emoji)) => emoji,
        _ => return respond_with_message(ctx, interaction, "Please provide a valid emoji").await,
    };

    return match subcommand.name.as_str() {
        "add" => add(ctx, interaction, subcommand, guild_id, channel, message_id, emoji).await,
        "remove" => remove(ctx, interaction, guild_id, channel, message_id, emoji).await,
        _ => respond_with_message(ctx, interaction, "Invalid subcommand").await,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("reaction_role")
        .description("Adds or removes a reaction role")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .create_option(|option| {
            option.name("add")
                .description("Adds a reaction role")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option| {
                    sub_option.name("channel")
                        .description("The channel the message is in")
                        .kind(CommandOptionType::Channel)
                        .required(true)
                })
                .create_sub_option(|sub_option| {
                    sub_option.name("message_id")
                        .description("The message id of the reaction role message")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                })
                .create_sub_option(|sub_option| {
                    sub_option.name("emoji")
                        .description("The emoji of the reaction role")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|sub_option| {
                    sub_option.name("role")
                        .description("The role to add when the emoji is reacted to")
                        .kind(CommandOptionType::Role)
                        .required(true)
                })
        })
        .create_option(|option| {
            option.name("remove")
                .description("Removes a reaction role")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option| {
                    sub_option.name("channel")
                        .description("The channel the message is in")
                        .kind(CommandOptionType::Channel)
                        .required(true)
                })
                .create_sub_option(|sub_option| {
                    sub_option.name("message_id")
                        .description("The message id of the reaction role message")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                })
                .create_sub_option(|sub_option| {
                    sub_option.name("emoji")
                        .description("The emoji of the reaction role")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
}