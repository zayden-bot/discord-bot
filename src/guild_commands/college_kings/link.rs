use crate::{
    utils::{message_response, parse_options},
    SERVER_URL,
};
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
    interaction.defer(ctx).await?;

    let subcommand = match subcommand {
        ResolvedValue::SubCommand(subcommand) => subcommand,
        _ => unreachable!("Subcommand is required"),
    };

    let options = parse_options(subcommand);

    let game = match options.get("game") {
        Some(ResolvedValue::String(game)) => game,
        _ => unreachable!("Game option is required"),
    };

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
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
}
