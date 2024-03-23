use crate::infraction_type::InfractionType;
use crate::sqlx_lib::{create_user_infraction, get_user_infractions};
use crate::utils::{message_response, parse_options};
use chrono::{Duration, Months, TimeDelta, Utc};
use serenity::all::{CommandInteraction, CommandOptionType, CreateEmbed, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateMessage};
use serenity::model::prelude::{GuildId, Member};
use serenity::model::{Permissions, Timestamp};
use serenity::prelude::Context;
use std::cmp;

use crate::{Error, Result};

async fn warn<'a>(
    ctx: &Context,
    member: Member,
    guild_id: &GuildId,
    moderator: Member,
    points: i64,
    reason: &str,
) -> Result<&'a str> {
    member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been warned")
                    .description(format!(
                        "You have been warned in {} for the following reason: {}",
                        guild_id.to_partial_guild(ctx).await?.name,
                        reason
                    )),
            ),
        )
        .await?;

    let user_id = member.user.id.get() as i64;
    let username = member.user.name.as_str();
    let guild_id = guild_id.get() as i64;
    let infraction_type = InfractionType::Warn;
    let points = points as i32;

    create_user_infraction(
        user_id,
        username,
        guild_id,
        infraction_type,
        moderator.user,
        points,
        reason,
    )
    .await?;

    Ok("User has been warned")
}

async fn mute<'a>(
    ctx: &Context,
    mut member: Member,
    guild_id: &GuildId,
    moderator: Member,
    duration: Duration,
    points: i64,
    reason: &str,
) -> Result<&'a str> {
    let timestamp = (Utc::now() + duration).timestamp();

    member
        .disable_communication_until_datetime(ctx, Timestamp::from_unix_timestamp(timestamp)?)
        .await?;

    member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been muted")
                    .description(format!(
                        "You have been muted in {} for the following reason: {}",
                        guild_id.to_partial_guild(ctx).await?.name,
                        reason
                    )),
            ),
        )
        .await?;

    let user_id = member.user.id.get() as i64;
    let username = member.user.name.as_str();
    let guild_id = guild_id.get() as i64;
    let infraction_type = InfractionType::Mute;
    let points = points as i32;

    create_user_infraction(
        user_id,
        username,
        guild_id,
        infraction_type,
        moderator.user,
        points,
        reason,
    )
    .await?;

    Ok("User has been muted")
}

async fn ban<'a>(
    ctx: &Context,
    member: Member,
    guild_id: &GuildId,
    moderator: Member,
    points: i64,
    reason: &str,
) -> Result<&'a str> {
    member.ban_with_reason(ctx, 7, &reason).await?;

    member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been banned")
                    .description(format!(
                        "You have been banned from {} for the following reason: {}",
                        guild_id.to_partial_guild(ctx).await?.name,
                        reason
                    )),
            ),
        )
        .await?;

    let user_id = member.user.id.get() as i64;
    let username = member.user.name.as_str();
    let guild_id = guild_id.get() as i64;
    let infraction_type = InfractionType::Ban;
    let points = points as i32;

    create_user_infraction(
        user_id,
        username,
        guild_id,
        infraction_type,
        moderator.user,
        points,
        reason,
    )
    .await?;

    Ok("User has been banned")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => unreachable!("User option is required"),
    };

    let moderator = guild_id.member(&ctx, interaction.user.id).await?;

    let points = match options.get("points") {
        Some(ResolvedValue::Integer(points)) => *points,
        _ => 1,
    };

    let reason = match options.get("reason") {
        Some(ResolvedValue::String(reason)) => *reason,
        _ => "No reason provided",
    };

    let user_infractions = get_user_infractions(user.id.get() as i64, false).await?;

    let six_months_age = Utc::now()
        .checked_sub_months(Months::new(6))
        .ok_or_else(|| Error::TimeDelta)?
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

    let member = guild_id.member(&ctx, user).await?;

    let message = match infraction_count {
        n if n <= 1 => warn(ctx, member, &guild_id, moderator, points, reason).await?,
        2 => {
            mute(
                ctx,
                member,
                &guild_id,
                moderator,
                TimeDelta::try_hours(1).ok_or_else(|| Error::TimeDelta)?,
                points,
                reason,
            )
            .await?
        }
        3 => {
            mute(
                ctx,
                member,
                &guild_id,
                moderator,
                TimeDelta::try_hours(8).ok_or_else(|| Error::TimeDelta)?,
                points,
                reason,
            )
            .await?
        }
        4 => {
            mute(
                ctx,
                member,
                &guild_id,
                moderator,
                TimeDelta::try_days(28).ok_or_else(|| Error::TimeDelta)?,
                points,
                reason,
            )
            .await?
        }
        n if n >= 5 => ban(ctx, member, &guild_id, moderator, points, reason).await?,
        _ => unreachable!("Invalid infraction count"),
    };

    message_response(ctx, interaction, message).await?;

    Ok(())
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
