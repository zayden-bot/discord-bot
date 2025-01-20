use std::collections::HashMap;

use async_trait::async_trait;
use patreon_api::types::Member;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    EditInteractionResponse, Permissions, Ready, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{parse_options, SlashCommand};

use crate::guilds::ServersTable;
use crate::modules::bunny;
use crate::modules::patreon::patreon_member;
use crate::{Error, Result};

pub struct Link;

#[async_trait]
impl SlashCommand<Error, Postgres> for Link {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        let ResolvedValue::SubCommand(options) = options.remove(0).value else {
            unreachable!("Subcommand is required");
        };

        let options = parse_options(options);

        download(ctx, interaction, pool, options).await?;

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
    mut options: HashMap<&str, ResolvedValue<'_>>,
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
        .ok_or(Error::MissingGuildId)?
        .parent_id
        .is_some_and(|id| id == support_channel_id)
    {
        interaction.defer(ctx).await.unwrap();
    } else {
        interaction.defer_ephemeral(ctx).await.unwrap();
    }

    let Some(ResolvedValue::String(game)) = options.remove("game") else {
        unreachable!("Game option is required");
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
            return Err(Error::PatreonTierTooLow);
        }
    }

    let game_folder = game.to_lowercase();

    let platform = match options.get("platform") {
        Some(ResolvedValue::String(platform)) => *platform,
        _ => unreachable!("Platform option is required"),
    };

    let link = bunny::latest_download_link(&game_folder, platform).await?;

    interaction
        .edit_response(ctx, EditInteractionResponse::new().content(link))
        .await
        .unwrap();

    Ok(())
}
