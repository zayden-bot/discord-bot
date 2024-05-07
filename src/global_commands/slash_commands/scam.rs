use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, DiscordJsonError, ErrorResponse, Permissions, ResolvedValue,
};
use serenity::http::HttpError::UnsuccessfulRequest;

use crate::utils::{embed_response, parse_options};
use crate::{Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(&ctx).await?;

    let guild_id = interaction.guild_id.ok_or_else(|| Error::NotInGuild)?;

    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("member") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => unreachable!("Expected user"),
    };
    let reason = match options.get("reason") {
        Some(ResolvedValue::String(reason)) => reason,
        _ => "Compromised account: Sending scam links.",
    };

    let member = match guild_id.member(&ctx, user).await {
        Ok(member) => member,
        Err(_) => {
            embed_response(
                ctx,
                interaction,
                CreateEmbed::new()
                    .title("Error")
                    .description("Member not found."),
            )
            .await?;
            return Ok(());
        }
    };

    let guild = guild_id.to_partial_guild(&ctx).await?;

    match user
        .create_dm_channel(&ctx)
        .await?
        .send_message(
            &ctx,
            CreateMessage::new().add_embed(CreateEmbed::new().description(format!(
                "You have been soft banned from {} for the following reason: {}",
                guild.name, reason
            ))),
        )
        .await
    {
        // 50007: Cannot send messages to this user
        Err(serenity::Error::Http(UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 50007, .. },
            ..
        }))) => {}
        result => {
            result?;
        }
    }

    guild_id.ban_with_reason(&ctx, user, 1, reason).await?;
    guild_id.unban(&ctx, user).await?;

    embed_response(
        ctx,
        interaction,
        CreateEmbed::new().title("Soft Banned").description(format!(
            "{} has been successfully soft banned for the following reason: {}",
            member.user.name, reason
        )),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("scam")
        .description("Soft ban a compromised account")
        .default_member_permissions(Permissions::KICK_MEMBERS)
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "member", "Member to soft ban")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "reason", "Reason for soft ban")
                .required(false),
        )
}
