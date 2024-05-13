use crate::{
    guilds::college_kings::SUPPORT_CHANNEL_ID,
    patreon_lib,
    utils::{message_response, parse_options},
    Error, SERVER_URL,
};
use reqwest::Client;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Permissions, ResolvedValue,
};

use crate::Result;

async fn download(
    ctx: &Context,
    interaction: &CommandInteraction,
    subcommand: &ResolvedValue<'_>,
) -> Result<()> {
    if interaction
        .channel
        .as_ref()
        .ok_or_else(|| Error::NotInGuild)?
        .parent_id
        .is_some_and(|id| id == SUPPORT_CHANNEL_ID)
    {
        interaction.defer(ctx).await?;
    } else {
        interaction.defer_ephemeral(ctx).await?;
    }

    let subcommand = match subcommand {
        ResolvedValue::SubCommand(subcommand) => subcommand,
        _ => unreachable!("Subcommand is required"),
    };

    let options = parse_options(subcommand);

    let game = match options.get("game") {
        Some(ResolvedValue::String(game)) => *game,
        _ => unreachable!("Game option is required"),
    };

    if !interaction.member.as_ref().is_some_and(|member| {
        member
            .permissions
            .is_some_and(|perms| perms.contains(Permissions::MANAGE_MESSAGES))
    }) {
        let user = patreon_lib::get_user(&Client::new(), interaction.user.id, false).await?;

        if game == "College_Kings_2"
            && user.tier.amount_cents < 1000
            && user.lifetime_support_cents < 2000
        {
            message_response(
                ctx,
                interaction,
                "To access College Kings 2, you need to be an active $10 (Junior) patron with a lifetime subscription of $20.\nUse `/patreon login` to manually update the cache and link your Discord account.",
            )
            .await?;
            return Ok(());
        }
    }

    let game_folder = game.to_lowercase();

    let platform = match options.get("platform") {
        Some(ResolvedValue::String(platform)) => platform,
        _ => unreachable!("Platform option is required"),
    };

    let response = reqwest::get(format!(
        "{}/api/v1/bunny/latest/{}/{}",
        SERVER_URL, game_folder, platform
    ))
    .await?;

    let link = response.text().await?;
    message_response(ctx, interaction, link).await?;

    Ok(())
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let command = &interaction.data.options()[0];

    download(ctx, interaction, &command.value).await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("link")
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
        )
}
