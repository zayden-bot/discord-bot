use crate::infraction_type::InfractionType;
use crate::models::Infraction;
use crate::sqlx_lib::{create_user_infraction, get_user_infractions, PostgresPool};
use crate::utils::{message_response, parse_options};
use chrono::{Duration, Months, TimeDelta, Utc};
use serenity::all::{Command, CommandInteraction, CommandOptionType, CreateEmbed, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateMessage};
use serenity::model::prelude::{GuildId, Member};
use serenity::model::{Permissions, Timestamp};
use serenity::prelude::Context;
use sqlx::{Pool, Postgres};
use std::cmp;

use crate::{Error, Result};

async fn warn<'a>(
    ctx: &Context,
    pool: &Pool<Postgres>,
    member: Member,
    guild_id: &GuildId,
    moderator: Member,
    points: i32,
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

    create_user_infraction(
        pool,
        Infraction::new(
            member.user.id.get(),
            &member.user.name,
            guild_id.get(),
            InfractionType::Ban,
            moderator,
            points,
            reason,
        )?,
    )
    .await?;

    Ok("User has been warned")
}

async fn mute<'a>(
    ctx: &Context,
    pool: &Pool<Postgres>,
    mut member: Member,
    moderator: Member,
    duration: Duration,
    points: i32,
    reason: &str,
) -> Result<&'a str> {
    let guild_id = member.guild_id;
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

    create_user_infraction(
        pool,
        Infraction::new(
            member.user.id.get(),
            &member.user.name,
            guild_id.get(),
            InfractionType::Ban,
            moderator,
            points,
            reason,
        )?,
    )
    .await?;

    Ok("User has been muted")
}

async fn ban<'a>(
    ctx: &Context,
    pool: &Pool<Postgres>,
    member: Member,
    guild_id: &GuildId,
    moderator: Member,
    points: i32,
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

    create_user_infraction(
        pool,
        Infraction::new(
            member.user.id.get(),
            &member.user.name,
            guild_id.get(),
            InfractionType::Ban,
            moderator,
            points,
            reason,
        )?,
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

    let moderator = guild_id.member(ctx, interaction.user.id).await?;

    let points = match options.get("points") {
        Some(ResolvedValue::Integer(points)) => *points as i32,
        _ => 1,
    };

    let reason = match options.get("reason") {
        Some(ResolvedValue::String(reason)) => *reason,
        _ => "No reason provided",
    };

    let data = ctx.data.read().await;
    let pool = data
        .get::<PostgresPool>()
        .expect("PostgresPool should exist in data.");

    let user_infractions = get_user_infractions(pool, user.id.get(), false).await?;

    let six_months_age = Utc::now()
        .checked_sub_months(Months::new(6))
        .ok_or_else(|| Error::TimeDelta)?
        .naive_utc();

    let infractions: Vec<_> = user_infractions
        .iter()
        .filter(|infraction| infraction.created_at >= six_months_age)
        .collect();
    let infraction_count: i32 = infractions
        .into_iter()
        .map(|infraction| infraction.points)
        .sum();
    let infraction_count = cmp::min(infraction_count + points, 5);

    let member = guild_id.member(&ctx, user).await?;

    let message = match infraction_count {
        1 => warn(ctx, pool, member, &guild_id, moderator, points, reason).await?,
        2 => {
            mute(
                ctx,
                pool,
                member,
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
                pool,
                member,
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
                pool,
                member,
                moderator,
                TimeDelta::try_days(28).ok_or_else(|| Error::TimeDelta)?,
                points,
                reason,
            )
            .await?
        }
        5 => ban(ctx, pool, member, &guild_id, moderator, points, reason).await?,
        _ => unreachable!("Invalid infraction count"),
    };

    message_response(ctx, interaction, message).await?;

    Ok(())
}

pub async fn register(ctx: &Context) -> Result<()> {
    Command::create_global_command(
        ctx,
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
            )),
    )
    .await?;

    Ok(())
}
