use crate::utils::{respond_with_embed, respond_with_message};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, Permissions,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
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

    let user_id = match interaction.data.options[0].value.as_user_id() {
        Some(user) => user,
        None => {
            return respond_with_message(ctx, interaction, "Cannot get member: Unknown Member")
                .await
        }
    };

    let reason = interaction
        .data
        .options
        .get(1)
        .and_then(|option| option.value.as_str())
        .unwrap_or("Compromised account: Sending scam links.");

    let member = match guild_id.member(&ctx, &user_id).await {
        Ok(member) => member,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting member").await,
    };

    let partial_guild = match guild_id.to_partial_guild(&ctx).await {
        Ok(partial_guild) => partial_guild,
        Err(_) => return respond_with_message(ctx, interaction, "Error getting guild").await,
    };

    let _ = user_id
        .create_dm_channel(&ctx)
        .await
        .unwrap()
        .send_message(
            &ctx,
            CreateMessage::new().add_embed(CreateEmbed::new().description(format!(
                "You have been soft banned from {} for the following reason: {}",
                partial_guild.name, reason
            ))),
        )
        .await;

    if guild_id
        .ban_with_reason(&ctx, user_id, 1, reason)
        .await
        .is_err()
    {
        return respond_with_message(ctx, interaction, "Error banning user").await;
    };

    if (guild_id.unban(&ctx, user_id).await).is_err() {
        return respond_with_message(ctx, interaction, "Error unbanning user").await;
    }

    respond_with_embed(
        ctx,
        interaction,
        CreateEmbed::new().title("Soft Banned").description(format!(
            "{} has been successfully soft banned for the following reason: {}",
            member.user.name, reason
        )),
    )
    .await
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
