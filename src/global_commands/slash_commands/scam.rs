use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateMessage, DiscordJsonError, EditInteractionResponse, ErrorResponse,
    HttpError, Permissions, Ready, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{parse_options, SlashCommand};

use crate::{Error, Result};

pub struct Scam;

#[async_trait]
impl SlashCommand<Error, Postgres> for Scam {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(&ctx).await.unwrap();

        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let mut options = parse_options(options);

        let Some(ResolvedValue::User(user, _)) = options.remove("member") else {
            unreachable!("Expected user");
        };

        let reason = match options.remove("reason") {
            Some(ResolvedValue::String(reason)) => reason,
            _ => "Compromised account: Sending scam links.",
        };

        let guild = guild_id.to_partial_guild(&ctx).await.unwrap();

        match user
            .create_dm_channel(&ctx)
            .await
            .unwrap()
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
            Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 50007, .. },
                ..
            }))) => {}
            result => {
                result.unwrap();
            }
        }

        guild_id
            .ban_with_reason(&ctx, user, 1, reason)
            .await
            .unwrap();
        guild_id.unban(&ctx, user).await.unwrap();

        let embed = CreateEmbed::new().title("Soft Banned").description(format!(
            "{} has been successfully soft banned for the following reason: {}",
            user.display_name(),
            reason
        ));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("scam")
            .description("Soft ban a compromised account")
            .default_member_permissions(Permissions::KICK_MEMBERS)
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "member", "Member to soft ban")
                    .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "reason",
                    "Reason for soft ban",
                )
                .required(false),
            );

        Ok(command)
    }
}
