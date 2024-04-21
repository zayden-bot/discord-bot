use crate::infraction_type::InfractionType;
use crate::models::Infraction;
use crate::sqlx_lib::{create_user_infraction, get_pool, get_user_infractions};
use crate::utils::{embed_response, parse_options};
use chrono::{Months, TimeDelta, Utc};
use serenity::all::{CommandInteraction, CommandOptionType, CreateEmbed, ResolvedValue};
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
) -> Result<CreateEmbed> {
    member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been warned")
                    .description(format!(
                        "You have been warned in {} for the following reason:\n{}",
                        guild_id.to_partial_guild(ctx).await?.name,
                        reason
                    )),
            ),
        )
        .await?;

    let username = member.user.name;

    create_user_infraction(
        pool,
        Infraction::new(
            member.user.id.get(),
            &username,
            guild_id.get(),
            InfractionType::Ban,
            moderator,
            points,
            reason,
        )?,
    )
    .await?;

    let mut embed = CreateEmbed::new().title(format!("{} has been warned", username));
    if reason != "No reason provided." {
        embed = embed.description(reason);
    }
    Ok(embed)
}

async fn mute<'a>(
    ctx: &Context,
    pool: &Pool<Postgres>,
    mut member: Member,
    moderator: Member,
    duration: TimeDelta,
    points: i32,
    reason: &str,
) -> Result<CreateEmbed> {
    let guild_id = member.guild_id;
    let timestamp = (Utc::now() + duration).timestamp();

    member
        .disable_communication_until_datetime(ctx, Timestamp::from_unix_timestamp(timestamp)?)
        .await?;

    let days = duration.num_days();
    let hours = duration.num_hours() % 24;

    let mut duration_str = String::new();
    if days > 0 {
        duration_str.push_str(&format!("{} day", days));
        if days > 1 {
            duration_str.push('s');
        }
    } else if hours > 0 {
        duration_str.push_str(&format!("{} hour", hours));
        if hours > 1 {
            duration_str.push('s');
        }
    }

    let desc: String = if reason == "No reason provided." {
        format!(
            "You have been muted in {} for {}.",
            guild_id.to_partial_guild(ctx).await?.name,
            duration_str
        )
    } else {
        format!(
            "You have been muted in {} for {}\n{}",
            guild_id.to_partial_guild(ctx).await?.name,
            duration_str,
            reason
        )
    };

    member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been muted")
                    .description(desc),
            ),
        )
        .await?;

    let username = member.user.name;

    create_user_infraction(
        pool,
        Infraction::new(
            member.user.id.get(),
            &username,
            guild_id.get(),
            InfractionType::Ban,
            moderator,
            points,
            reason,
        )?,
    )
    .await?;

    let mut embed = CreateEmbed::new().title(format!("{} has been muted", username));
    if reason != "No reason provided." {
        embed = embed.description(reason);
    }
    Ok(embed)
}

async fn ban<'a>(
    ctx: &Context,
    pool: &Pool<Postgres>,
    member: Member,
    guild_id: &GuildId,
    moderator: Member,
    points: i32,
    reason: &str,
) -> Result<CreateEmbed> {
    member.ban_with_reason(ctx, 1, &reason).await?;

    member
        .user
        .dm(
            ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("You have been banned")
                    .description(format!(
                        "You have been banned in {} for the following reason:\n{}",
                        guild_id.to_partial_guild(ctx).await?.name,
                        reason
                    )),
            ),
        )
        .await?;

    let username = member.user.name;

    create_user_infraction(
        pool,
        Infraction::new(
            member.user.id.get(),
            &username,
            guild_id.get(),
            InfractionType::Ban,
            moderator,
            points,
            reason,
        )?,
    )
    .await?;

    let mut embed = CreateEmbed::new().title(format!("{} has been banned", username));
    if reason != "No reason provided." {
        embed = embed.description(reason);
    }
    Ok(embed)
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
        _ => "No reason provided.",
    };

    let pool = get_pool(ctx).await?;

    let user_infractions = get_user_infractions(&pool, user.id.get(), false).await?;

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

    let embed = match infraction_count {
        1 => warn(ctx, &pool, member, &guild_id, moderator, points, reason).await?,
        2 => {
            mute(
                ctx,
                &pool,
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
                &pool,
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
                &pool,
                member,
                moderator,
                TimeDelta::try_days(28).ok_or_else(|| Error::TimeDelta)?,
                points,
                reason,
            )
            .await?
        }
        5 => ban(ctx, &pool, member, &guild_id, moderator, points, reason).await?,
        _ => unreachable!("Invalid infraction count"),
    };

    embed_response(ctx, interaction, embed).await?;

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
