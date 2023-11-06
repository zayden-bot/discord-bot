use crate::utils::{respond_with_embed, respond_with_message};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Permissions;
use serenity::prelude::Context;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return respond_with_message(
                ctx,
                interaction,
                "This command can only be used in a server",
            )
            .await
        }
    };

    let user = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::User(user, _member)) => user,
        _ => {
            return respond_with_message(ctx, interaction, "Cannot get member: Unknown Member")
                .await
        }
    };

    let reason = match interaction.data.options.get(1) {
        Some(reason) => match reason.resolved.as_ref() {
            Some(CommandDataOptionValue::String(reason)) => reason,
            _ => return respond_with_message(ctx, interaction, "Invalid reason").await,
        },
        _ => "Compromised account: Sending scam links.",
    };

    let member = match guild_id.member(&ctx, &user.id).await {
        Ok(member) => member,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting member").await,
    };

    let partial_guild = match guild_id.to_partial_guild(&ctx).await {
        Ok(partial_guild) => partial_guild,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting guild").await,
    };

    let _ = user
        .dm(&ctx, |message| {
            message.embed(|e| {
                e.description(format!(
                    "You have been soft banned from {} for the following reason: {}",
                    partial_guild.name, reason
                ))
            })
        })
        .await;

    if guild_id
        .ban_with_reason(&ctx, user.id, 1, reason)
        .await
        .is_err()
    {
        return respond_with_message(ctx, interaction, "Error banning user").await;
    };

    if (guild_id.unban(&ctx, user.id).await).is_err() {
        return respond_with_message(ctx, interaction, "Error unbanning user").await;
    }

    respond_with_embed(ctx, interaction, |e| {
        e.title("Soft Banned").description(format!(
            "{} has been successfully soft banned for the following reason: {}",
            member.user.name, reason
        ))
    })
    .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("scam")
        .description("Soft ban a compromised account")
        .default_member_permissions(Permissions::KICK_MEMBERS)
        .create_option(|option| {
            option
                .name("member")
                .description("Member to soft ban")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("reason")
                .description("Reason for soft ban")
                .kind(CommandOptionType::String)
        })
}
