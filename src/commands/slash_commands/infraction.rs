use std::cmp;
use chrono::{Duration, Months, Utc};
use serenity::builder::CreateApplicationCommand;
use serenity::model::{Permissions, Timestamp};
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::{Guild, Member};
use serenity::prelude::Context;
use crate::infraction_type::InfractionType;
use crate::sqlx_lib::{create_user_infraction, get_user_infractions};
use crate::utils::respond_with_message;

async fn warn(ctx: &Context, member: Member, guild: &Guild, moderator: &Member, points: i64, reason: String) -> Result<String, String> {
    let _ = member.user.dm(ctx, |message| {
        message.embed(|embed| {
            embed.title("You have been warned");
            embed.description(format!("You have been warned in {} for the following reason: {}", guild.name, reason));
            embed
        })
    }).await;

    let user_id = member.user.id.0 as i64;
    let username = member.user.name.as_str();
    let guild_id = guild.id.0 as i64;
    let infraction_type = InfractionType::Warn;
    let moderator_id = moderator.user.id.0 as i64;
    let moderator_name = moderator.user.name.as_str();
    let points = points as i32;
    let reason = reason.as_str();

    let result = create_user_infraction(user_id, username, guild_id, infraction_type, moderator_id, moderator_name, points, reason).await;

    if result.is_err() {
        return Err("Failed to create database infraction".to_string());
    }

    Ok("User has been warned".to_string())
}

async fn mute(ctx: &Context, mut member: Member, guild: &Guild, moderator: &Member, duration: Duration, points: i64, reason: String) -> Result<String, String> {
    let timestamp = (Utc::now() + duration).timestamp();

    let result= member.disable_communication_until_datetime(ctx, Timestamp::from_unix_timestamp(timestamp).unwrap()).await;

    if result.is_err() {
        return Err("Failed to mute user".to_string());
    }

    let _ = member.user.dm(ctx, |message| {
        message.embed(|embed| {
            embed.title("You have been muted");
            embed.description(format!("You have been muted in {} for the following reason: {}", guild.name, reason));
            embed
        })
    }).await;

    let user_id = member.user.id.0 as i64;
    let username = member.user.name.as_str();
    let guild_id = guild.id.0 as i64;
    let infraction_type = InfractionType::Mute;
    let moderator_id = moderator.user.id.0 as i64;
    let moderator_name = moderator.user.name.as_str();
    let points = points as i32;
    let reason = reason.as_str();

    let result = create_user_infraction(user_id, username, guild_id, infraction_type, moderator_id, moderator_name, points, reason).await;

    if result.is_err() {
        return Err("Failed to create database infraction".to_string());
    }

    Ok("User has been muted".to_string())
}

async fn ban(ctx: &Context, member: Member, guild: &Guild, moderator: &Member, points: i64, reason: String) -> Result<String, String> {
    let result = member.ban_with_reason(ctx, 7, &reason).await;

    if let Err(_) = result {
        return Err("Failed to ban user".to_string());
    }

    let _ = member.user.dm(ctx, |message| {
        message.embed(|embed| {
            embed.title("You have been banned");
            embed.description(format!("You have been banned from {} for the following reason: {}", guild.name, reason));
            embed
        })
    }).await;

    let user_id = member.user.id.0 as i64;
    let username = member.user.name.as_str();
    let guild_id = guild.id.0 as i64;
    let infraction_type = InfractionType::Ban;
    let moderator_id = moderator.user.id.0 as i64;
    let moderator_name = moderator.user.name.as_str();
    let points = points as i32;
    let reason = reason.as_str();

    let result = create_user_infraction(user_id, username, guild_id, infraction_type, moderator_id, moderator_name, points, reason).await;

    if result.is_err() {
        return Err("Failed to create database infraction".to_string());
    }

    Ok("User has been banned".to_string())
}

fn get_option_by_name(interaction: &ApplicationCommandInteraction, name: &str) -> Option<CommandDataOptionValue> {
    match interaction.data.options.clone().into_iter().find(|option| option.name == name) {
        Some(option) => option.resolved,
        None => None,
    }
}

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let author_id = interaction.user.id;

    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let guild = match guild_id.to_guild_cached(&ctx) {
        Some(guild) => guild,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let user = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::User(user, _member)) => user,
        _ => return respond_with_message(ctx, interaction, "Please provide a valid user").await,
    };

    let moderator = match guild.members.get(&author_id) {
        Some(moderator) => moderator,
        None => return respond_with_message(ctx, interaction, "Invalid moderator").await,
    };

    let member = match guild.members.get(&user.id) {
        Some(member) => member.to_owned(),
        None => return respond_with_message(ctx, interaction, "Please provide a valid user").await,
    };

    let points = match get_option_by_name(interaction, "points") {
        Some(CommandDataOptionValue::Integer(points)) => points,
        _ => 1,
    };

    let reason = match get_option_by_name(interaction, "reason") {
        Some(CommandDataOptionValue::String(reason)) => reason,
        _ => "No reason provided".to_string(),
    };

    let user_infractions = match get_user_infractions(member.user.id.0 as i64).await {
        Ok(user_infractions) => user_infractions,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting user config").await,
    };

    let six_months_age = Utc::now().checked_sub_months(Months::new(6)).unwrap().naive_utc();

    let infractions = user_infractions.iter().filter(|infraction| infraction.created_at >= six_months_age).collect::<Vec<_>>();
    let infraction_count = infractions.into_iter().map(|infraction| infraction.points).sum::<i32>();
    let infraction_count = cmp::min((infraction_count as i64) + points, 5);

    let result = match infraction_count {
        1 => warn(ctx, member, &guild, moderator, points, reason).await,
        2 => mute(ctx, member, &guild, moderator, Duration::hours(1), points,reason).await,
        3 => mute(ctx, member, &guild, moderator, Duration::hours(8), points, reason).await,
        4 => mute(ctx, member, &guild, moderator, Duration::days(28), points, reason).await,
        5 => ban(ctx, member, &guild, moderator, points, reason).await,
        _ => return respond_with_message(ctx, interaction, "Invalid amount of infraction points").await,
    };

    match result {
        Ok(message) => respond_with_message(ctx, interaction, message.as_str()).await,
        Err(message) => respond_with_message(ctx, interaction, message.as_str()).await,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("infraction")
        .description("Warn, mute, or ban a user")
        .default_member_permissions(Permissions::MODERATE_MEMBERS)
        .create_option(|option| {
            option.name("user")
                .description("The user to warn, mute, or ban")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option.name("points")
                .description("The number of infractions to give the user")
                .kind(CommandOptionType::Integer)
        })
        .create_option(|option| {
            option.name("reason")
                .description("The reason for the infraction")
                .kind(CommandOptionType::String)
        })
}