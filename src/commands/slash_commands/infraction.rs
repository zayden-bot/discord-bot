use crate::infraction_type::InfractionType;
use crate::sqlx_lib::{create_user_infraction, get_user_infractions};
use crate::utils::{message_response, send_message};
use chrono::{Duration, Months, Utc};
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CreateEmbed, Message,
};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateMessage};
use serenity::model::prelude::{GuildId, Member};
use serenity::model::{Permissions, Timestamp};
use serenity::prelude::Context;
use std::cmp;

async fn warn(
    ctx: &Context,
    member: Member,
    guild_id: &GuildId,
    moderator: Member,
    points: i64,
    reason: String,
) -> Result<String, String> {
    let partial_guild = guild_id.to_partial_guild(ctx).await.unwrap();

    let _ = member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been warned")
                    .description(format!(
                        "You have been warned in {} for the following reason: {}",
                        partial_guild.name, reason
                    )),
            ),
        )
        .await;

    let user_id = member.user.id.get() as i64;
    let username = member.user.name.as_str();
    let guild_id = guild_id.get() as i64;
    let infraction_type = InfractionType::Warn;
    let points = points as i32;
    let reason = reason.as_str();

    let result = create_user_infraction(
        user_id,
        username,
        guild_id,
        infraction_type,
        moderator.user,
        points,
        reason,
    )
    .await;

    if result.is_err() {
        return Err("Failed to create database infraction".to_string());
    }

    Ok("User has been warned".to_string())
}

async fn mute(
    ctx: &Context,
    mut member: Member,
    guild_id: &GuildId,
    moderator: Member,
    duration: Duration,
    points: i64,
    reason: String,
) -> Result<String, String> {
    let timestamp = (Utc::now() + duration).timestamp();

    let result = member
        .disable_communication_until_datetime(
            ctx,
            Timestamp::from_unix_timestamp(timestamp).unwrap(),
        )
        .await;

    if result.is_err() {
        return Err("Failed to mute user".to_string());
    }

    let _ = member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been muted")
                    .description(format!(
                        "You have been muted in {} for the following reason: {}",
                        guild_id.to_partial_guild(ctx).await.unwrap().name,
                        reason
                    )),
            ),
        )
        .await;

    let user_id = member.user.id.get() as i64;
    let username = member.user.name.as_str();
    let guild_id = guild_id.get() as i64;
    let infraction_type = InfractionType::Mute;
    let points = points as i32;
    let reason = reason.as_str();

    let result = create_user_infraction(
        user_id,
        username,
        guild_id,
        infraction_type,
        moderator.user,
        points,
        reason,
    )
    .await;

    if result.is_err() {
        return Err("Failed to create database infraction".to_string());
    }

    Ok("User has been muted".to_string())
}

async fn ban(
    ctx: &Context,
    member: Member,
    guild_id: &GuildId,
    moderator: Member,
    points: i64,
    reason: String,
) -> Result<String, String> {
    let result = member.ban_with_reason(ctx, 7, &reason).await;

    if result.is_err() {
        return Err("Failed to ban user".to_string());
    }

    let _ = member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been banned")
                    .description(format!(
                        "You have been banned from {} for the following reason: {}",
                        guild_id.to_partial_guild(ctx).await.unwrap().name,
                        reason
                    )),
            ),
        )
        .await;

    let user_id = member.user.id.get() as i64;
    let username = member.user.name.as_str();
    let guild_id = guild_id.get() as i64;
    let infraction_type = InfractionType::Ban;
    let points = points as i32;
    let reason = reason.as_str();

    let result = create_user_infraction(
        user_id,
        username,
        guild_id,
        infraction_type,
        moderator.user,
        points,
        reason,
    )
    .await;

    if result.is_err() {
        return Err("Failed to create database infraction".to_string());
    }

    Ok("User has been banned".to_string())
}

fn get_option_by_name(
    interaction: &CommandInteraction,
    name: &str,
) -> Option<CommandDataOptionValue> {
    match interaction
        .data
        .options
        .clone()
        .into_iter()
        .find(|option| option.name == name)
    {
        Some(option) => Some(option.value),
        None => None,
    }
}

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let author_id = interaction.user.id;

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

    let user = match interaction.data.options[0].value.as_user_id() {
        Some(user) => user,
        None => return message_response(&ctx, interaction, "Please provide a valid user").await,
    };

    let moderator = match guild_id.member(&ctx, &author_id).await {
        Ok(moderator) => moderator,
        Err(_) => return message_response(&ctx, interaction, "Invalid moderator").await,
    };

    let member = match guild_id.member(&ctx, user).await {
        Ok(member) => member,
        Err(_) => {
            return message_response(&ctx, interaction, "User not found in this server").await
        }
    };

    let points = match get_option_by_name(interaction, "points") {
        Some(CommandDataOptionValue::Integer(points)) => points,
        _ => 1,
    };

    let reason = match get_option_by_name(interaction, "reason") {
        Some(CommandDataOptionValue::String(reason)) => reason,
        _ => "No reason provided".to_string(),
    };

    let user_infractions = match get_user_infractions(member.user.id.get() as i64).await {
        Ok(user_infractions) => user_infractions,
        Err(_) => return message_response(&ctx, interaction, "Error getting user config").await,
    };

    let six_months_age = Utc::now()
        .checked_sub_months(Months::new(6))
        .unwrap()
        .naive_utc();

    let infractions = user_infractions
        .iter()
        .filter(|infraction| infraction.created_at >= six_months_age)
        .collect::<Vec<_>>();
    let infraction_count = infractions
        .into_iter()
        .map(|infraction| infraction.points)
        .sum::<i32>();
    let infraction_count = cmp::min((infraction_count as i64) + points, 5);

    let result = match infraction_count {
        1 => warn(&ctx, member, &guild_id, moderator, points, reason).await,
        2 => {
            mute(
                &ctx,
                member,
                &guild_id,
                moderator,
                Duration::hours(1),
                points,
                reason,
            )
            .await
        }
        3 => {
            mute(
                &ctx,
                member,
                &guild_id,
                moderator,
                Duration::hours(8),
                points,
                reason,
            )
            .await
        }
        4 => {
            mute(
                &ctx,
                member,
                &guild_id,
                moderator,
                Duration::days(28),
                points,
                reason,
            )
            .await
        }
        5 => ban(&ctx, member, &guild_id, moderator, points, reason).await,
        _ => {
            return message_response(&ctx, interaction, "Invalid amount of infraction points").await
        }
    };

    match result {
        Ok(message) => send_message(&ctx, interaction, message.as_str()).await,
        Err(message) => message_response(&ctx, interaction, message.as_str()).await,
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("infraction")
        .name("infraction")
        .description("Warn, mute, or ban a user")
        .default_member_permissions(Permissions::MODERATE_MEMBERS)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to warn, mute, or ban",
            )
            .required(true),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::Integer,
            "points",
            "The number of infractions to give the user",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "reason",
            "The reason for the infraction",
        ))
}
