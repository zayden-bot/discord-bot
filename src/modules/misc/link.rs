use async_trait::async_trait;
use patreon_api::types::Member;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Permissions, Ready, ResolvedOption, ResolvedValue,
};
use sqlx::PgPool;
use zayden_core::{parse_options, SlashCommand};

use crate::guilds::ServersTable;
use crate::modules::bunny;
use crate::modules::patreon::patreon_member;
use crate::sqlx_lib::PostgresPool;
use crate::utils::message_response;
use crate::{Error, Result};

pub struct Link;

#[async_trait]
impl SlashCommand<Error> for Link {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        let command = &interaction.data.options()[0];

        download(ctx, interaction, &pool, &command.value).await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("link")
            .description("Helper command for getting links")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "download",
                    "Direct download link",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "game",
                        "The game to get the download link for",
                    )
                    .add_string_choice("College Kings 1", "College_Kings")
                    .add_string_choice("College Kings 2", "College_Kings_2")
                    .required(true),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "platform",
                        "The platform to get the download link for",
                    )
                    .add_string_choice("Windows", "pc")
                    .add_string_choice("Mac", "mac")
                    .add_string_choice("Linux", "pc")
                    .required(true),
                ),
            );

        Ok(command)
    }
}

async fn download(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &PgPool,
    subcommand: &ResolvedValue<'_>,
) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

    let support_channel_id = ServersTable::get_row(pool, guild_id)
        .await
        .unwrap()
        .unwrap()
        .get_support_channel_id()
        .unwrap();

    if interaction
        .channel
        .as_ref()
        .ok_or_else(|| Error::MissingGuildId)?
        .parent_id
        .is_some_and(|id| id == support_channel_id)
    {
        interaction.defer(ctx).await.unwrap();
    } else {
        interaction.defer_ephemeral(ctx).await.unwrap();
    }

    let options = match subcommand {
        ResolvedValue::SubCommand(options) => parse_options(options),
        _ => unreachable!("Subcommand is required"),
    };

    let game = match options.get("game") {
        Some(ResolvedValue::String(game)) => *game,
        _ => unreachable!("Game option is required"),
    };

    if !interaction.member.as_ref().is_some_and(|member| {
        member
            .permissions
            .is_some_and(|perms| perms.contains(Permissions::MANAGE_MESSAGES))
    }) {
        let Member {
            campaign_lifetime_support_cents,
            currently_entitled_amount_cents,
            ..
        } = patreon_member(pool, &interaction.user.id.to_string(), false)
            .await?
            .ok_or_else(|| Error::PatreonAccountNotFound(interaction.user.id.to_string()))?
            .data
            .attributes;

        if game == "College_Kings_2"
            && currently_entitled_amount_cents < 1000
            && campaign_lifetime_support_cents < 2000
        {
            message_response(
                ctx,
                interaction,
                "To access College Kings 2, you need to be an active $10 (Junior) patron with a lifetime subscription of $20.\nUse `/patreon_user login` to manually update the cache and link your Discord account.",
            )
            .await.unwrap();

            return Ok(());
        }
    }

    let game_folder = game.to_lowercase();

    let platform = match options.get("platform") {
        Some(ResolvedValue::String(platform)) => *platform,
        _ => unreachable!("Platform option is required"),
    };

    let link = bunny::latest_download_link(&game_folder, platform).await?;

    message_response(ctx, interaction, link).await.unwrap();

    Ok(())
}
